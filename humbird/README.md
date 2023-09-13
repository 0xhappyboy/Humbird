## 🐦 Humbird
humbird network communication core library  
**You Know, for Faster!**
## 📦 Crates
```

```
## 👉 Usage
```rust
use humbird::{
    config::config::load_config,
    run,
};

fn main() {
    // load config
    load_config("config-template.toml".to_string());
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
