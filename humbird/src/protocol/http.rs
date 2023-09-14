use std::{
    collections::HashMap,
    fs,
    io::{Read, Write},
    path::Path,
};

use mio::event::Event;
use regex::Regex;
use tokio::{io::AsyncWriteExt, net::tcp::OwnedReadHalf};
use tracing::{error, instrument};

use crate::{
    core::server::{NetModel, ROOT_PATH},
    plugins::web::ROUTER_TABLE,
};

/// http request process
pub type HttpRequestProcess = fn(Request, Response) -> Response;

// delimiter
#[derive(Debug)]
pub enum Delimiter {
    HEAD,
    BODY,
}

// overall encapsulation of http protocol packets
#[derive(Debug)]
pub struct Http {
    pub request: Request,
    pub response: Response,
    pub multi_thread_write: Option<tokio::net::tcp::OwnedWriteHalf>,
    pub event_poll_write: Option<mio::net::TcpStream>,
    pub event: Option<Event>,
    pub net_model: NetModel,
}

impl Http {
    pub async fn new_multi_thread(stream: tokio::net::TcpStream) -> Result<Http, String> {
        return Http::handle_multi_thread(stream).await;
    }
    pub async fn new_event_poll(
        event: &Event,
        stream: mio::net::TcpStream,
    ) -> Result<Http, String> {
        return Http::handle_event_poll(event, stream).await;
    }
    // handle multi thread model http
    async fn handle_multi_thread(stream: tokio::net::TcpStream) -> Result<Http, String> {
        let (r, w) = stream.into_split();
        match Request::multi_thread_read(r).await {
            Ok(request) => {
                let response = Response::new(&request);
                let mut http = Http {
                    request: request,
                    response: response,
                    multi_thread_write: Some(w),
                    event_poll_write: None,
                    event: None,
                    net_model: NetModel::Multithread,
                };
                // exec plugin
                http.exec_plugin();
                return Ok(http);
            }
            Err(_e) => {
                error!("http request processing failed");
                return Err("http request processing failed".to_string());
            }
        }
    }
    // handle event poll model http
    async fn handle_event_poll(event: &Event, stream: mio::net::TcpStream) -> Result<Http, String> {
        if event.is_readable() {
            match Request::event_poll_read(stream).await {
                Ok(request) => {
                    let response = Response::new(&request);
                    let mut http = Http {
                        request: request,
                        response: response,
                        event_poll_write: None,
                        multi_thread_write: None,
                        event: Some(event.clone()),
                        net_model: NetModel::EventPoll,
                    };
                    // exec plugin
                    http.exec_plugin();
                    return Ok(http);
                }
                Err(_e) => {
                    error!("http request processing failed");
                    return Err("http request processing failed".to_string());
                }
            }
        } else {
            return Err("the event is not readable".to_string());
        }
    }
    // multi thread response
    pub async fn multi_thread_response(mut self) {
        match self.multi_thread_write {
            Some(mut w) => {
                w.write_all(&self.response.body[..]).await;
            }
            None => todo!(),
        }
    }
    // event poll response
    pub async fn event_poll_response(mut self) {
        match self.event {
            Some(e) => {
                if e.is_writable() {
                    match self.event_poll_write {
                        Some(mut stream) => stream.write_all(&self.response.body[..]).unwrap(),
                        None => todo!(),
                    }
                }
            }
            None => todo!(),
        }
    }
    // is http protocol
    pub fn is(c: String) -> bool {
        let re = Regex::new(r"^(GET|HEAD|POST|PUT|DELETE|CONNECT|OPTIONS|TRACE)\s(([/0-9a-zA-Z.]+)?(\?[0-9a-zA-Z&=]+)?)\s(HTTP/1.0|HTTP/1.1|HTTP/2.0)\r\n$").unwrap();
        re.is_match(&c)
    }
    /// execute plugin
    fn exec_plugin(&mut self) {
        match ROUTER_TABLE.lock() {
            Ok(t) => {
                if t.len() > 0 && t.contains_key(&self.request.path) {
                    t.get(&self.request.path).unwrap()(self.request.clone(), self.response.clone());
                }
            }
            Err(_) => {}
        }
    }
}

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
    pub async fn multi_thread_read(r: OwnedReadHalf) -> Result<Self, String> {
        use tokio::io::AsyncBufReadExt;
        use tokio::io::AsyncReadExt;
        use tokio::io::BufReader;
        use tokio::net::tcp::OwnedReadHalf;
        let mut protocol_line = String::default();
        let mut r_buf: BufReader<OwnedReadHalf> = BufReader::new(r);
        let _ = r_buf.read_line(&mut protocol_line).await;
        if !Request::is(protocol_line.to_string()) {
            return Err("http request processing failed".to_string());
        }
        let items: Vec<&str> = protocol_line.split(" ").collect();
        let mut req_str_buf = String::default();
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
    pub async fn event_poll_read(stream: mio::net::TcpStream) -> Result<Self, String> {
        use std::io::BufRead;
        use std::io::BufReader;
        let mut r_buf = BufReader::new(stream);
        let mut protocol_line = String::default();
        let _ = r_buf.read_line(&mut protocol_line);
        if !Request::is(protocol_line.to_string()) {
            return Err("http request processing failed".to_string());
        }
        let items: Vec<&str> = protocol_line.split(" ").collect();
        let mut req_str_buf = String::default();
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
                    match r_buf.read_line(&mut req_str_buf) {
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
                            match r_buf.read(&mut buf) {
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
    pub async fn read(r: OwnedReadHalf) -> Result<Self, String> {
        use tokio::io::AsyncBufReadExt;
        use tokio::io::AsyncReadExt;
        use tokio::io::BufReader;
        use tokio::net::tcp::OwnedReadHalf;
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
        Ok(response)
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
        let resource = format!("{}{}", ROOT_PATH.lock().unwrap(), self.req_path);
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
