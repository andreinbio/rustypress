use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::sync::Arc;
use hyper;

// use hyper::Method;

//use recognizer::Router as Recognizer;
// use recognizer::{Match, Params};

struct RouterInner {
    // The routers, specialized by method.
    // pub routers: BTreeMap<method::Method, Recognizer<Box<Handler>>,
    pub routers: HashMap<hyper::Method, HashMap<String, String>>,

    // Routes that accept any method.
    //pub wildcard: Recognizer<Box<Handler>>,
    // Used in url generation.
    pub route_ids: HashMap<String, String>
}

pub struct Router {
    inner: Arc<RouterInner>
}

impl Router {
    pub fn new() -> Self {
        Router {
            inner: Arc::new(RouterInner {
                routers: HashMap::new(),
                route_ids: HashMap::new()
            })
        }
    }

    fn mut_inner(&mut self) -> &mut RouterInner {
        Arc::get_mut(&mut self.inner).expect("Cannot modify router at this point.")
    }

    pub fn route<S: AsRef<str>, I: AsRef<str>>(&mut self, method: hyper::Method, glob: S, route_id: I) -> &mut Router {
        self.mut_inner().routers
        .entry(method)
        .or_insert(HashMap::new())
        .insert(glob.as_ref().to_string(), route_id.as_ref().to_string());
        self.mut_inner().route_ids.insert(route_id.as_ref().to_string(), glob.as_ref().to_string());
        self
    }
}