extern crate hyper;
extern crate futures;
extern crate toml;

mod server;
mod router;
mod base;
mod controllers;
mod utils;

pub use server::Server as RustyServer;