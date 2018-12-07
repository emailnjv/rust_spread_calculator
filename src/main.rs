extern crate env_logger;
extern crate ws;
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use ws::{connect, CloseCode, Handler, Handshake, Message, Result, Sender};
use serde_json::{Value};
mod lib;
// Our Handler struct.
// Here we explicity indicate that the Client needs a Sender,
// whereas a closure captures the Sender for us automatically.
struct Client {
    out: Sender,
}

// We implement the Handler trait for Client so that we can get more
// fine-grained control of the connection.
impl Handler for Client {
    // `on_open` will be called only after the WebSocket handshake is successful
    // so at this point we know that the connection is ready to send/receive messages.
    // We ignore the `Handshake` for now, but you could also use this method to setup
    // Handler state or reject the connection based on the details of the Request
    // or Response, such as by checking cookies or Auth headers.
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        // Now we don't need to call unwrap since `on_open` returns a `Result<()>`.
        // If this call fails, it will only result in this connection disconnecting.
        self.out
            .send(r#"{"action": "/api/v1/public/getinstruments"}"#)
    }

    // `on_message` is roughly equivalent to the Handler closure. It takes a `Message`
    // and returns a `Result<()>`.
    fn on_message(&mut self, msg: Message) -> Result<()> {
        // Close the connection when we get a response from the server
        parse_instruments(msg);
        self.out.close(CloseCode::Normal)
    }


}

fn main() {
    fetch_instruments();
}

fn fetch_instruments() {
    connect("wss://www.deribit.com/ws/api/v1/", |out| Client {
        out: out,
    }).unwrap()
}

fn parse_instruments(msg: Message) {
    let text_version = msg.as_text().unwrap();
    let v: lib::InstrumentsResponse = serde_json::from_str(text_version).expect("@@@@@@@@@@@@@@@@@@@@@@@@@@");
    println!("{:?}", v.result[0].instrumentName);
    // for x in v["result"].iter() {
    //     println!("{:?}", x["instrumentName"])
    // }
}