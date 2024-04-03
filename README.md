 
## fshare
Official Rust implementation of Command-line File-Sharing tool 🦀.

[![MIT License](https://img.shields.io/github/license/dec0dOS/amazing-github-template.svg?style=flat-square)](https://github.com/ynwqmv/netprotocol/discussions/3)
[![Version](https://img.shields.io/badge/version-1.0-red.svg)](https://github.com/ynwqmv/netplatform/blob/master/NETWORK.md)
![](https://camo.githubusercontent.com/a080948f1963a87a71216a884b318e6d84825d4cb0be5b242b3153e5b096486c/68747470733a2f2f696d672e736869656c64732e696f2f62616467652f432b2b2d536f6c7574696f6e732d626c75652e7376673f7374796c653d666c6174266c6f676f3d63253242253242)
---





## Run
```sh
cargo run <command> <path>
```
### Commands (v1.0)
#### `send` - Sends the file to the server for download.
    
 

# ⚙️ About
| `fshare` version | Dependencies  | Version | 
|---------------------|--------|---------| 
|     `v1.0`           | `ActixWeb`     | `4.5.1`  |
|                    | `qrcode` | `0.14.0`
|            | `local-ip-address` | `0.6.1` | 
|     |  `rand` | `0.8.5` |
|   |  ` actix-files` | `0.6.5`  |
 
 ## Example
 ```sh
cargo run send path/to/file
```
 
 ```sh
Scan the qrcode: 

█▀▀▀▀▀█ ▄██ ▀▄▀▄█▄ █▀ █▀▀▀▀▀█
█ ███ █  █ ▀▀▄▀ █▀█▄▀ █ ███ █
█ ▀▀▀ █ ▀▄▄▀▄▀█▄  ██▀ █ ▀▀▀ █
▀▀▀▀▀▀▀ ▀ █▄█ ▀▄▀▄█ ▀ ▀▀▀▀▀▀▀
▀▄▀▄█ ▀▄ ▄  █▄ █▄▄█▀  ▄▄▀▄ ▀▄
▄▀▄▄▄█▀ █ ▀ ▄▄█▀ █▄ ▄███▀▄▀█▀
█ █▄ █▀▀▄ ▄▄▀ ██ █▀   ██ █▄▀█
▀█▄▀█▄▀▀ ▄  ▀▀   █▄▄▀▄  ▄▄ █▀
 ▄█▀▄█▀██▄█ ▄▄ █ ▄▀  ▄█▄  ▄▀█
▀▄▄▄ ▀▀▄█▀█▄ ▄█▀ ▄█▄ █▄▄ █ █▀
▀ ▀▀ ▀▀▀▄█ █▄ ██▄▄▀ █▀▀▀█ ▄▄▄
█▀▀▀▀▀█  ██▀▄▀ █▄█▀██ ▀ ██ ▀▀
█ ███ █ ▀▄▄██▄ █▄ █▄██▀▀██ ▀▀
█ ▀▀▀ █ ▀█▄▄▀▄█▀ ▄██▄█  ▀█▀▄▀
▀▀▀▀▀▀▀ ▀ ▀ ▀  ▀  ▀ ▀▀▀  ▀ ▀▀

Or manually open the URL in browser http://192.168.1.107:56459/download
[*] Server is listening on 0.0.0.0:56459
Sending file "file"... to 192.168.1.107:56459
```
 

____

