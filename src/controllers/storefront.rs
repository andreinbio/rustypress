use base::Handler;
use base::Request;
use base::Response;
// use hyper::{StatusCode, header};

pub struct Index;

impl Handler for Index {
    fn handle(&self, _req: &mut Request) -> Response {
        let mut response = Response::new();

        response.set_body("storefront here...".to_string());
        response
    }
}