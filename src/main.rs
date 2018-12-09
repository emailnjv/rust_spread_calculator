extern crate env_logger;
extern crate ws;
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate serde_derive;

mod lib;

fn main() {
    lib::fetch_instruments();
}

