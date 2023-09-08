use std::{fs, io::Read};

use serde::{Deserialize, Serialize};

/// server listening address
pub const SERVER_LISTENING_ADDR: &'static str = "0.0.0.0";
/// server listening port,default 9999
pub static mut SERVER_LISTENING_PORT: &str = "9999";
// local static resources root path
pub static mut ROOT_PATH: &str = "";

// persistence configuration
pub static mut CONFIG: Option<Config> = None;

// config
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Config {
    listening_addr: String,
    listening_port: String,
    root_path: String,
}

impl Config {
    pub fn new(path: String) {
        let file = fs::File::open(path);
        match file {
            Ok(mut f) => unsafe {
                let mut s_buf = String::default();
                let _ = f.read_to_string(&mut s_buf);
                CONFIG = Some(Config {
                    listening_addr: todo!(),
                    listening_port: todo!(),
                    root_path: todo!(),
                })
            },
            Err(_) => {
                // TODO
            }
        }
    }
    // re-load the parameters in the configuration file into the structure
    pub fn reload(path: String) {}
}
