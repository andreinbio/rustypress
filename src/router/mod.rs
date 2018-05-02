//extern crate router_recognizer as recognizer;

//pub use routerdata::{Router, NoRoute, TrailingSlash};
//pub use recognizer::Params;
use hyper;
use hyper::Method;
mod routerdata;
pub use self::routerdata::Router;

pub struct RouteResult {
    router: Router,
    req: hyper::server::Request,
}

impl RouteResult {
    pub fn new(req: hyper::server::Request) -> Self {
        let mut router = Router::new();
        router.route(Method::Get, "/test", "test page");
        router.route(Method::Get, "/", "index");
        
        RouteResult {
            router: router,
            req: req,
        }
    }

    pub fn get_response(&self) -> String {
        let router = self.router.recognize(self.req.method(), self.req.path());
        router.unwrap()
    }
}