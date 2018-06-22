use base::Handler;
use base::Request;
use base::Response;
use base::Body;

use utils::Utils;
use rustyview::View;

pub struct Index {
    utils: Utils,
    template: View,
}

impl Handler for Index {
    fn handle(&self, _req: &mut Request<Body>) -> Response<Body> {
        let mut response = Response::new(Body::empty());
        *response.body_mut() = Body::from("default body, 404 maybe ?");
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