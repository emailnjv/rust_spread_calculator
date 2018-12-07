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
    // pub optionType: String,
    pub pricePrecision: u128,
    pub settlement: String,
    //pub  strike: f64,
    pub tickSize: f64,
}