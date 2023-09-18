use serde::{Deserialize, Serialize};
use std::{fs, io::Read};

use crate::{
    core::proxy::PROXY_TARGET,
    core::server::{ROOT_PATH, SERVER_LISTENING_PORT},
};

/// server config
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
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
            let config = s_buf.parse::<toml::Table>().unwrap().clone();
            // server
            if config.contains_key("server") {
                match config["server"].get("port") {
                    Some(p) => {
                        SERVER_LISTENING_PORT.lock().unwrap().clear();
                        SERVER_LISTENING_PORT
                            .lock()
                            .unwrap()
                            .push_str(&p.to_string());
                    }
                    None => todo!(),
                }
            }
            // directory
            if config.contains_key("directory") {
                match config["directory"].get("root-path") {
                    Some(p) => {
                        ROOT_PATH.lock().unwrap().clear();
                        ROOT_PATH.lock().unwrap().push_str(&p.to_string());
                    }
                    None => todo!(),
                }
            }
            // porxy
            if config.contains_key("proxy") {
                match config["proxy"].get("target") {
                    Some(p) => p
                        .as_array()
                        .unwrap()
                        .iter()
                        .for_each(|e| PROXY_TARGET.push(e.to_string())),
                    None => todo!(),
                }
            }
        },
        Err(_) => {
            // TODO
        }
    }
}
