use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::protocol::http::{request::Request, response::Response};

#[derive(Debug)]
pub struct Proxy {
    request: Request,
    response: Response,
}

impl Proxy {
    pub async fn request(addr: String) -> Result<Response, String> {
        let t = TcpStream::connect("").await.unwrap();
        let (mut r, mut w) = t.into_split();
        let _ = w.write_all("".as_bytes()).await;
        match Response::read(r).await {
            Ok(response) => {
                return Ok(response);
            }
            Err(_) => {
                // TODO
                return Err("".to_string());
            }
        }
    }
}
