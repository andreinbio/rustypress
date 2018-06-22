use base::Handler;
use base::Request;
use base::Response;
use base::Body;
use utils::Utils;
use rustyview::View;
// use hyper::{StatusCode, header};

pub struct Index {
    utils: Utils,
    template: View,
}

impl Handler for Index {
    fn handle(&self, _req: &mut Request<Body>) -> Response<Body> {
        let mut response = Response::new(Body::empty());
        let model = json!({
            "pageTitle": "Testing",
        });

        *response.body_mut() = Body::from(self.template.render("index.html", model));
        response
    }
}

impl Index {
    pub fn new(utils: Utils, template: View) -> Self {
        Index {
            utils: utils,
            template: template,
        }
    }
}