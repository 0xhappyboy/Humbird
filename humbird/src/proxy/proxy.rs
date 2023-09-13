use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::protocol::http::{request::Request, response::Response};

/// load balancing mode
#[derive(Debug, Clone)]
pub enum BalancingMode {
    /// weight mode
    WEIGHT,
    /// random mode
    RANDOM,
    /// polling mode
    POLLING,
}

/// network agent abstract structure
#[derive(Debug, Clone)]
pub struct Proxy {
    /// request abstract structure
    request: Request,
    /// Response abstract structure
    response: Response,
}

impl Proxy {
    /// load balancing
    pub async fn load_balancing(
        host: &str,
        port: &str,
        request: Request,
        mode: BalancingMode,
    ) -> Result<Self, String> {
        match mode {
            BalancingMode::WEIGHT => Proxy::to(host, port, request).await,
            BalancingMode::RANDOM => Proxy::to(host, port, request).await,
            BalancingMode::POLLING => Proxy::to(host, port, request).await,
            _ => Err("".to_string()),
        }
    }
    /// forward the request to a third-party server
    pub async fn to(host: &str, port: &str, request: Request) -> Result<Self, String> {
        let t = TcpStream::connect(format!("{}:{}", host, port))
            .await
            .unwrap();
        let (r, mut w) = t.into_split();
        let _ = w.write_all(request.raw.as_bytes()).await;
        match Response::read(r).await {
            Ok(response) => {
                return Ok(Proxy {
                    request: request,
                    response: response,
                });
            }
            Err(_) => {
                // TODO
                return Err("".to_string());
            }
        }
    }
}
