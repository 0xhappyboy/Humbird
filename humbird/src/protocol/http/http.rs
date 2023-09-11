use regex::Regex;
use tokio::{
    io::{AsyncWriteExt, BufReader},
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
};
use tracing::{error, instrument};

use super::{request::Request, response::Response};
use crate::plugins::web::ROUTER_TABLE;

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
            Ok(request) => {
                let response = Response::new(&request);
                let mut http = Http {
                    w: w,
                    request: request,
                    response: response,
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
    // response
    pub async fn response(mut self) {
        let _ = self.w.write_all(&self.response.body[..]).await;
    }
    // is http protocol
    pub fn is_http_protocol(c: String) -> bool {
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
