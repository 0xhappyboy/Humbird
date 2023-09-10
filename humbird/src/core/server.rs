/// core network service module, providing core network functions
use crate::{async_exe, boot_output, config::config::*};

use regex::Regex;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener,
    },
    task,
};
use tracing::{info, instrument};

use crate::protocol::http::http::*;

/// used to start services, requires asynchronous runtime
///
/// Example
/// ```rust
/// run!();
/// ```
#[macro_export]
macro_rules! run {
    () => {
        use $crate::core::server::Server;
        Server::start().await;
    };
}

pub struct Server {}

impl Server {
    /// used to start services, requires asynchronous runtime
    ///
    /// Example
    /// ```rust
    /// Server::start();
    /// ```
    pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
        // tcp listener
        let l = TcpListener::bind(format!("{}:{}", SERVER_LISTENING_ADDR, unsafe {
            SERVER_LISTENING_PORT
        }))
        .await?;
        boot_output!();
        loop {
            let (stream, socket) = l.accept().await?;
            info!("new visitor,ip:{}", socket.ip());
            let (r, w) = stream.into_split();
            async_exe!(Server::handle_tcp(r, w));
        }
    }

    /// handle tcp message
    ///
    /// Example
    /// ```rust
    /// Server::start();
    /// ```
    #[instrument]
    async fn handle_tcp(r: OwnedReadHalf, w: OwnedWriteHalf) {
        let mut req_str_buf = String::new();
        let mut r_buf: BufReader<OwnedReadHalf> = BufReader::new(r);
        loop {
            match r_buf.read_line(&mut req_str_buf).await {
                Ok(0) => {
                    break;
                }
                Ok(_n) => {
                    let c = req_str_buf.drain(..).as_str().to_string();
                    match Http::new(c, r_buf, w).await {
                        Ok(http) => {
                            // respose
                            async_exe!(http.response());
                        }
                        Err(_) => {}
                    }
                    break;
                }
                Err(_) => {}
            }
        }
    }
}
