use std::convert::Infallible;
use hyper::{Body, Method, Request, Response};
use hyper::header::{HeaderName, HeaderValue};
use crate::app;

enum TranslatorError {
    InvalidMethod,
    InvalidHeaders,
    InvalidBody,
}

pub fn get_try_app(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let Ok(req) = get_req_to_app(req) else {
        return Ok(Response::new(Body::from("404!")));
    };

    let res = app::router(req);

    Ok(app_to_res(Ok(res)))
}

pub async fn try_app(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let Ok(req) = req_to_app(req).await else {
        return Ok(Response::new(Body::from("404!")));
    };

    let res = app::router(req);

    Ok(app_to_res(Ok(res)))
}

fn get_req_to_app(req: Request<Body>) -> Result<app::AppRequest, TranslatorError> {
    let mut headers = Vec::new();
    for (name, value) in req.headers() {
        let value_str = match value.to_str() {
            Ok(v) => v.to_string(),
            Err(_) => return Err(TranslatorError::InvalidHeaders),
        };
        headers.push((name.to_string(), value_str));
    }

    Ok(app::AppRequest {
        method: app::Method::GET,
        path: req.uri().path().to_string(),
        headers,
        body: None,
    })
}

async fn req_to_app(req: Request<Body>) -> Result<app::AppRequest, TranslatorError> {
    use hyper::body::to_bytes;

    let method = match req.method() {
        &Method::POST => app::Method::POST,
        &Method::DELETE => app::Method::DELETE,
        &Method::PUT => app::Method::PUT,
        _ => return Err(TranslatorError::InvalidMethod),
    };

    let path = req.uri().path().to_string();

    let mut headers = Vec::new();
    for (name, value) in req.headers() {
        let value_str = match value.to_str() {
            Ok(v) => v.to_string(),
            Err(_) => return Err(TranslatorError::InvalidHeaders),
        };
        headers.push((name.to_string(), value_str));
    }

    let body = match method {
        app::Method::GET => return Err(TranslatorError::InvalidMethod),
        _ => match to_bytes(req.into_body()).await {
            Err(_) => return Err(TranslatorError::InvalidBody),
            Ok(bytes) => match String::from_utf8(bytes.to_vec()) {
                Ok(s) => Some(s),
                Err(_) => return Err(TranslatorError::InvalidBody),
            }
        }
    };

    Ok(app::AppRequest {
        method, path, headers, body
    })
}

fn app_to_res(res: Result<app::AppResponse, TranslatorError>) -> Response<Body> {
    let Ok(mut res) = res else {
        return Response::new(Body::from("404!"));
    };

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



#[cfg(test)]
mod tests {
    use super::*;
    use hyper::{Request, Body};

    #[tokio::test]
    async fn sum() {
        let req = Request::builder()
            .method("POST")
            .uri("/sum")
            .header("content-type", "application/json")
            .body(Body::from(r#"{"a":3,"b":4}"#))
            .unwrap();

        let res = try_app(req).await.unwrap();

        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(body, "Sum = 7");
    }

    #[tokio::test]
    async fn health() {
        let req = Request::builder()
            .method("GET")
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let res = get_try_app(req).unwrap();

        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(body, "ok!");
    }

    #[tokio::test]
    async fn invalid_route() {
        let req = Request::builder()
            .method("GET")
            .uri("/invalid")
            .body(Body::empty())
            .unwrap();

        let res = get_try_app(req).unwrap();

        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(body, "404!");
    }

    #[tokio::test]
    async fn invalid_json() {
        let req = Request::builder()
            .method("POST")
            .uri("/sum")
            .body(Body::from("whatever"))
            .unwrap();

        let res = try_app(req).await.unwrap();

        let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
        let body = String::from_utf8(bytes.to_vec()).unwrap();

        assert_eq!(body, "404!");
    }
}
