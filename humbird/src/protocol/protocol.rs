use std::collections::HashMap;

use mio::{net::TcpStream, Token};

use super::http::Http;

pub fn handle_protocol(m: &HashMap<Token, TcpStream>, token: &Token) {
    match m.get(token) {
        Some(stream) => {
            let _ = Http::new(m, token);
        }
        None => {}
    }
}

/// base protocol trait
/// all communication protocols should implement this feature
///
/// Example
/// ```rust
///
/// ```
pub trait BaseProtocol<T> {
    fn new(m: &HashMap<Token, TcpStream>, token: &Token) -> Result<T, String>;
    fn is(stream: &mio::net::TcpStream) -> bool;
}
