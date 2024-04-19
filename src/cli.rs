use crate::server;
use crate::server::Action;
use std::env::args;
use std::path::PathBuf;
use tracing;
use tracing::{
    debug,
    error,
    info,
    trace,
    warn,
    Level
};
use tracing_subscriber::FmtSubscriber;
use clap::{Arg, Parser, Subcommand};

#[derive(Parser)]
#[command(name="fshare",version="2.1",about,long_about=None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    Send {
        #[arg(short,long,help="Path of file for sending to server for downloading.")]
        path: String,
        #[arg(short,long,help="Choose log-level. Level::DEBUG set as default.")]
        log: String,
    },
    Recv {
        #[arg(short,long,help="Choose log-level. Level::DEBUG set as default.")]
        log: String,
    },
}

fn handle_log(log: String) -> tracing::Level {
    match log.as_str() {
        "log_info"  =>  tracing::Level::INFO ,
        "log_debug" =>  tracing::Level::DEBUG,
        "log_warn"  =>  tracing::Level::WARN ,
        "log_trace" =>  tracing::Level::TRACE,
        "log_err"   =>  tracing::Level::ERROR,
        _           =>  tracing::Level::DEBUG,
    }
}

pub async fn start() {
    // logger init default as Level::DEBUG
    let mut logger = tracing::Level::DEBUG;
    // collects all command-line arguments
    let args: Vec<String> = args().collect();
    // action init
    let mut action = Action::Download {
        file_path: PathBuf::new(),
    };
    let argz = Args::parse();

    match argz.command {
        Commands::Send { path,log} => {
            let f = handle_log(log);
            logger = f;
            let path_buf = PathBuf::from(path);

            if !path_buf.is_dir() && path_buf.is_file() {
                action = Action::Download {
                    file_path: path_buf,
                };
            } else {
                eprintln!("Found directory, please select a file instead!");
                std::process::exit(1);
            }
        }
        Commands::Recv {log} => {
            let f = handle_log(log);
            logger = f;
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
    }

    let subscriber = FmtSubscriber::builder()
        .with_max_level(logger)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    // start server with action (asynchronously)
    let _ = crate::server::start(action).await;
}
