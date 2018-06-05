use base::Handler;
use base::Request;
use base::Response;
// use hyper::{StatusCode, header};

use utils::Utils;
use rustyview::View;

pub struct Index {
    utils: Utils,
    template: View,
}

impl Handler for Index {
    fn handle(&self, _req: &mut Request) -> Response {
        let mut response = Response::new();

        // if true {
        //     response.set_status(StatusCode::Forbidden);
        //     response.headers_mut().set(header::Location::new("/lala"));
        //     return response;
        // }

        response.set_body("admin test body".to_string());
        response
    }
}

impl Index {
    pub fn new(utils: Utils, admin_template: View) -> Index {
        Index {
            utils: utils,
            template: admin_template,
        }
    }
}