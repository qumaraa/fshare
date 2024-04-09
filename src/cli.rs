use crate::server;
use crate::server::Action;
use crate::VERSION;
use std::env::args;
use std::path::PathBuf;
use tracing;
use tracing::log::__private_api::log;
use tracing::{debug, error, info, trace, warn, Level};
use tracing_subscriber::FmtSubscriber;

pub async fn start() {
    let mut logger = tracing::Level::DEBUG;

    let args: Vec<String> = args().collect();
    let mut action = Action::Download {
        file_path: PathBuf::new(),
    };
    if args.len() < 3 {
        eprintln!("Usage: {} <command> <path>", &args[0]);
        eprintln!("Commands: send, recv");
        std::process::exit(1);
    }

    let command = &args[1];
    let subcommand = &args[2];

    match subcommand.as_str().to_uppercase().as_str() {
        "log_info" => {
            logger = tracing::Level::INFO;
        }
        "log_warn" => {
            logger = tracing::Level::INFO;
        }
        "log_err" => {
            logger = tracing::Level::ERROR;
        }
        "log_debug" => {
            logger = tracing::Level::DEBUG;
        }
        "log_trace" => {
            logger = tracing::Level::TRACE;
        }
        "--" => { /* NO FLAG [LOG_OFF] */ }
        _ => {
            eprintln!("Unknown flag");
        }
    }
    if subcommand.as_str() != "--" {
        let subscriber = FmtSubscriber::builder().with_max_level(logger).finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
    }

    match command.as_str() {
        "send" => {
            let path_buf = PathBuf::from(subcommand);

            if !path_buf.is_dir() && path_buf.is_file() {
                action = Action::Download {
                    file_path: path_buf,
                };
                info!("version {}", VERSION);
            } else {
                eprintln!("Found directory, please select a file instead!");
                std::process::exit(1);
            }
        }

        "recv" => {
            let fs = match tokio::fs::create_dir_all("uploads").await {
                Ok(_) => {
                    info!("created dir '/uploads'");
                }
                Err(err) => {
                    error!("couldn't create dir '/uploads'")
                }
            };
            action = Action::Upload;
        }
        _ => {
            eprintln!("Usage: {} <command> <path>", &args[0]);
            eprintln!("Commands: send, recv");
            std::process::exit(1);
        }
    }
    let _ = crate::server::start(action).await;
}
