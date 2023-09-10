use std::{collections::HashMap, fs, io::Error, path::Path};

use regex::Regex;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
    task,
};
use tracing::{error, event, info, instrument, Level};

use crate::{async_exe, config::config::ROOT_PATH};

use super::{method::Method, request::Request, response::Response};

// overall encapsulation of http protocol packets
#[derive(Debug)]
pub struct Http {
    pub w: OwnedWriteHalf,
    pub request: Request,
    pub response: Response,
}

impl Http {
    #[instrument]
    pub async fn new(
        request_line: String,
        r: BufReader<OwnedReadHalf>,
        w: OwnedWriteHalf,
    ) -> Result<Http, String> {
        if !Http::is_http_protocol(request_line.clone()) {
            return Err("http request processing failed".to_string());
        }
        match Request::new(request_line, r).await {
            Ok(req) => {
                let response = Response::new(&req).await;
                let mut http = Http {
                    w: w,
                    request: req,
                    response: response,
                };
                return Ok(http);
            }
            Err(e) => {
                error!("http request processing failed");
                return Err("http request processing failed".to_string());
            }
        }
    }
    // response
    pub async fn response(mut self) {
        let _ = self.w.write_all(&self.response.body[..]).await;
    }
    // is http protocol
    pub fn is_http_protocol(c: String) -> bool {
        let re = Regex::new(r"^(GET|HEAD|POST|PUT|DELETE|CONNECT|OPTIONS|TRACE)\s(([/0-9a-zA-Z.]+)?(\?[0-9a-zA-Z&=]+)?)\s(HTTP/1.0|HTTP/1.1|HTTP/2.0)\r\n$").unwrap();
        re.is_match(&c)
    }
}
