mod cli;
mod server;
mod qr;
static VERSION: &'static str = "1.0";

#[actix_web::main]
async fn main() {
    let _ = cli::start().await;
}