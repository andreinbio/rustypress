use base::Handler;
use base::Request;
use base::Response;
use utils::Utils;
use rustyview::View;
// use hyper::{StatusCode, header};

pub struct Index {
    utils: Utils,
    template: View,
}

impl Handler for Index {
    fn handle(&self, _req: &mut Request) -> Response {
        let mut response = Response::new();
        let model = json!({
            "pageTitle": "Testing",
        });

        response.set_body(self.template.render("index.html", model));
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