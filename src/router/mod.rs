//extern crate router_recognizer as recognizer;

//pub use routerdata::{Router, NoRoute, TrailingSlash};
//pub use recognizer::Params;
use hyper;
use hyper::Method;
mod routerdata;
pub use self::routerdata::Router;

pub struct RouteResult;

impl RouteResult {
    pub fn new(req: hyper::server::Request) -> Self {
        let mut router = Router::new();
        router.route(Method::Get, "test", "index");
        RouteResult
    }

    pub fn get_response(&self) -> String {
        String::from("Hello")
    }
}