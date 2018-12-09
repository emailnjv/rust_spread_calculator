extern crate serde_json;

use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct InstrumentsResponse {
    pub message: String,
    pub result: Vec<ResponseResult>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseResult {
    pub baseCurrency: String,
    pub created: String,
    pub currency: String,
    pub expiration: String,
    pub instrumentName: String,
    pub isActive: bool,
    pub kind: String,
    pub minTradeSize: f64,
    // pub optionType: String, // was throwing error with these two options not commented, and works without them idk why...
    pub pricePrecision: u128,
    pub settlement: String,
    //pub  strike: f64,
    pub tickSize: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderBookResponse {
    pub result: OrderBookResult
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderBookResult {
    pub state: String,
    pub settlementPrice: f64,
    pub instrument: String,
    pub bids: Vec<OrderPriceStruct>,
    pub asks: Vec<OrderPriceStruct>,
    pub tstamp: i64,
    pub last: i64,
    pub low: f64,
    pub high: f64,
    pub mark: f64,
    pub uPx: f64,
    pub uIx: String,
    pub iR: i64,
    pub markIv: i64,
    pub askIv: f64,
    pub bidIv: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderPriceStruct {
    pub quantity: i64,
    pub price: f64,
    pub cm: i64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiRequest {
    pub action: String,
    pub arguments: ApiArguments
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiArguments {
    pub instrument: String
}