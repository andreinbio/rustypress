//extern crate router_recognizer as recognizer;

//pub use routerdata::{Router, NoRoute, TrailingSlash};
//pub use recognizer::Params;
mod routerdata;
mod recognizer;
use self::recognizer::Recognizer;
pub use self::routerdata::Router;
