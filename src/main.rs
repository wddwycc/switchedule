// core
extern crate hyper;
extern crate futures;
// template
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

use futures::future::Future;
use hyper::server::{Http, Request, Response, Service};
use hyper::{Method, StatusCode};
use tera::Tera;
use models::{Game};


lazy_static! {
    pub static ref TERA: Tera = {
        let mut tera = compile_templates!("templates/**/*");
        tera.autoescape_on(vec!["html", ".sql"]);
//        tera.register_filter("do_nothing", do_nothing_filter);
        tera
    };
}

struct ReqHandler;

impl Service for ReqHandler {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let mut response = Response::new();
        match (req.method(), req.path()) {
            (&Method::Get, "/jp") => {
                let product = Game { name: "product name".to_string() };
                let rv = TERA.render("index.html", &product).unwrap();
                response.set_body(rv);
            }
            (&Method::Get, "/us") => {
                response.set_body("us games");
            }
            _ => {
                response.set_status(StatusCode::NotFound);
                response.set_body("404 Not Found");
            }
        };
        Box::new(futures::future::ok(response))
    }
}


fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(ReqHandler)).unwrap();
    server.run().unwrap();
}
