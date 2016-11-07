extern crate futures;
extern crate tokio_core;
extern crate tokio_tls;
extern crate curl;
// extern crate env_logger;
extern crate tokio_curl;
use std::io::{self, Write};

use futures::Future;
use tokio_core::reactor::Core;
use curl::easy::{Easy, List};
use tokio_curl::Session;
use std::collections::HashMap;
use std::collections::LinkedList;
// use std::net::ToSocketAddrs;
// use tokio_core::net::TcpStream;
// use tokio_tls::ClientContext;
extern crate tokio_minihttp as http;


enum HttpMethod {
    GET,
    POST,
    PUT,
}

struct HttpRequest {
    httpmethod: HttpMethod,
    url: String,
    headers: Option<LinkedList<String>>,
}

#[derive(Debug)]
pub struct Response {
    headers: Vec<(String, String)>,
    response: String,
    statusCode:u8
}

impl Future for Response {
    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        unimplemented!()
    }
}

fn main() {
    let method = HttpMethod::GET;
    let request = HttpRequest {
        httpmethod: method,
        url: "https://www.rust-lang.org".to_string(),
        headers: None,
    };
    let response = http_request(request);
    println!("{:?}", response);
}




fn http_request(request: Request) -> Response {


    let mut hResponse = Response {
        headers: (),
        statusCode: (),
        response: ""
    };


    let mut lp = Core::new().unwrap(); // start event loop
    let session = Session::new(lp.handle()); // session cache for multipe http requests
    let mut easy = Easy::new();
    easy.get(true).unwrap();
    easy.url(&request.url).unwrap();
    let mut tranfer = easy.transfer();
    tranfer.write_function(move |data| {
            let mut dst = Vec::new();
            dst.extend_from_slice(data);
            Ok(data.len())
        })
        .unwrap();

    let requests = session.perform(easy);
    let mut a = lp.run(requests).unwrap();

}