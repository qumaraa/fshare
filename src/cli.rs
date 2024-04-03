use crate::server;
use std::env::args;
use std::path::PathBuf;
use crate::server::Action;


pub async fn start() {
    let args: Vec<String> = args().collect();
    let mut action = Action::Download { file_path: PathBuf::new() };
    if args.len() < 3 {
        eprintln!("Usage: {} <command> <path>", &args[0]);
        return;
    }

    let command = &args[1];
    let path = &args[2];
    match command.as_str() {
        "send" => {
            let path_buf = PathBuf::from(path);
            action = Action::Download { file_path: path_buf };
        }
        _ => {

        }

    }
    let _ = crate::server::start(action).await;
}






