use humbird::{
    protocol::http::{request::Request, response::Response},
  run, config::config::load_config,
};

fn main() {
    // load config
    load_config("".to_string());
    // cli
    //let cli = Cli::parse();
    run!();
}
