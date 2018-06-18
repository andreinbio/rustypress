use hyper;
use futures::future;
use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn;

use base;
pub struct RustyServer;

type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;
// impl Service for RustyService {
//     // boilerplate hooking up hyper's server types
//     type Request = Request;
//     type Response = Response;
//     type Error = hyper::Error;
//     // The future representing the eventual Response your call will
//     // resolve to. This can change to whatever Future you need.
//     type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;
//
//     fn call(&self, req: Request) -> Self::Future {
//         let mut result = base::Routers::new(req);
//         Box::new(futures::future::ok(result.get_response()))
//     }
// }

fn echo(req: Request<Body>) -> BoxFut {
    let mut result = base::Routers::new(req);
    Box::new(future::ok(result.get_response()))
}

impl RustyServer {
    pub fn new() -> Self {
        RustyServer
    }

    fn call(req: Request<Body>) -> BoxFut {
        let mut result = base::Routers::new(req);
        Box::new(future::ok(result.get_response()))
    }

    pub fn http(&self, address: &str) -> () {
        let addr = address.parse().unwrap();
        // Http::new().bind(&addr, || Ok(RustyService)).unwrap().run().unwrap();
        let server = Server::bind(&addr).serve(|| service_fn(RustyServer::call)).map_err(|e| eprintln!("server error: {}", e));
        hyper::rt::run(server)
    }
}