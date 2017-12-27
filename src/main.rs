extern crate dotenv;
#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate hyper;

use dotenv::dotenv;
use futures::future::Future;
use hyper::{StatusCode};
use hyper::header::{ContentLength, ContentType, Host, Location};
use hyper::server::{Http, Request, Response, Service};
use std::env;
use std::net::SocketAddr;
use std::str;

error_chain! {
    foreign_links {
        AddrParse(std::net::AddrParseError);
        Hyper(hyper::Error);
        Utf8(std::str::Utf8Error);
    }
}

struct Not80;

impl Not80 {
    fn location_from_request(&self, request: &Request) -> String {
        let host = request.headers().get::<Host>()
            .expect("Host was not included.");
        let query = match request.query() {
            Some(q) => format!("?{}", q),
            None => "".to_string()
        };

        format!("https://{}{}{}", host, request.path(), query)
    }

    fn content_from_location(&self, location: &String) -> String {
        format!("<!doctype>\n<html><body><a href=\"{}\">{}</a></body></html>\n", location, location)
    }

    fn location_and_content(&self, request: &Request) -> (String, String) {
        let location = self.location_from_request(&request);
        let content = self.content_from_location(&location);

        (location, content)
    }
}

impl Service for Not80 {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, request: Request) -> Self::Future {
        let (location, content) = self.location_and_content(&request);
        let response = Response::new()
            .with_status(StatusCode::Found)
            .with_header(Location::new(location))
            .with_header(ContentLength(content.len() as u64))
            .with_header(ContentType::html())
            .with_body(content);

        Box::new(futures::future::ok(response))
    }
}

fn http_server(listen: &String) -> Result<()> {
    let address: SocketAddr = listen.parse()?;
    let server = Http::new().bind(&address, || Ok(Not80))?;

    server.run()?;

    Ok(())
}

fn run() -> Result<()> {
    dotenv().ok();

    let listen: String = match env::var("LISTEN") {
        Ok(value) => value,
        Err(_) => panic!("Environment variable LISTEN must be defined."),
    };

    println!("{}", listen);

    http_server(&listen)?;

    Ok(())
}

quick_main!(run);