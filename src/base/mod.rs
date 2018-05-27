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
    default_handler: Box<Handler>,
}

impl Routers {
    pub fn new(request: Request) -> Self {
        let mut router = Router::new();
        let controllers = Controllers::new();
        router.route(Method::Get, "/", controllers.storefront,  "index");
        router.route(Method::Get, "/admin", controllers.admin, "admin");

        Routers {
            router: router,
            request: request,
            default_handler: Box::new(controllers.default),
        }
    }

    pub fn get_response(&mut self) -> Response {
        let router = self.router.recognize(self.request.method(), self.request.path());
        if router.is_some() {
            router.unwrap().handle(&mut self.request)
        } else {
            self.default_handler.handle(&mut self.request)
        }
    }
}