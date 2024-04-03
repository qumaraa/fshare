mod cli;
mod server;
mod qr;
#[actix_web::main]
async fn main() {
    let _ = cli::start().await;
}