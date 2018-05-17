use futures;
use hyper;
use futures::future::Future;
use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};
use hyper::{Method, StatusCode};

use base;
struct RustyService;
pub struct Server;

impl Service for RustyService {
    // boilerplate hooking up hyper's server types
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let response = Response::new();
        let mut result = base::Routers::new(req, response);
        Box::new(futures::future::ok(result.get_response()))
    }
}

impl Server {
    pub fn new() -> Self {
        Server
    }

    pub fn http(&self, address: &str) -> () {
        let addr = address.parse().unwrap();
        Http::new().bind(&addr, || Ok(RustyService)).unwrap().run().unwrap();
    }
}