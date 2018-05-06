extern crate hyper;
extern crate futures;

mod server;
mod router;
mod base;

pub use server::Server as RustyServer;