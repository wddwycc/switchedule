extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate futures;
#[macro_use]
extern crate tera;
// serde
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
// vendor
#[macro_use]
extern crate lazy_static;

mod models;
mod server;
mod crawler;

use std::env;
use hyper::server::Http;


fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd = &args[1];
    match cmd.as_ref() {
        "server" => {
            let addr = "127.0.0.1:3000".parse().unwrap();
            let server = Http::new().bind(&addr, || Ok(server::Nintendo)).unwrap();
            server.run().unwrap();
        }
        "crawler" => {
            crawler::crawl_jp();
        }
        _ => {
            println!("Command not supported")
        }
    }
}
