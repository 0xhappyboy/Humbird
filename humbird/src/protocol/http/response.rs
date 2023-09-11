use std::{collections::HashMap, fs, path::Path};

use crate::config::config::ROOT_PATH;

use super::{method::Method, request::Request};

// generic response wrapper
#[derive(Debug, Clone)]
pub struct Response {
    pub head: Vec<(String, String)>,
    pub body: Vec<u8>,
    req_method: Method,
    req_path: String,
    raw: String,
}

impl Response {
    pub fn new(request: &Request) -> Response {
        let mut response = Response {
            req_method: request.method.clone(),
            req_path: request.path.clone(),
            head: vec![],
            body: vec![],
            raw: String::from(""),
        };
        response.handle_response();
        response
    }
    pub fn hand_map(&self) -> HashMap<String, String> {
        let map: HashMap<String, String> = self.head.clone().into_iter().collect();
        map
    }
    fn handle_response(&mut self) {
        match self.req_method {
            Method::GET(_) => {
                self.handle_get_response();
            }
            Method::POST(_) => {
                self.handle_post_response();
            }
            Method::HEAD(_) => {
                //TODO
            }
            Method::PUT(_) => {
                //TODO
            }
            Method::DELETE(_) => {
                //TODO
            }
            Method::CONNECT(_) => {
                //TODO
            }
            Method::OPTIONS(_) => {
                //TODO
            }
            Method::TRACE(_) => {
                //TODO
            }
            Method::DEFAULT(_) => {
                //TODO
            }
        }
    }
    /// handle get method response
    fn handle_get_response(&mut self) {
        let resource = format!("{}{}", unsafe { ROOT_PATH }, self.req_path);
        let mut res: String = String::default();
        match fs::read_to_string(Path::new(&resource)) {
            Ok(c) => {
                res = format!(
                    "HTTP/1.1 200 OK \r\nContent-Length:{} \r\n\r\n{}\r\n",
                    c.len(),
                    c
                );
            }
            Err(_) => {
                let c = String::from("page does not exist");
                res = format!(
                    "HTTP/1.1 404 OK \r\nContent-Length:{} \r\n\r\n{}\r\n",
                    c.len(),
                    c
                );
            }
        }
        self.body = res.as_bytes().to_vec();
    }
    /// handle post method response
    fn handle_post_response(&mut self) {
        let c = format!("response test");
        let res = format!(
            "HTTP/1.1 200 OK \r\nContent-Length:{} \r\n\r\n{}\r\n",
            c.len(),
            c
        );
        self.body = res.as_bytes().to_vec()
    }
}
