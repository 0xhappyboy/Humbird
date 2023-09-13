#![doc(html_logo_url = "https://github.com/0xhappyboy/humbird/blob/main/assets/imgs/logo.jpg")]
/// this module is responsible for the underlying network communication
pub mod config;
/// this module is responsible for the underlying network communication
pub mod core;
/// this module encapsulates some commonly used utility macros
pub mod macros;
/// this module provides network protocol abstraction
pub mod protocol;
pub mod plugins;