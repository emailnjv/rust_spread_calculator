use ws::{connect, CloseCode, Handler, Handshake, Message, Result, Sender};
use lib;



pub struct InstrumentClient {
    pub out: Sender,
}
impl Handler for InstrumentClient {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        self.out
            .send(r#"{"action": "/api/v1/public/getinstruments"}"#)
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("{:?}", msg);
        let orderbook_request_vector = parse_instruments(msg);
        for orderbook_request in orderbook_request_vector.iter() {
            // print!("{:?}", orderbook_request)
        }
        self.out.close(CloseCode::Normal)
    }

    
}
fn parse_instruments(msg: Message) -> Vec<String> {
    let text_version = msg.as_text().unwrap();
    let instrument_response: lib::json_structs::InstrumentsResponse = serde_json::from_str(text_version).expect("@@@@@@@@@@@@@@@@@@@@@@@@@@");
    let mut request_vector = Vec::new();
    for x in instrument_response.result.iter() {
        request_vector.push(fetch_order_book_request(&x.instrumentName))
    }
    request_vector
}

fn fetch_order_book_request(instrument: &String) -> String{
    let orderbook_request = lib::json_structs::ApiRequest{
        action: "/api/v1/public/getorderbook".to_owned(),
        arguments: lib::json_structs::ApiArguments {
            instrument: instrument.clone()
        }
    };
    serde_json::to_string(&orderbook_request).expect("Error converting request into JSON")
}


