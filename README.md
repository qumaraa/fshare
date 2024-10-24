 
## fshare 2.1 (Latest) 📁
**Official Rust implementation of Command-line File-Sharing tool** 🦀.
#### Available Send & Upload features!

[![MIT License](https://img.shields.io/github/license/dec0dOS/amazing-github-template.svg?style=flat-square)](https://github.com/ynwqmv/netprotocol/discussions/3)
[![Version](https://img.shields.io/badge/version-2.1-red.svg)](https://github.com/ynwqmv/netplatform/blob/master/NETWORK.md)
![](https://camo.githubusercontent.com/a080948f1963a87a71216a884b318e6d84825d4cb0be5b242b3153e5b096486c/68747470733a2f2f696d672e736869656c64732e696f2f62616467652f432b2b2d536f6c7574696f6e732d626c75652e7376673f7374796c653d666c6174266c6f676f3d63253242253242)
---


 
# Logs 🖥️
#### `log_debug` (set as default)
#### `log_info`
#### `log_warn`  
#### `log_err` 
#### `log_trace`  
 

# Port Generation
```rs
let port = rng.gen_range(49152..=65535);
```
```rs
let srv = HttpServer::new({
       /**/
    })
    .bind(("0.0.0.0", port.clone())) /* <- binds to a local network with a randomly generated port */
    .unwrap()
    .run();

    /*
      Sometimes it may happen that the code can generate
      a port that is used by the operating system or other programs, but it's not critical.
    */
```
 
 ## Example 
```powershell
Usage: fshare <COMMAND>

Commands:
  send
  recv
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
```powershell
fshare help send
Usage: fshare send --path <PATH> --log <LOG>

Options:
  -p, --path <PATH>  Path of file for sending to server for downloading.
  -l, --log <LOG>    Choose log-level. Level::DEBUG set as default.
  -h, --help         Print help
```
```powershell
fshare help recv
Usage: fshare recv --log <LOG>

Options:
  -l, --log <LOG>  Choose log-level. Level::DEBUG set as default.
  -h, --help       Print help
```
```powershell
fshare --version
fshare 2.1
```
____



