use hyper;
pub use hyper::server::Request as Request;
pub use hyper::server::Response as Response;
use hyper::Method;
use router::Router as Router;
pub use self::handler::Handler;
use controllers::Controllers;

mod handler;

pub struct Routers {
    router: Router,
    request: Request,
    response: Response,
}

impl Routers {
    pub fn new(request: Request, response: Response) -> Self {
        let mut router = Router::new();
        let controllers = Controllers::new();
        // router.route(Method::Get, "/test", "test page");
        router.route(Method::Get, "/", controllers.admin,  "index");

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