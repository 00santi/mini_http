use std::convert::Infallible;
use hyper::{Body, Method, Request, Response};
use hyper::header::{HeaderName, HeaderValue};
use crate::app;

pub fn get_try_app(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let Some(req) = get_req_to_app(req) else {
        return Ok(Response::new(Body::from("404!")));
    };

    let res = app::router(req);

    Ok(app_to_req(res))
}

pub async fn try_app(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let Some(req) = req_to_app(req).await else {
        return Ok(Response::new(Body::from("404!")));
    };

    let res = app::router(req);

    Ok(app_to_req(res))
}

fn get_req_to_app(req: Request<Body>) -> Option<app::AppRequest> {
    let mut headers = Vec::new();
    for (name, value) in req.headers() {
        let value_str = match value.to_str() {
            Ok(v) => v.to_string(),
            Err(_) => return None,
        };
        headers.push((name.to_string(), value_str));
    }

    Some(app::AppRequest {
        method: app::Method::GET,
        path: req.uri().path().to_string(),
        headers,
        body: None,
    })
}

async fn req_to_app(req: Request<Body>) -> Option<app::AppRequest> {
    use hyper::body::to_bytes;

    let method = match req.method() {
        &Method::POST => app::Method::POST,
        &Method::DELETE => app::Method::DELETE,
        &Method::PUT => app::Method::PUT,
        _ => return None,
    };

    let path = req.uri().path().to_string();

    let mut headers = Vec::new();
    for (name, value) in req.headers() {
        let value_str = match value.to_str() {
            Ok(v) => v.to_string(),
            Err(_) => return None,
        };
        headers.push((name.to_string(), value_str));
    }

    let body = match method {
        app::Method::GET => return None,
        _ => match to_bytes(req.into_body()).await {
            Err(_) => return None,
            Ok(bytes) => match String::from_utf8(bytes.to_vec()) {
                Ok(s) => Some(s),
                Err(_) => return None,
            }
        }
    };

    Some(app::AppRequest {
        method, path, headers, body
    })
}

fn app_to_req(mut res: app::AppResponse) -> Response<Body> {
    let code = match res.code {
        app::StatusCode::OK => 200,
        app::StatusCode::NOTFOUND => 404,
    };

    let mut builder = Response::builder().status(code);

    for (header, val) in res.headers.drain(..) {
        if let (Ok(header), Ok(val)) = (HeaderName::from_bytes(header.as_bytes()), HeaderValue::from_str(&val)) {
            builder = builder.header(header, val);
        }
    }

    let body = Body::from(res.body.unwrap_or_default());
    builder.body(body).unwrap()
}