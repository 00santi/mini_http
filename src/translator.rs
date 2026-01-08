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