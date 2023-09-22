## <center>🐦 Humbird </center>  
the high-performance server,Let network transmission be like hummingbird flapping its wings.  
**You Know, for Faster!**  
## 📚 Directory
| **directory** | **role** | 
| :-----| :----- |
| **humbird** | server core library, which can be applied to any project |  
| **humbird-server** | server application |  
| **humbird-jdk** | server jdk, used for introduction and use in Java Maven projects, using JNI to link core libraries |  
| **humbird-homepage** | humbird homepage |  
| **humbird-examples** | code Cases Related to Humbird Server |  
## 🚚 Crates
```
https://crates.io/crates/humbird
```
## 📄 Docs
```
https://docs.rs/humbird/0.1.3/humbird/
```
## 👉 Command
```
You Know, for Faster! 

Usage: humbird-server [OPTIONS]

Options:
  -p, --port <PORT>
          server port (default: 9999).
  -h, --help
          Print help
  -V, --version
          Print version
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
## 🗓️ Plan
| **Plan** | **Status** | 
| :-----| :----- |
| Support for some common web protocols,e.g.HTTP 0.9、HTTP 1.0、HTTP 1.1、HTTP 2.0. | ⏲ |  
| Support for load balancing strategies. | ✅ |  
| Support access to static resources. | ✅ |  
| Universal codec. | ⏲ |  
| Support related functions of the registration center. | ⏲ |  
| Support distributed related functions such as cluster deployment and service discovery. | ⏲ |
| Supports dynamic configuration of network load for rational allocation of valuable network resources. | ⏲ |  
| dash board. | ⏲ |  
| look forward to more.... | ⏲ |  