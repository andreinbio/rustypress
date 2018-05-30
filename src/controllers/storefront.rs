use base::Handler;
use base::Request;
use base::Response;
use utils::Utils;
// use hyper::{StatusCode, header};

pub struct Index;

impl Handler for Index {
    fn handle(&self, _req: &mut Request) -> Response {
        let mut response = Response::new();
        let m_utils = Utils::new();
        let text = m_utils.get_config("config").get("server.ip").unwrap().as_str().unwrap().to_string();
        println!("{:?}", text);
        let mut body = "storefront => ".to_string();
        body.push_str(text.as_str());
        response.set_body(body);
        response
    }
}