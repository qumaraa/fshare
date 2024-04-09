mod cli;
mod qr;
mod recv;
mod server;
//mod recv;

static VERSION: &'static str = "2.0";

#[actix_web::main]
async fn main() {
    let _ = cli::start().await;
}
