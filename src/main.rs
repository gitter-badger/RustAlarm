#![deny(warnings)]
extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate url;
use std::{thread,time};
//extern crate url;
//use std::env;
//use std::io::{self, Write};

//use futures::Future;
//use futures::stream::Stream;

//use hyper::{Client};

use hyper::Method;
mod http;
use http::HttpRequest;
use http::execute_request;
use url::{Url};

fn main() {

    let  request = HttpRequest {
        method:Method::Get,
        url: Url::parse("https://www.rust-lang.org/en-US/").unwrap(),
        headers:None,
        body:None
    };

    let resp = execute_request(request);
    println!("Now {:?} will print!", resp.body());
    let ten_millis = time::Duration::from_millis(10000);
    thread::sleep(ten_millis);

}