use hyper;
use base::Handler;
use base::Request;
use base::Response;
use base::Body;
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
    fn handle(&self, _req: &mut Request<Body>) -> Response<Body> {
        let mut response = Response::new(Body::empty());
        let pathhhh = format!("{}{}", "src/admin/", _req.uri().path());
        let path = Path::new(&pathhhh[..]);

        match path.extension().and_then(OsStr::to_str) {
            Some(ext) => match ext {
                "html" => response.headers_mut().insert(hyper::header::CONTENT_TYPE, "text/plain".parse().unwrap()),
                "css" => response.headers_mut().insert(hyper::header::CONTENT_TYPE, "text/css".parse().unwrap()),
                "js" => response.headers_mut().insert(hyper::header::CONTENT_TYPE, "application/javascript".parse().unwrap()),
                _ => response.headers_mut().insert(hyper::header::CONTENT_TYPE, "text/plain".parse().unwrap()),
            },
            None => response.headers_mut().insert(hyper::header::CONTENT_TYPE, "text/plain".parse().unwrap()),
        };

        let mut f        = File::open(path).expect("error");
        let file_len     = f.metadata().unwrap().len() as usize;
        println!("file length is: {}", file_len);

        let mut contents: Vec<u8> = Vec::with_capacity(file_len + 1);

        let length = hyper::header::HeaderValue::from_str(&file_len.to_string()[..]).unwrap();
        println!("header length: {:?}", length);
        response.headers_mut().insert(hyper::header::CONTENT_LENGTH, length);

        f.read_to_end(&mut contents).expect("second error");

        println!("buffer length: {:?}", contents.capacity());

        *response.body_mut() = Body::from(contents);
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