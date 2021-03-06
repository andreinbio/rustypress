use base::Request;
use base::Response;

pub trait Handler: Send + Sync + 'static {
    fn handle(&self, request: &mut Request) -> Response;
}

// impl<F> Handler for F
// where F: Send + Sync + 'static + Fn(&mut Request) -> Response {
//     fn handle(&self, req: &mut Request) -> Response {
//         (*self)(req)
//     }
// }
//
// impl Handler for Box<Handler> {
//     fn handle(&self, req: &mut Request) -> Response {
//         (**self).handle(req)
//     }
// }