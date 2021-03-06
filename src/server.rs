use futures::{self};
use futures::future::Future;
use hyper::{self};
use hyper::server::{Request, Response, Service};
use hyper::{Method, StatusCode};
use tera::Tera;

use models::{Area, Game};


lazy_static! {
    static ref TERA: Tera = {
        let mut tera = compile_templates!("templates/**/*");
        tera.autoescape_on(vec!["html", ".sql"]);
        tera
    };
}


pub struct Nintendo;


impl Service for Nintendo {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let mut response = Response::new();
        match (req.method(), req.path()) {
            (&Method::Get, path) => {
                // todo: serve crawled data instead
                let data = Area {
                    name: format!("{}", path),
                    games: vec![
                        Game { name: "Mario Odyssey".to_string() },
                    ],
                };
                match TERA.render("index.html", &data) {
                    Ok(html) => { response.set_body(html); }
                    Err(_) => { response.set_status(StatusCode::InternalServerError); }
                };
            }
            _ => {
                response.set_status(StatusCode::NotFound);
                response.set_body("404 Not Found");
            }
        };
        Box::new(futures::future::ok(response))
    }
}
