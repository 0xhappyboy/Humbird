use std::{collections::HashMap, default};

use regex::Regex;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, BufReader},
    net::tcp::OwnedReadHalf,
};
use tracing::instrument;

use super::{http::Delimiter, method::Method};

// generic request wrapper
#[derive(Debug, Clone)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub protocol: String,
    pub cookie: HashMap<String, String>,
    pub head: HashMap<String, String>,
    pub body: Vec<u8>,
    pub raw: String,
}

impl Request {
    #[instrument]
    pub async fn new() -> Result<(), String> {
        Ok(())
    }
    #[instrument]
    pub async fn read(r: OwnedReadHalf) -> Result<Self, String> {
        let mut protocol_line = String::default();
        let mut r_buf: BufReader<OwnedReadHalf> = BufReader::new(r);
        let _ = r_buf.read_line(&mut protocol_line).await;
        if !Request::is(protocol_line.to_string()) {
            return Err("http request processing failed".to_string());
        }
        let items: Vec<&str> = protocol_line.split(" ").collect();
        let mut req_str_buf = String::new();
        let mut delimiter = Delimiter::HEAD;
        let mut req = Request {
            method: Method::new(items[0]),
            path: items[1].to_string(),
            protocol: items[2].to_string().replace("\r\n", ""),
            cookie: HashMap::default(),
            head: HashMap::default(),
            body: Vec::new(),
            raw: String::from(protocol_line.clone()),
        };
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
                            req.raw.push_str(&c);
                            if c.eq("\r\n") {
                                delimiter = Delimiter::BODY;
                                continue;
                            };
                            // push request head
                            req.push_head(c);
                        }
                        Err(_) => {
                            // error
                            break;
                        }
                    }
                }
                Delimiter::BODY => {
                    match req.method {
                        Method::POST => {
                            let mut buf = vec![
                                0u8;
                                req.head
                                    .get("Content-Length")
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
                                    req.body = buf;
                                    break;
                                }
                                Err(_) => {
                                    // TODO
                                    break;
                                }
                            }
                        }
                        Method::GET => {
                            break;
                        }
                        Method::HEAD => {
                            //TODO
                        }
                        Method::PUT => {
                            //TODO
                        }
                        Method::DELETE => {
                            //TODO
                        }
                        Method::CONNECT => {
                            //TODO
                        }
                        Method::OPTIONS => {
                            //TODO
                        }
                        Method::TRACE => {
                            //TODO
                        }
                        Method::DEFAULT => {
                            // TODO
                        }
                    }
                }
            }
        }
        Ok(req)
    }
    pub fn push_head(&mut self, item: String) {
        let item_split: Vec<&str> = item.split(":").collect();
        if item_split.len() == 0 {
            return;
        }
        let k = item_split[0].trim().to_string();
        let v = item_split[1];
        self.head.insert(
            k.to_owned(),
            v.trim()
                .to_string()
                .chars()
                .into_iter()
                .filter(|c| !c.eq(&'\r') && !c.eq(&'\n'))
                .collect(),
        );
        // cookies
        if k.clone().eq("Cookie") {
            let cookies: Vec<&str> = v.split(";").collect();
            let _ = cookies.iter().map(|&e| {
                let cookie_split: Vec<&str> = e.split("=").collect();
                if cookie_split.len() > 0 {
                    self.cookie
                        .insert(cookie_split[0].to_owned(), cookie_split[1].to_owned());
                }
            });
        }
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
        &self.raw
    }
    /// determine whether it is an http request
    pub fn is(r: String) -> bool {
        let re = Regex::new(r"^(GET|HEAD|POST|PUT|DELETE|CONNECT|OPTIONS|TRACE)\s(([/0-9a-zA-Z.]+)?(\?[0-9a-zA-Z&=]+)?)\s(HTTP/1.0|HTTP/1.1|HTTP/2.0)\r\n$").unwrap();
        re.is_match(&r)
    }
}
