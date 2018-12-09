pub mod json_structs;
pub mod client_structs;

use ws::{connect};


pub fn fetch_instruments() {
    connect("wss://www.deribit.com/ws/api/v1/", |out| client_structs::InstrumentClient {
        out: out,
    }).expect("Error during connecting to get Intruments")
}

fn fetch_order_books(requests: Vec<String>) {
    connect("wss://www.deribit.com/ws/api/v1/", |out| client_structs::InstrumentClient {
        out: out,
    }).expect("Error during connection to get order books")
}