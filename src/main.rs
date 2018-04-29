extern crate hyper;
extern crate futures;

// use futures::future::Future;
// use hyper::header::ContentLength;
// use hyper::server::{Http, Request, Response, Service};
// use hyper::{Method, StatusCode};

mod server;
mod router;

fn main() {
    // // let Router = router::Router::new();
    // let ref custom_router = router::Router {
    //     routes: vec!("test".to_string())
    // };
    // let addr = "127.0.0.1:3000".parse().unwrap();
    // // let server = Http::new().bind(&addr, || Ok(router::Router{routes: vec!("test".to_string())})).unwrap();
    // let server = Http::new().bind(&addr, || Ok(router::Router {routes: vec!("test".to_string())})).unwrap();
    // server.run().unwrap();
    let socket = "3000";
    let ip = "127.0.0.1";
    let address = format!("{}:{}", ip, socket);

    println!("server running at localhost:{0}", socket);
    server::Server::new().http(&address);
}
