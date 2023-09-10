use std::{collections::HashMap, fs, path::Path};

use tokio::{io::AsyncWriteExt, net::tcp::OwnedWriteHalf};

use crate::config::config::ROOT_PATH;

use super::{
    method::Method,
    request::{self, Request},
};

// generic response wrapper
#[derive(Debug)]
pub struct Response {
    pub header: Vec<(String, String)>,
    pub body: Vec<u8>,
    req_method: Method,
    req_path: String,
}

impl Response {
    pub async fn new(request: &Request) -> Response {
        let mut response = Response {
            req_method: request.method.clone(),
            req_path: request.path.clone(),
            header: vec![],
            body: vec![],
        };
        response.handle_response();
        response
    }
    pub fn hander_map(&self) -> HashMap<String, String> {
        let map: HashMap<String, String> = self.header.clone().into_iter().collect();
        map
    }
    /// convert response body structure to http protocol response structure string
    ///
    /// Example
    /// ```
    /// HTTP/1.1 200 OK \r\n
    /// response head
    /// \r\n\r\n
    /// response body
    /// ```
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
