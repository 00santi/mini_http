use std::net::SocketAddr;
use hyper::{Body, Method, Request, Response, Server};
use hyper::service::{ make_service_fn, service_fn };
use std::convert::Infallible;
mod translator;
mod app;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Start of async server");
    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));
    println!("Opened a socket.  Ip: {}.  Port: {}", addr.ip(), addr.port());

    let make_service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(router)) });

    let server = Server::bind(&addr).serve(make_service);
    println!("Server up on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}

async fn router(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match req.method() {
        &Method::GET => get::handle_get(req),
        &Method::POST => post::handle_post(req).await,
        _ => handle_404(req)
    }
}

fn handle_404(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("404!")))
}

fn health_ok(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("ok!")))
}



mod get {
    use std::convert::Infallible;
    use hyper::{Body, Request, Response};
    use super::translator;

    pub fn handle_get(req: Request<Body>) -> Result<Response<Body>, Infallible> {
        match req.uri().path() {
            "/" | "/health" => super::health_ok(req),
            "/echo" => get_echo(req),
            _ => translator::get_try_app(req),
        }
    }

    fn get_echo(req: Request<Body>) -> Result<Response<Body>, Infallible> {
        let mut result = String::new();

        let method = req.method().as_str();
        let path = req.uri().path();

        result.push_str(method);
        result.push('\n');
        result.push_str(path);
        result.push('\n');

        for (name, value) in req.headers() {
            result.push_str(name.as_str());
            result.push_str(" || ");
            result.push_str(value.to_str().unwrap_or("! invalid value"));
            result.push('\n');
        }

        Ok(Response::new(Body::from(result)))
    }
}



mod post {
    use std::convert::Infallible;
    use hyper::{Body, Request, Response};
    use super::translator;

    pub async fn handle_post(req: Request<Body>) -> Result<Response<Body>, Infallible> {
        match req.uri().path() {
            "/echo_body" => echo_body(req).await,
            _ => translator::try_app(req).await,
        }
    }

    async fn echo_body(req: Request<Body>) -> Result<Response<Body>, Infallible> {
        let result = req.into_body();
        Ok(Response::new(result))
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use hyper::{Request, Body};

    #[tokio::test]
    async fn sum() {
        let req = Request::builder()
            .method("POST")
            .uri("/sum")
            .body(Body::from(r#"{"a":10,"b":20}"#))
            .unwrap();

        let res = router(req).await.unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(body, "Sum = 30");
    }

    #[tokio::test]
    async fn health() {
        let req = Request::builder()
            .method("GET")
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let res = router(req).await.unwrap();
        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(body, "ok!");
    }
}
