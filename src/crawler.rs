use std::io::{self, Write};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;
use hyper_tls::HttpsConnector;


pub fn crawl_jp() {
    let site = "https://www.nintendo.co.jp".to_string();
    let uri = format!("{}/data/schedule/xml-system/list.xml", &site).parse().unwrap();
    let mut core = Core::new().unwrap();
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &core.handle()).unwrap())
        .build(&core.handle());

    let work = client.get(uri).and_then(|res| {
        println!("Response: {}", res.status());

        res.body().for_each(|chunk| {
            io::stdout()
                .write_all(&chunk)
                .map_err(From::from)
        })
    });
    core.run(work).unwrap();
    println!("crawling nintendo jp");
}


