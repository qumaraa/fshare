mod cli;
mod qr;
mod recv;
mod server;

static VERSION: &'static str = "2.0";

#[actix_web::main] // actix-web runtime
async fn main() {
    // starting cli
    let _ = cli::start().await;
}
