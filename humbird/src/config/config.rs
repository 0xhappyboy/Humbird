use serde::{Deserialize, Serialize};
use std::{fs, io::Read};

/// server listening address
pub const SERVER_LISTENING_ADDR: &'static str = "0.0.0.0";
/// server listening port,default 9999
pub static mut SERVER_LISTENING_PORT: &str = "9999";
// local static resources root path
pub static mut ROOT_PATH: &str = "";

// server config
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Server {
    addr: Option<String>,
    port: Option<u32>,
    webapps: Option<String>,
}

pub fn load_config(path: String) {
    let file = fs::File::open(path);
    match file {
        Ok(mut f) => unsafe {
            let mut s_buf = String::default();
            let _ = f.read_to_string(&mut s_buf);
            let config = s_buf.parse::<toml::Table>().unwrap();
        },
        Err(_) => {
            // TODO
        }
    }
}