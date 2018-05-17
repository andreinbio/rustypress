use hyper;
pub use hyper::server::Request as Request;
pub use hyper::server::Response as Response;
use hyper::Method;
use router::Router as Router;
pub use self::handler::Handler;
use controllers::Controllers;

mod handler;

struct DefaultHandler;

impl Handler for DefaultHandler {
    fn handle(&self, req: &mut Request) -> Response {
        let mut response = Response::new();
        response.set_body("default handler".to_string());
        response
    }
}

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

    pub fn get_response(&mut self) -> Response {
        let router = self.router.recognize(self.request.method(), self.request.path());
        if (router.is_some()) {
            router.unwrap().handle(&mut self.request)
        } else {
            DefaultHandler.handle(&mut self.request)
        }
    }
}