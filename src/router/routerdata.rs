use std::collections::HashMap;
use std::collections::BTreeMap;
// use std::error::Error;
// use std::fmt;
use std::sync::Arc;
use hyper;

pub use base::Handler as Handler;

struct RouterInner {
    // The routers, specialized by method.
    pub routers: HashMap<hyper::Method, BTreeMap<String, Box<Handler>>>,

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

    pub fn route<S: AsRef<str>, H: Handler, I: AsRef<str>>(&mut self, method: hyper::Method, glob: S, handler: H, route_id: I) -> &mut Router {
        self.mut_inner().routers
        .entry(method)
        .or_insert(BTreeMap::new())
        .insert(glob.as_ref().to_string(), Box::new(handler));
        self.mut_inner().route_ids.insert(route_id.as_ref().to_string(), glob.as_ref().to_string());
        self
    }

    pub fn recognize(&self, method: &hyper::Method, path: &str) -> Option<&Box<Handler>> {
        let found_method = self.inner.routers.get(method);

        if found_method.is_some() {
            let found_handler = found_method.unwrap().get(path);

            if found_handler.is_some() {
                return found_handler;
            }
        }

        None
    }
}