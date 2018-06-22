pub use hyper::Request as Request;
pub use hyper::Response as Response;
pub use hyper::Body as Body;
use hyper::Method;
use router::Router as Router;
pub use self::handler::Handler;
use controllers::Controllers;

mod handler;

pub struct Routers {
    router: Router,
    request: Request<Body>,
    default_handler: Box<Handler>,
}

impl Routers {
    pub fn new(request: Request<Body>) -> Self {
        let mut router = Router::new();
        let controllers = Controllers::new();
        router.route(Method::GET, "/", controllers.storefront,  "index");
        router.route(Method::GET, "/admin", controllers.admin, "admin");
        router.route(Method::GET, "/static/", controllers.mount, "static");

        Routers {
            router: router,
            request: request,
            default_handler: Box::new(controllers.default),
        }
    }

    pub fn get_response(&mut self) -> Response<Body> {
        let router = self.router.recognize(self.request.method(), self.request.uri().path());
        if router.is_some() {
            router.unwrap().handle(&mut self.request)
        } else {
            self.default_handler.handle(&mut self.request)
        }
    }
}