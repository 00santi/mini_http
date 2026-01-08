use serde::Deserialize;

pub struct AppRequest {
    pub method: Method,
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

pub struct AppResponse {
    pub code: StatusCode,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

pub enum StatusCode {
    OK,
    NOTFOUND,
}


pub fn router(req: AppRequest) -> AppResponse {
    match req.method {
        Method::GET => {
            route_get(req)
        },
        Method::POST => {
            route_post(req)
        },
        _ => handle_404(),
    }
}

pub fn route_get(req: AppRequest) -> AppResponse {
    match req.path.as_str() {
        "/" | "/health" => health_ok(),
        "/time" => time(req),
        _ => handle_404(),
    }
}

pub fn route_post(req: AppRequest) -> AppResponse {
    match req.path.as_str() {
        "/sum" => sum(req),
        _ => handle_404(),
    }
}

fn health_ok() -> AppResponse {
    AppResponse {
        code: StatusCode::OK,
        headers: vec![],
        body: Some(String::from("ok!")),
    }
}

fn handle_404() -> AppResponse {
    AppResponse {
        code: StatusCode::NOTFOUND,
        headers: vec![],
        body: Some(String::from("404!")),
    }
}

fn time(req: AppRequest) -> AppResponse {
    AppResponse {
        code: StatusCode::OK,
        headers: vec![],
        body: Some(format!("{:?}", std::time::SystemTime::now()))
    }
}

#[derive(Deserialize)]
struct SumInput {
    a: i32,
    b: i32,
}

fn sum(req: AppRequest) -> AppResponse {
    let Some(json) = req.body else {
        return handle_404()
    };

    let Ok(parsed): Result<SumInput, _> = serde_json::from_str(&json) else {
        return handle_404()
    };

    AppResponse {
        code: StatusCode::OK,
        headers: vec![],
        body: Some(format!("Sum = {}", parsed.a + parsed.b)),
    }
}