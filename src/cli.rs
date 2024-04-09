use crate::server;
use crate::server::Action;
use crate::VERSION;
use std::env::args;
use std::path::PathBuf;
use tokio::fs;
use tracing::log::__private_api::log;
use tracing::{debug, error, info, warn};
use tracing_subscriber::FmtSubscriber;

pub async fn start() {
    let mut logstate = tracing::Level::DEBUG;

    let args: Vec<String> = args().collect();
    let mut action = Action::Download {
        file_path: PathBuf::new(),
    };
    if args.len() < 3 {
        eprintln!("Usage: {} <command> <path>", &args[0]);
        eprintln!("Commands: send,recv");
        std::process::exit(1);
    }

    let command = &args[1];
    let subcommand = &args[2];

    match subcommand.as_str() {
        "log_info" => {
            logstate = tracing::Level::INFO;
        }
        "log_debug" => {
            logstate = tracing::Level::DEBUG;
        }
        "log_err" => {
            logstate = tracing::Level::ERROR;
        }
        "log_warn" => {
            logstate = tracing::Level::WARN;
        }
        "log_trace" => {
            logstate = tracing::Level::TRACE;
        }
        "--" => {
        }
        _ => {
            eprintln!("Unknown subcommand");
            std::process::exit(1);
        }
    }
    if subcommand.as_str() != "--" {
        let subscriber = FmtSubscriber::builder().with_max_level(logstate).finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
        info!("log initialized");
    }

    match command.as_str() {
        "send" => {
            let path_buf = PathBuf::from(subcommand);
            // CH:1.1
            if !path_buf.is_dir() && path_buf.is_file() {
                debug!("version {}", VERSION);
                action = Action::Download {
                    file_path: path_buf,
                };
            } else {
                eprintln!("Error: please select a file instead!");
                std::process::exit(1);
            }
        }
        "recv" => {
            let fs = match tokio::fs::create_dir_all("uploads").await {
                Ok(_) => {
                    debug!("created dir `/uploads`");
                }
                Err(err) => {
                    error!("error creating dir: {err}");
                }
            };
            debug!("version {}", VERSION);
            action = Action::Upload;
        }
        _ => {
            eprintln!("Usage: {} <command> <path>", &args[0]);
            std::process::exit(1);
        }
    }


    let _ = crate::server::start(action).await;
}
