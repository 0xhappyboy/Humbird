/// core network service module, providing core network functions
use crate::{async_exe, protocol::http::Http};
use chrono::Local;
use lazy_static::lazy_static;
use mio::{event::Event, Events, Interest, Poll, Token};
use std::{collections::HashMap, io, sync::Mutex};
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
/// server listening default port
pub const DEFAULT_SERVER_LISTENING_PORT: &'static str = "9999";
/// global constants related to services
lazy_static! {
   /// server listening port,default 9999
   pub static ref SERVER_LISTENING_PORT: Mutex<String> = Mutex::new(String::from(DEFAULT_SERVER_LISTENING_PORT.to_string()));
   /// local static resources root path
   pub static ref ROOT_PATH: Mutex<String> = Mutex::new(String::default());
}
// humbird server token
const HUMBIRD_SERVER_TOKEN: Token = Token(0);
// event pool count
const EVENT_POOL_COUNT: usize = 1024;
// network model
#[derive(Debug)]
pub enum NetModel {
    Multithread,
    EventPoll,
}

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
                server.start($crate::core::server::NetModel::EventPoll);
            }
            None => {
                tracing::error!("server instance creation failed");
            }
        }
    };
}

/// network services core abstraction
pub struct Server {
    rt: Runtime,
}

impl Server {
    /// create a network service core abstraction instance
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
    pub fn start(&self, model: NetModel) {
        init_log();
        match model {
            NetModel::Multithread => self.handle_multi_thread(),
            NetModel::EventPoll => self.handle_event_poll(),
        }
    }
    /// handle multi thread
    fn handle_multi_thread(&self) {
        self.rt.block_on(async {
            // tcp listener
            let l = TcpListener::bind(format!(
                "{}:{}",
                SERVER_LISTENING_ADDR,
                SERVER_LISTENING_PORT.lock().unwrap()
            ))
            .await
            .unwrap();
            loop {
                let (stream, socket) = l.accept().await.unwrap();
                info!("new visitor,ip:{}", socket.ip());
                Server::to_multi_thread_http(stream).await;
            }
        });
    }
    /// handle evet poll
    fn handle_event_poll(&self) {
        use mio::net::TcpListener;
        match Poll::new() {
            Ok(mut poll) => {
                let mut events = Events::with_capacity(EVENT_POOL_COUNT);
                let address = format!(
                    "{}:{}",
                    SERVER_LISTENING_ADDR,
                    SERVER_LISTENING_PORT.lock().unwrap()
                )
                .parse()
                .unwrap();
                let mut server = TcpListener::bind(address).unwrap();
                poll.registry()
                    .register(
                        &mut server,
                        HUMBIRD_SERVER_TOKEN,
                        Interest::READABLE.add(Interest::WRITABLE),
                    )
                    .unwrap();
                // connection pool mapping
                let mut connections = HashMap::new();
                let mut unique_token = Token(HUMBIRD_SERVER_TOKEN.0 + 1);
                loop {
                    let _ = poll.poll(&mut events, None).unwrap();
                    for event in events.iter() {
                        match event.token() {
                            // a new connection
                            HUMBIRD_SERVER_TOKEN => {
                                let (mut connection, address) = match server.accept() {
                                    Ok((connection, address)) => (connection, address),
                                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                                        break;
                                    }
                                    Err(e) => {
                                        break;
                                    }
                                };
                                // the unique token of the tcp link
                                let token = {
                                    let next = unique_token.0;
                                    unique_token.0 += 1;
                                    Token(next)
                                };
                                poll.registry()
                                    .register(
                                        &mut connection,
                                        token,
                                        Interest::READABLE.add(Interest::WRITABLE),
                                    )
                                    .unwrap();
                                connections.insert(token, connection);
                            }
                            // reuse
                            token => {
                                if connections.contains_key(&token) {
                                    match connections.get(&token) {
                                        Some(stream) => {
                                            Server::to_event_poll_http(
                                                event,
                                                connections.get(&token).unwrap(),
                                            );
                                        }
                                        None => {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(_) => {}
        }
    }
    /// handle tcp message
    ///
    /// Example
    /// ```rust
    /// Server::start();
    /// ```
    async fn to_multi_thread_http(stream: tokio::net::TcpStream) {
        match Http::new_multi_thread(stream).await {
            Ok(http) => {
                http.multi_thread_response().await;
            }
            Err(_) => {}
        }
    }
    fn to_event_poll_http(event: &Event, stream: &mio::net::TcpStream) {
        match Http::new_event_poll(event, stream) {
            Ok(http) => {
                http.event_poll_response();
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
        "You Know, for Faster! ",
        "0xhappyboy",
        "✅"
    ]);
    format!("{}\n{}", logo, table.to_string())
}
