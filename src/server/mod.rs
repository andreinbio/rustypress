use hyper;
use futures::future;
use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn;

use base;
pub struct RustyServer;

type BoxFut = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

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
        let server = Server::bind(&addr).serve(|| service_fn(RustyServer::call)).map_err(|e| eprintln!("server error: {}", e));
        hyper::rt::run(server)
    }
}