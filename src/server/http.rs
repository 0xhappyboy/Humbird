use std::{fs, path::Path};

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    join,
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
};

use crate::config::config::ROOT_PATH;

/// http protocol method encapsulation
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

/// encapsulation of header information structure
#[derive(Debug)]
pub struct Header {
    pub k: String,
    pub v: String,
}

/// generic request wrapper
#[derive(Debug)]
pub struct Request {
    pub header: Vec<Header>,
}

impl Request {
    pub fn push_header(&mut self, item: String) {
        let item_split: Vec<&str> = item.split(":").collect();
        self.header.push(Header {
            k: item_split[0].to_string(),
            v: item_split[1].to_string(),
        });
    }
    pub fn to_string(&mut self) -> String {
        self.header.iter();
        String::from("")
    }
}

/// generic response wrapper
#[derive(Debug)]
pub struct Response {
    pub header: Vec<Header>,
}

impl Response {
    pub fn push_header(&mut self, item: String) {
        let item_split: Vec<&str> = item.split(":").collect();
        self.header.push(Header {
            k: item_split[0].to_string(),
            v: item_split[1].to_string(),
        });
    }
    pub fn to_string(&mut self) -> String {
        self.header.iter();
        String::from("")
    }
}

/// overall encapsulation of http protocol packets
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
    pub async fn New(c: String, mut r_buf: BufReader<OwnedReadHalf>, w: OwnedWriteHalf) -> Http {
        let items: Vec<&str> = c.split(" ").collect();
        let mut http = Http {
            w: w,
            method: Method::new(items[0]),
            path: items[1].to_string(),
            protocol: items[2].to_string().replace("\r\n", ""),
            request: Request { header: vec![] },
            response: Response { header: vec![] },
        };
        let mut req_str_buf = String::new();
        loop {
            match r_buf.read_line(&mut req_str_buf).await {
                Ok(0) => {
                    break;
                }
                Ok(_n) => {
                    let c = req_str_buf.drain(..).as_str().to_string();
                    if c.eq("\r\n") {
                        break;
                    };
                    // push request header
                    http.request.push_header(c);
                }
                Err(_) => {
                    break;
                }
            }
        }
        http
    }

    /// response
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
    async fn handle_post_response(&mut self) {}
}
