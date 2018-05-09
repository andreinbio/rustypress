use base::Handler;
use base::Request;
use base::Response;

pub struct Index;

impl Handler for Index {
    fn handle(&self, req: &mut Request) -> Response {
        let mut response = Response::new();
        response.set_body("admin test body".to_string());
        response
    }
}