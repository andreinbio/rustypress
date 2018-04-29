extern crate router_recognizer as recognizer;

pub use routerdata::{Router, NoRoute, TrailingSlash};
pub use recognizer::Params;

mod routerdata;