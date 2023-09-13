/// core network service module, providing core network functions
use crate::async_exe;
use crate::protocol::http::http::*;
use chrono::Local;
use std::io;
use tokio::{
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpListener,
    },
    runtime::Runtime,
    task,
};
use tracing::Level;
use tracing::{info, instrument};
use tracing_subscriber::fmt::{format::Writer, time::FormatTime};

/// server listening address
pub const SERVER_LISTENING_ADDR: &'static str = "0.0.0.0";
/// server listening port,default 9999
pub static mut SERVER_LISTENING_PORT: &str = "";
/// local static resources root path
pub static mut ROOT_PATH: &str = "";

/// used to start services, requires asynchronous runtime
///
/// Example
/// ```rust
/// run!();
/// ```
#[macro_export]
macro_rules! run {
    () => {
        match $crate::core::server::Server::new() {
            Some(server) => {
                server.start();
            }
            None => {
                tracing::error!("server instance creation failed");
            }
        }
    };
}

pub struct Server {
    rt: Runtime,
}

impl Server {
    pub fn new() -> Option<Server> {
        let r = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(10)
            .enable_all()
            .build();
        match r {
            Ok(rt) => {
                let server = Server { rt: rt };
                return Some(server);
            }
            Err(_) => {
                return None;
            }
        }
    }
    /// used to start services, requires asynchronous runtime
    ///
    /// Example
    /// ```rust
    /// Server::start();
    /// ```
    pub fn start(&self) {
        init_log();
        self.rt.block_on(async {
            // tcp listener
            let l = TcpListener::bind(format!("{}:{}", SERVER_LISTENING_ADDR, unsafe {
                SERVER_LISTENING_PORT
            }))
            .await
            .unwrap();
            loop {
                let (stream, socket) = l.accept().await.unwrap();
                info!("new visitor,ip:{}", socket.ip());
                let (r, w) = stream.into_split();
                async_exe!(Server::handle_tcp(r, w));
            }
        });
    }

    /// handle tcp message
    ///
    /// Example
    /// ```rust
    /// Server::start();
    /// ```
    #[instrument]
    async fn handle_tcp(r: OwnedReadHalf, w: OwnedWriteHalf) {
        match Http::new(r, w).await {
            Ok(http) => {
                // respose
                async_exe!(http.response());
            }
            Err(_) => {}
        }
    }
}

pub fn init_log() {
    // log time
    struct LocalTimer;
    impl FormatTime for LocalTimer {
        fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
            write!(w, "{}", Local::now().format("%FT%T%.3f"))
        }
    }
    // log init
    let file_appender = tracing_appender::rolling::daily("", "numbird.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(true)
        .with_timer(LocalTimer);
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_writer(io::stdout)
        .with_writer(non_blocking)
        .with_ansi(false)
        .event_format(format)
        .init();
}

use prettytable::{row, Table};

pub fn boot_info_string() -> String {
    let logo: &str = "
██░ ██  █    ██  ███▄ ▄███▓ ▄▄▄▄    ██▓ ██▀███  ▓█████▄ 
▓██░ ██▒ ██  ▓██▒▓██▒▀█▀ ██▒▓█████▄ ▓██▒▓██ ▒ ██▒▒██▀ ██▌
▒██▀▀██░▓██  ▒██░▓██    ▓██░▒██▒ ▄██▒██▒▓██ ░▄█ ▒░██   █▌
░▓█ ░██ ▓▓█  ░██░▒██    ▒██ ▒██░█▀  ░██░▒██▀▀█▄  ░▓█▄   ▌
░▓█▒░██▓▒▒█████▓ ▒██▒   ░██▒░▓█  ▀█▓░██░░██▓ ▒██▒░▒████▓ 
▒ ░░▒░▒░▒▓▒ ▒ ▒ ░ ▒░   ░  ░░▒▓███▀▒░▓  ░ ▒▓ ░▒▓░ ▒▒▓  ▒ 
▒ ░▒░ ░░░▒░ ░ ░ ░  ░      ░▒░▒   ░  ▒ ░  ░▒ ░ ▒░ ░ ▒  ▒ 
░  ░░ ░ ░░░ ░ ░ ░      ░    ░    ░  ▒ ░  ░░   ░  ░ ░  ░ 
░  ░  ░   ░            ░    ░       ░     ░        ░    
                                     ░               ░";
    let mut table = Table::new();
    table.add_row(row![
        "Name", "Version", "Author", "Slogan", "Github", "Status"
    ]);
    table.add_row(row![
        "🐦Humbird",
        "v0.1.0",
        "HappyBoy🎈",
        "You know,for faster",
        "0xhappyboy",
        "✅"
    ]);
    format!("{}\n{}", logo, table.to_string())
}
