use crate::config::config::*;

use tokio::{
    io::{AsyncBufReadExt},
    net::{tcp::OwnedWriteHalf, TcpListener},
};

pub struct HttpServer {}

impl HttpServer {
    pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
        let l = TcpListener::bind(format!("{}:{}", SERVER_LISTENING_ADDR, unsafe {
            SERVER_LISTENING_PORT
        }))
        .await?;
        loop {
            let (mut stream, _socket) = l.accept().await?;
            let (r, w) = stream.into_split();
            let mut r_buf = tokio::io::BufReader::new(r);
            let mut w_buf: tokio::io::BufWriter<tokio::net::tcp::OwnedWriteHalf> =
                tokio::io::BufWriter::new(w);
            let mut req_buf = String::new();
            loop {
                match r_buf.read_line(&mut req_buf).await {
                    Err(_e) => {
                        eprintln!("read from client error");
                        break;
                    }
                    Ok(0) => {
                        eprintln!("EOF");
                        break;
                    }
                    Ok(n) => {
                        let content = req_buf.drain(..).as_str().to_string();
                        // TODO
                    }
                }
            }
        }
    }
}

/// handle http message
fn handle_http() {}

fn handle_http_response(
    mut w: tokio::io::BufWriter<OwnedWriteHalf>,
) -> Result<(), Box<dyn std::error::Error>> {
    // TODO
    Ok(())
}
