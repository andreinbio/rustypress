use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use hyper::Method;

use recognizer::Router as Recognizer;
use recognizer::{Match, Params};

pub struct RouterInner {
    // The routers, specialized by method.
    pub routers: BTreeMap<method::Method, Recognizer<Box<Handler>>,
    // Routes that accept any method.
    pub wildcard: Recognizer<Box<Handler>>,
    // Used in url generation.
    pub route_ids: HashMap<String, String>
}