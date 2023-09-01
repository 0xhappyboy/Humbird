use std::{fs, path::Path};

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
};

use crate::config::config::ROOT_PATH;

#[derive(Debug)]
pub struct Http {
    pub method: String,
    pub path: String,
    pub protocol: String,
    pub w: OwnedWriteHalf,
}

impl Http {
    pub async fn New(c: String, mut r_buf: BufReader<OwnedReadHalf>, w: OwnedWriteHalf) -> Http {
        let items: Vec<&str> = c.split(" ").collect();
        let mut http = Http {
            w: w,
            method: items[0].to_string(),
            path: items[1].to_string(),
            protocol: items[2].to_string().replace("\r\n", ""),
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
                    let head_items: Vec<&str> = c.split(":").collect();
                    match head_items[0] {
                        _ if head_items[0] == "Host" => {
                            // TODO
                        }
                        _ => {
                            // TODO
                        }
                    }
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
}
