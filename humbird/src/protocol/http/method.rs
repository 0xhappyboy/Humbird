// http protocol method encapsulation
#[derive(Debug, Clone)]
pub enum Method {
    DEFAULT(String),
    GET(String),
    POST(String),
    HEAD(String),
    PUT(String),
    DELETE(String),
    CONNECT(String),
    OPTIONS(String),
    TRACE(String),
}

impl Method {
    pub fn new(m: &str) -> Self {
        match m {
            "GET" => Method::GET(m.to_string()),
            "POST" => Method::POST(m.to_string()),
            _ => {
                todo!()
            }
        }
    }
}
