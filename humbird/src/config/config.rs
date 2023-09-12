use serde::{Deserialize, Serialize};
use std::{fs, io::Read};

/// server listening address
pub const SERVER_LISTENING_ADDR: &'static str = "0.0.0.0";
/// server listening port,default 9999
pub static mut SERVER_LISTENING_PORT: &str = "9999";
// local static resources root path
pub static mut ROOT_PATH: &str = "";

/// server config
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Server {
    /// localhost server address
    addr: Option<String>,
    /// localhost listening port
    port: Option<u32>,
    /// static resource root directory
    webapps: Option<String>,
    plugins: Option<String>,
}

/// plugins manage
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Plugins {
    path: Option<Vec<String>>,
}

/// proxy
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct Porxy {
    /// poxy host address
    ///
    /// ```
    /// [
    /// ip:port,
    /// ip:port,
    /// ]
    /// ```
    host: Option<Vec<String>>,
}

/// load confin file
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
