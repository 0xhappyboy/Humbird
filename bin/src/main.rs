mod cli;
use crate::cli::cli::Cli;
use chrono::Local;
use clap::Parser;
use humbird::config::config::Config;
use humbird::server::server::Server;
use std::io;
use tracing::Level;
use tracing_subscriber::fmt::{format::Writer, time::FormatTime};

fn main() {
    // config file init,load
    Config::new("".to_string());
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
    // cli
    // let cli = Cli::parse();
    let r = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(10)
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            Server::run().await;
        });
}

// log time
struct LocalTimer;

impl FormatTime for LocalTimer {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%FT%T%.3f"))
    }
}
