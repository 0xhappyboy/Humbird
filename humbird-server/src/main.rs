use humbird::{
    config::config::load_config,
    protocol::http::{request::Request, response::Response},
    run,
};

fn main() {
    // load config
    load_config("".to_string());
    // cli
    //let cli = Cli::parse();
    run!();
}
