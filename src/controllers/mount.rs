use base::Handler;
use base::Request;
use base::Response;
use hyper::{StatusCode, header};
use std::path::{PathBuf, Path};
use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;

use utils::Utils;
use rustyview::View;

pub struct Index {
    utils: Utils,
    template: View,
}

impl Handler for Index {
    fn handle(&self, _req: &mut Request) -> Response {
        let mut response = Response::new();
        let pathhhh = format!("{}{}", "src/admin/", _req.path());
        let path = Path::new(&pathhhh[..]);

        match path.extension().and_then(OsStr::to_str) {
            Some(ext) => match ext {
                "html" => response.headers_mut().set(header::ContentType::html()),
                "css" => response.headers_mut().set_raw("Content-Type", "text/css"),
                "js" => response.headers_mut().set_raw("Content-Type", "application/javascript"),
                _ => response.headers_mut().set(header::ContentType::text()),
            },
            None => response.headers_mut().set(header::ContentType::text()),
        }

        let mut f = File::open(path).expect("error");
        let mut contents = Vec::new();
        f.read_to_end(&mut contents).expect("second error");
        println!("testtingggg");

        response.set_body(contents);
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