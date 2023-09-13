use std::{collections::HashMap, fs, hash::Hash, path::Path};

use regex::Regex;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, BufReader},
    net::tcp::OwnedReadHalf,
};
use tracing::instrument;

use crate::config::config::ROOT_PATH;

use super::{http::Delimiter, method::Method, request::Request};

// generic response wrapper
#[derive(Debug, Clone)]
pub struct Response {
    pub protocol: String,
    pub status_code: String,
    pub status_msg: String,
    pub head: HashMap<String, String>,
    pub body: Vec<u8>,
    pub content_length: u64,
    req_method: Method,
    req_path: String,
    raw: String,
}

impl Response {
    #[instrument]
    pub fn new(request: &Request) -> Self {
        let mut response = Response {
            protocol: String::default(),
            status_code: String::default(),
            status_msg: String::default(),
            head: HashMap::default(),
            body: vec![],
            raw: String::default(),
            req_method: Method::DEFAULT,
            req_path: String::default(),
            content_length: 0,
        };
        response.handle_response();
        response
    }
    #[instrument]
    pub async fn read(mut r: OwnedReadHalf) -> Result<Self, String> {
        let mut protocol_line = String::default();
        let mut r_buf: BufReader<OwnedReadHalf> = BufReader::new(r);
        let _ = r_buf.read_line(&mut protocol_line).await;
        if !Response::is(protocol_line.to_string()) {
            return Err("this is not an http response body".to_string());
        }
        let items: Vec<&str> = protocol_line.split(" ").collect();
        let mut response_str_buf = String::default();
        let mut delimiter = Delimiter::HEAD;
        let mut response = Response {
            protocol: items[0].to_string(),
            status_code: items[1].to_string(),
            status_msg: match Some(items[2]) {
                Some(s) => s.to_string().replace("\r\n", ""),
                None => "".to_string(),
            },
            head: HashMap::default(),
            body: vec![],
            raw: String::default(),
            req_method: Method::DEFAULT,
            req_path: String::default(),
            content_length: 0,
        };

        loop {
            match delimiter {
                Delimiter::HEAD => {
                    match r_buf.read_line(&mut response_str_buf).await {
                        Ok(0) => {
                            break;
                        }
                        Ok(_n) => {
                            let c = response_str_buf.drain(..).as_str().to_string();
                            response.raw.push_str(&c);
                            if c.eq("\r\n") {
                                delimiter = Delimiter::BODY;
                                continue;
                            };
                            // push request head
                            response.push_head(c);
                        }
                        Err(_) => {
                            break;
                        }
                    }
                }
                Delimiter::BODY => {
                    let mut buf = vec![
                        0u8;
                        response
                            .head
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
                            // save response body
                            response.body = buf;
                            break;
                        }
                        Err(_) => {
                            // TODO
                            break;
                        }
                    }
                }
            }
        }
        Ok((response))
    }
    fn handle_response(&mut self) {
        match self.req_method {
            Method::GET => {
                self.handle_get_response();
            }
            Method::POST => {
                self.handle_post_response();
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
    pub fn push_head(&mut self, item: String) {
        let item_split: Vec<&str> = item.split(":").collect();
        if item_split.len() == 0 {
            return;
        }
        let k = item_split[0].trim().to_string();
        let v = item_split[1].trim().to_string();
        if k.eq("Content-Length") {
            self.content_length = match v.parse::<u64>() {
                Ok(length) => length,
                Err(_) => 0,
            };
        }
        self.head.insert(
            k,
            v.trim()
                .to_string()
                .chars()
                .into_iter()
                .filter(|c| !c.eq(&'\r') && !c.eq(&'\n'))
                .collect(),
        );
    }
    /// determine whether it is an http response
    fn is(r: String) -> bool {
        let re = Regex::new(
            r"^(HTTP/1.0|HTTP/1.1|HTTP/2.0)\s(200|400|401|403|404|500|503)\s(([/0-9a-zA-Z.]+)?(\?[0-9a-zA-Z&=]+)?)\r\n$",
        )
        .unwrap();
        re.is_match(&r)
    }
}
