extern crate log;

use log::{
    info
};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("starting up...");

    println!("Hello, world!");


    info!("exiting");
    return Ok(());
}
