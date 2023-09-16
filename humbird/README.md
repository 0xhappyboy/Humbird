## 🐦 Humbird
humbird network communication core library  
**You Know, for Faster!**
## 📦 Crates
```
humbird = "0.1.2"
```
## 👉 Usage
```rust
use humbird::{
    config::config::load_config,
    protocol::http::{Request, Response},
    register_router_plugin, run,
};

fn test_1(req: Request, mut res: Response) -> Response {
    res.body = "response.....".as_bytes().to_vec();
    res
}

fn test_2(req: Request, mut res: Response) -> Response {
    res.body = "response.....".as_bytes().to_vec();
    res
}

fn main() {
    // load config
    load_config(
        "/Users/max/GitProject/Humbird/humbird-server/src/config-template.toml".to_string(),
    );
    // register routing plug-in
    // path binding
    register_router_plugin!(
        "/".to_string() => test_1,
        "/test".to_string() => test_2
    );
    // run humbird server
    run!();
}
```
## 📃 Configuration
Server configuration file templat
```
[server]
# port
port = "port"

[directory]
# local static resource path
root-path = ""

[proxy]
# target proxy host list
target = ["0.0.0.0:80", ""0.0.0.0:8080", "0.0.0.0:8888"]
# WEIGHT : weight mode
# RANDOM : random mode
# POLLING : polling mode
mode = "WEIGHT"
```
