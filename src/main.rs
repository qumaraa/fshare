mod cli;
mod qr;
mod recv;
mod server;

#[actix_web::main] // actix-web runtime
async fn main() {
    // starting cli
    let _ = cli::start().await;
}
