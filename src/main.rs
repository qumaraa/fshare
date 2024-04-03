mod cli;
mod server;

#[actix_web::main]
async fn main() {
    let _ = cli::start().await;
}

