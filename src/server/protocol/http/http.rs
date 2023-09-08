use std::{collections::HashMap, fs, io::Error, path::Path};

use serde::de;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    join,
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
};

use crate::config::config::ROOT_PATH;
use super::mime::*;

// delimiter
#[derive(Debug)]
pub enum Delimiter {
    HEAD,
    BODY,
}

// http protocol method encapsulation
#[derive(Debug)]
pub enum Method {
    GET(String),
    POST(String),
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

// generic request wrapper
#[derive(Debug, PartialEq, Eq)]
pub struct Request {
    pub cookie: Vec<(String, String)>,
    pub header: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn new() -> Request {
        Request {
            cookie: vec![],
            header: vec![],
            body: vec![],
        }
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
    pub fn hander_map(&self) -> HashMap<String, String> {
        let map: HashMap<String, String> = self.header.clone().into_iter().collect();
        map
    }
}

// generic response wrapper
#[derive(Debug)]
pub struct Response {
    pub header: Vec<(String, String)>,
}

impl Response {
    pub fn new() -> Response {
        Response { header: vec![] }
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
    pub fn hander_map(&self) -> HashMap<String, String> {
        let map: HashMap<String, String> = self.header.clone().into_iter().collect();
        map
    }
}

// overall encapsulation of http protocol packets
#[derive(Debug)]
pub struct Http {
    pub method: Method,
    pub path: String,
    pub protocol: String,
    pub w: OwnedWriteHalf,
    pub request: Request,
    pub response: Response,
}

impl Http {
    pub async fn new(c: String, mut r_buf: BufReader<OwnedReadHalf>, w: OwnedWriteHalf) -> Http {
        let items: Vec<&str> = c.split(" ").collect();
        let mut http = Http {
            w: w,
            method: Method::new(items[0]),
            path: items[1].to_string(),
            protocol: items[2].to_string().replace("\r\n", ""),
            request: Request::new(),
            response: Response::new(),
        };
        let mut req_str_buf = String::new();
        let mut delimiter = Delimiter::HEAD;
        loop {
            match delimiter {
                Delimiter::HEAD => {
                    // handle head
                    match r_buf.read_line(&mut req_str_buf).await {
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
                            http.request.push_header(c);
                        }
                        Err(_) => {
                            // error
                            break;
                        }
                    }
                }
                Delimiter::BODY => {
                    match http.method {
                        Method::POST(_) => {
                            let mut buf = vec![
                                0u8;
                                http.get_head_info("Content-Length")
                                    .unwrap()
                                    .parse::<u64>()
                                    .unwrap()
                                    .try_into()
                                    .unwrap()
                            ];
                            match r_buf.read(&mut buf).await {
                                Ok(0) => {
                                    // TODO
                                    break;
                                }
                                Ok(_s) => {
                                    // TODO
                                    // save request body
                                    http.request.body = buf;
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
                    }
                }
            }
        }
        http
    }

    // handle request body
    fn handle_request_body(&mut self) {}

    // response
    pub async fn response(&mut self) {
        match self.method {
            Method::GET(_) => {
                join!(self.handle_get_response());
            }
            Method::POST(_) => {
                join!(self.handle_post_response());
            }
        }
    }

    // handle get method response
    async fn handle_get_response(&mut self) {
        let resource = format!("{}{}", unsafe { ROOT_PATH }, self.path);
        match fs::read_to_string(Path::new(&resource)) {
            Ok(c) => {
                let res = format!(
                    "HTTP/1.1 200 OK \r\nContent-Length:{} \r\n\r\n{}\r\n",
                    c.len(),
                    c
                );
                let _ = self.w.write_all(res.as_bytes()).await;
            }
            Err(_) => {
                let c = String::from("page does not exist");
                let res = format!(
                    "HTTP/1.1 404 OK \r\nContent-Length:{} \r\n\r\n{}\r\n",
                    c.len(),
                    c
                );
                let _ = self.w.write_all(res.as_bytes()).await;
            }
        }
    }
    // handle post method response
    async fn handle_post_response(&mut self) {
        let c = format!("response test");
        let res = format!(
            "HTTP/1.1 200 OK \r\nContent-Length:{} \r\n\r\n{}\r\n",
            c.len(),
            c
        );
        let _ = self.w.write_all(res.as_bytes()).await;
    }

    // get head info
    fn get_head_info(&self, k: &str) -> Option<String> {
        let h_map = self.request.hander_map();
        if h_map.is_empty() {
            return None;
        }
        if !h_map.contains_key(k) {
            return None;
        }
        let v = self.request.hander_map().get(k).unwrap().to_string();
        Some(v)
    }
}
