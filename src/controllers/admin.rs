use base::Handler;
use base::Request;
use base::Response;

pub struct Index;

impl Handlers for Index {
    pub fn handle(&self, req: &mut Request) -> Response {
        let mut response = Response::new();
        //response.
    }
}