extern crate tokio_core;
extern crate url;

use hyper::{Client};
use hyper::Method;
//use hyper::client::FutureResponse;
use hyper::header::{Headers};


use url::Url;
use hyper::client::Response;

pub struct HttpRequest {
    pub method: Method,
    pub url: Url,
    pub headers: Option<Headers>,
    pub body:Option<String>
}

pub fn execute_request(http_request:HttpRequest) -> Response {
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();

    let client = Client::new(&handle).unwrap();
    //let mut request;
    let  work;
    //if httpRequest.method == Method::Get {
        work  =  client.get(http_request.url);
    //}
    /*
    let work = client.request()
        .and_then(|res| {
            println!("Response: {}", res.status());
            println!("Headers: \n{}", res.headers());
            res.body().for_each(|chunk| io::stdout().write_all(&chunk).map_err(From::from))
        })
        .map(|_| {
            println!("\n\nDone.");
        });
    */
     core.run(work).unwrap()
}