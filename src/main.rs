use env_logger;
use log::info;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));


pub mod greview;
pub mod storage;
pub mod data;
pub mod http;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    info!("starting greview server");
    http::run_server().await 
}
