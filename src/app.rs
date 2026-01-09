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



#[cfg(test)]
mod tests {
    use super::*;

    fn req_helper(method: Method, path: &str, body: Option<&str>) -> AppRequest {
        AppRequest {
            method,
            path: path.to_string(),
            headers: vec![],
            body: body.map(|s| s.to_string()),
        }
    }

    #[test]
    fn health() {
        let req = req_helper(Method::GET, "/health", None);
        let res = router(req);

        assert!(matches!(res.code, StatusCode::OK));
        assert_eq!(res.body.unwrap_or_default(), "ok!");
    }

    #[test]
    fn sum() {
        let req = req_helper(
            Method::POST,
            "/sum",
            Some(r#"{"a":5,"b":7}"#),
        );

        let res = router(req);

        assert!(matches!(res.code, StatusCode::OK));
        assert_eq!(res.body.unwrap_or_default(), "Sum = 12");
    }

    #[test]
    fn sum_fail() {
        let req = req_helper(Method::POST, "/sum", Some("invalid json {}"));
        let res = router(req);

        assert_eq!(res.body.unwrap_or_default(), "404!");
        assert!(matches!(res.code, StatusCode::NOTFOUND));
    }

    #[test]
    fn invalid_route() {
        let req = req_helper(Method::GET, "/invalid", None);
        let res = router(req);

        assert_eq!(res.body.unwrap_or_default(), "404!");
        assert!(matches!(res.code, StatusCode::NOTFOUND));
    }
}
