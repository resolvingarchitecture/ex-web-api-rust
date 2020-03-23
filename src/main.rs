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
    let id = format!("{}", id.as_str());
    json!({
        "id": id,
        "name": "Matt"
    })
}

/// Search the contact database by first and last name (last being optional)
#[get("/contact/search?<first>&<last>")]
fn contact_search(first: String, last: Option<String>) -> JsonValue {
    let first_name = format!("{}", first.as_str());
    let last_name = last.map(|last| format!("{}", last))
        .unwrap_or_else(|| "_".into());
    if String::from("_").eq(&last_name) {
        json!({
            "first": first_name
        })
    } else {
        json!({
            "first": first_name,
            "last": last_name
        })
    }
}

fn main() {
    simple_logger::init().unwrap();
    info!("Example Web API in Rust starting up...");
    rocket::ignite()
        .mount("/", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/public")))
        .mount("/api", routes![index,peer,contact_search])
        .launch();
}
