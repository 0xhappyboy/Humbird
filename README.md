# 🐦 Humbird
the high-performance server,Let network transmission be like hummingbird flapping its wings.  
**You Know, for Faster!**
# 👉 Command
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
# 📃 Configuration
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
# 🗓️ Plan
1. Support for some common web protocols,e.g.HTTP 0.9、HTTP 1.0、HTTP 1.1、HTTP 2.0. :stopwatch:
2. Support for load balancing strategies.:stopwatch:
3. Support access to static resources.:stopwatch:
4. Support related functions of the registration center.:stopwatch:
5. Support distributed related functions such as cluster deployment and service discovery.:stopwatch:
6. Supports dynamic configuration of network load for rational allocation of valuable network resources.:stopwatch:
7. dash board.:stopwatch:
8. look forward to more....

# 🖼 Logo
Soon ....
![Humbird](https://github.com/0xhappyboy/humbird/blob/main/assets/imgs/logo_2.jpg "Humbird Server")