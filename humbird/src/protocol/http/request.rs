use std::collections::HashMap;

use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, BufReader},
    net::tcp::OwnedReadHalf,
};

use super::method::Method;

// delimiter
#[derive(Debug)]
pub enum Delimiter {
    HEAD,
    BODY,
}

// generic request wrapper
#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub protocol: String,
    pub cookie: Vec<(String, String)>,
    pub header: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Request {
    pub async fn new(request_line: String, mut r: BufReader<OwnedReadHalf>) -> Result<Request, ()> {
        let items: Vec<&str> = request_line.split(" ").collect();
        let mut req_str_buf = String::new();
        let mut delimiter = Delimiter::HEAD;
        let mut req = Request {
            method: Method::new(items[0]),
            path: items[1].to_string(),
            protocol: items[2].to_string().replace("\r\n", ""),
            cookie: Vec::new(),
            header: Vec::new(),
            body: Vec::new(),
        };
        loop {
            match delimiter {
                Delimiter::HEAD => {
                    // handle head
                    match r.read_line(&mut req_str_buf).await {
                        Ok(0) => {
                            // end
                            break;
                        }
                        Ok(_n) => {
                            let c = req_str_buf.drain(..).as_str().to_string();
                            if c.eq("\r\n") {
                                delimiter = Delimiter::BODY;
                                continue;
                            };
                            // push request header
                            req.push_header(c);
                        }
                        Err(_) => {
                            // error
                            break;
                        }
                    }
                }
                Delimiter::BODY => {
                    match req.method {
                        Method::POST(_) => {
                            let mut buf = vec![
                                0u8;
                                req.get_head_info("Content-Length")
                                    .unwrap()
                                    .parse::<u64>()
                                    .unwrap()
                                    .try_into()
                                    .unwrap()
                            ];
                            match r.read(&mut buf).await {
                                Ok(0) => {
                                    // TODO
                                    break;
                                }
                                Ok(_s) => {
                                    // TODO
                                    // save request body
                                    req.body = buf;
                                    break;
                                }
                                Err(_) => {
                                    // TODO
                                    break;
                                }
                            }
                        }
                        Method::GET(_) => {
                            break;
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
                            // TODO
                        }
                    }
                }
            }
        }
        Ok(req)
    }
    pub fn push_header(&mut self, item: String) {
        let item_split: Vec<&str> = item.split(":").collect();
        self.header.push((
            item_split[0].trim().to_string(),
            item_split[1]
                .trim()
                .to_string()
                .chars()
                .into_iter()
                .filter(|c| !c.eq(&'\r') && !c.eq(&'\n'))
                .collect(),
        ));
    }
    /// get all request header information in the form of map
    pub fn hander_map(&self) -> HashMap<String, String> {
        let map: HashMap<String, String> = self.header.clone().into_iter().collect();
        map
    }
    /// get head info
    pub fn get_head_info(&self, k: &str) -> Option<String> {
        let h_map = self.hander_map();
        if h_map.is_empty() {
            return None;
        }
        if !h_map.contains_key(k) {
            return None;
        }
        let v = self.hander_map().get(k).unwrap().to_string();
        Some(v)
    }
    /// convert request body structure to http protocol request structure string
    ///
    /// Example
    /// ```
    /// GET / HTTP/1.1\r\n
    /// request head1\r\n
    /// request head1\r\n
    /// \r\n
    /// request body
    /// ```
    pub fn to_string(&self) -> &str {
        ""
    }
}
