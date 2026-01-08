pub struct AppRequest {
    method: Method,
    path: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
}

pub struct AppResponse {
    code: StatusCode,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

pub enum StatusCode {
    OK,
    NOTFOUND,
}
