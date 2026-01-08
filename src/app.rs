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
    match req.path.as_str() {
        "/time" => time(req),
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

fn sum(req: AppRequest) -> AppResponse {
    todo!();
}