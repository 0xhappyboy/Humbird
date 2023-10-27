/// core network service module, providing core network functions
use crate::protocol::http::Http;
use chrono::Local;
use lazy_static::lazy_static;
use mio::{Events, Interest, Poll, Token};
use std::{collections::HashMap, sync::Mutex};
use tokio::runtime::Runtime;
use tracing::Level;
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

/// network services core abstraction
pub struct Server {
    rt: Runtime,
}

impl Server {
    /// start server,based on configuration files
    ///
    /// Example
    /// ``` rust
    /// Server::config_run("/config.toml");
    /// ```
    pub fn config_run(config_file_path: &str) {
        Server::config(Some(config_file_path.to_string()));
        Server::run();
    }
    /// start server
    ///
    /// Example
    /// ```rust
    /// Server::run();
    /// ```
    pub fn run() {
        match Server::new() {
            Some(s) => {
                // initialize the log system
                init_log();
                s.event_poll();
            }
            None => {
                tracing::error!("server instance creation failed");
            }
        }
    }
    /// handle server global configurable constants, based on configuration files
    fn config(config_file_path: Option<String>) {
        load_config(config_file_path);
    }
    /// create a network service core abstraction instance
    fn new() -> Option<Server> {
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
    /// handle evet poll
    fn event_poll(&self) {
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
                let mut server = mio::net::TcpListener::bind(address).unwrap();
                match poll.registry().register(
                    &mut server,
                    HUMBIRD_SERVER_TOKEN,
                    Interest::READABLE.add(Interest::WRITABLE),
                ) {
                    Ok(_) => {
                        // connection pool mapping
                        let mut connections = HashMap::new();
                        let mut unique_token = Token(HUMBIRD_SERVER_TOKEN.0 + 1);
                        // launch info
                        println!("{}", boot_info_string(true));
                        loop {
                            let _ = poll.poll(&mut events, None).unwrap();
                            for event in events.iter() {
                                match event.token() {
                                    // a new connection
                                    HUMBIRD_SERVER_TOKEN => {
                                        let (mut connection, _address) = match server.accept() {
                                            Ok((connection, address)) => (connection, address),
                                            Err(e)
                                                if e.kind() == std::io::ErrorKind::WouldBlock =>
                                            {
                                                break;
                                            }
                                            Err(_e) => {
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
                                        match Http::new(&event, &connections, &token) {
                                            Ok(_http) => {
                                                continue;
                                            }
                                            Err(_) => {
                                                continue;
                                            }
                                        }
                                    }
                                    // reuse
                                    token => {
                                        if connections.contains_key(&token) {
                                            match connections.get(&token) {
                                                Some(_stream) => {
                                                    match Http::new(&event, &connections, &token) {
                                                        Ok(_http) => {
                                                            continue;
                                                        }
                                                        Err(_) => {
                                                            continue;
                                                        }
                                                    }
                                                }
                                                None => {
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(_) => {
                        // launch info
                        println!("{}", boot_info_string(false));
                        return;
                    }
                }
            }
            Err(_) => {
                self.event_poll();
            }
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
        .with_writer(std::io::stdout)
        .with_writer(non_blocking)
        .with_ansi(false)
        .event_format(format)
        .init();
}

use prettytable::{row, Table};

use super::config::load_config;

pub fn boot_info_string(status: bool) -> String {
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
        if status { "✅" } else { "⛔" }
    ]);
    table.add_row(row!["Address", "Port", "", "", "", ""]);
    table.add_row(row![
        SERVER_LISTENING_ADDR,
        DEFAULT_SERVER_LISTENING_PORT,
        "",
        " ",
        "",
        ""
    ]);
    format!("{}\n{}", logo, table.to_string())
}
