use base::Handler;
use base::Request;
use base::Response;

pub struct Index;

impl Handler for Index {
    fn handle(&self, req: &mut Request) -> Response {
        let mut response = Response::new();
        response.set_body("default body, 404 maybe ?".to_string());
        response
    }
}