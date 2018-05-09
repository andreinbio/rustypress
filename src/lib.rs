extern crate hyper;
extern crate futures;

mod server;
mod router;
mod base;
mod controllers;

pub use server::Server as RustyServer;