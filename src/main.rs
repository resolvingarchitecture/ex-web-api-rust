#![feature(proc_macro_hygiene, decl_macro)]

extern crate log;
extern crate simple_logger;
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use log::{info};
use std::path::{Path,PathBuf};

use rocket::response::NamedFile;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::JsonValue;

/// Macro can be: get, put, post, delete, head, patch, or options
#[get("/")]
fn index() -> &'static str {
    "API is up!"
}

#[get("/peer/<id>")]              // <- route attribute
fn peer(id: String) -> JsonValue {  // <- request handler
    let peer = format!("{}", id.as_str());
    json!({
        "peer": peer
    })
}

fn main() {
    simple_logger::init().unwrap();
    info!("Example Web API in Rust starting up...");
    rocket::ignite()
        .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/public")))
        .mount("/api", routes![index,peer])
        .launch();
}
