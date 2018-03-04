use futures;
use futures::future::Future;
use hyper;
use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};
use hyper::{Method, StatusCode};

pub struct Router {
    pub routes: Vec<String>
}

impl Service for Router {
    // boilerplate hooking up hyper's server types
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        println!("first rout is: {0}", self.routes.len());
        let mut response = Response::new();
        response.set_body(req.path().to_string());
        Box::new(futures::future::ok(response))
    }
}

impl Router {
    pub fn new() -> Router {
        Router {
            routes: vec!("test".to_string(), "newtest".to_string())
        }
    }
}