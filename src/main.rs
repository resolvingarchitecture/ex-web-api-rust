extern crate log;
extern crate simple_logger;

use log::{info};

fn main() {
    simple_logger::init().unwrap();
    info!("Hello, world!");
}
