// http protocol method encapsulation
#[derive(Debug, Clone)]
pub enum Method {
    DEFAULT,
    GET,
    POST,
    HEAD,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
}

impl Method {
    pub fn new(m: &str) -> Self {
        match m {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => Method::DEFAULT,
        }
    }
}
