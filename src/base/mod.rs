use hyper;
use hyper::server::Request as Request;
use hyper::server::Response as Response;
use hyper::Method;
use router::Router as Router;
pub use self::handler::Handler;

mod handler;

pub struct Routers {
    router: Router,
    request: Request,
    response: Response,
}

impl Routers {
    pub fn new(request: Request, response: Response) -> Self {
        let mut router = Router::new();
        router.route(Method::Get, "/test", "test page");
        router.route(Method::Get, "/", "index");

        Routers {
            router: router,
            request: request,
            response: response
        }
    }

    pub fn get_response(mut self) -> Response {
        let router = self.router.recognize(self.request.method(), self.request.path());
        self.response.set_body(router.unwrap());
        self.response
    }
}