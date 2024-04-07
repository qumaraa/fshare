use crate::server;
use crate::server::Action;
use std::env::args;
use std::path::PathBuf;
use crate::VERSION;


pub async fn start() {
    let args: Vec<String> = args().collect();
    let mut action = Action::Download {
        file_path: PathBuf::new(),
    };
    if args.len() < 3 {
        eprintln!("Usage: {} <command> <path>", &args[0]);
        std::process::exit(1);
    }

    let command = &args[1];
    let path = &args[2];
    match command.as_str() {
        "send" => {
            let path_buf = PathBuf::from(path);
            // CH:1.1
            if path_buf.is_dir() && !path_buf.is_file() {
                println!("Version: {}",VERSION);
                action = Action::Download {
                    file_path: path_buf,
                };
            }else {
                eprintln!("Error: please select a file instead!");
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("Usage: {} <command> <path>", &args[0]);
            std::process::exit(1);
        }
    }
    let _ = crate::server::start(action).await;
}
