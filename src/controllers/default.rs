use base::Handler;
use base::Request;
use base::Response;

use utils::Utils;
use rustyview::View;

pub struct Index {
    utils: Utils,
    template: View,
}

impl Handler for Index {
    fn handle(&self, _req: &mut Request) -> Response {
        let mut response = Response::new();
        response.set_body("default body, 404 maybe ?".to_string());
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