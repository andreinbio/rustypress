use base::Handler;
use base::Request;
use base::Response;
use hyper::{StatusCode, header};

pub struct Index;

impl Handler for Index {
    fn handle(&self, _req: &mut Request) -> Response {
        let mut response = Response::new();

        if true {
            response.set_status(StatusCode::Found);
            response.headers_mut().set(header::Location::new("/lala"));
            return response;
        }

        response.set_body("admin test body".to_string());
        response
    }
}