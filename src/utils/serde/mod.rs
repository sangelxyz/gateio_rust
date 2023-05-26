use serde::{Deserialize, Serialize};
// mod utils {
//     pub mod gateio;
//     pub mod lib;
//     pub mod serde;
// }
// Struts
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Candles {
    // #[serde(deserialize_with = "parse_string_to_i32")]
    pub time: u32,
    // #[serde(deserialize_with = "parse_string_to_f64")]
    pub open: f64,
    // #[serde(deserialize_with = "parse_string_to_f64")]
    pub high: f64,
    // #[serde(deserialize_with = "parse_string_to_f64")]
    pub low: f64,
    // #[serde(deserialize_with = "parse_string_to_f64")]
    pub close: f64,
    // #[serde(deserialize_with = "parse_string_to_f64")]
    pub volume: f64,
    // #[serde(deserialize_with = "parse_string_to_f64")]
    pub volumes: f64,
}

// #[allow(non_snake_case)]
// #[derive(Deserialize, Serialize, Debug)]
// pub struct CurrencyPairs {
//     pub id: String,
//     pub base: String,
//     pub quote: String,
//     pub fee: f64,
//     pub min_base_amount: f64,
//     pub min_quote_amount: f64,
//     pub amount_precision: u8,
//     pub precision: u8,
//     pub trade_status: String,
//     pub sell_start: u32,
//     pub buy_start: u32,
// }

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Products {
    pub currency: String,
    pub delisted: String,
    pub withdraw_disabled: bool,
    pub withdraw_delayed: bool,
    pub deposit_disabled: bool,
    pub trade_disabled: bool,
    pub chain: String,
}

// Serde Value as u32 output.
use serde_json::Value;
pub fn as_u32(item: &Value) -> u32 {
    match item {
        Value::String(contents) => match contents.parse::<u32>() {
            Ok(value) => value,
            Err(_) => {
                println!("Failed to parse string as i32");
                0
            }
        },
        _ => 0,
    }
}

// Serde Value as bool.
pub fn as_bool(item: &Value) -> bool {
    match item {
        Value::String(contents) => {
            if contents == "true" {
                true
            } else {
                false
            }
        }
        _ => false,
    }
}

// Serde Value as String.
pub fn as_string(item: &Value) -> String {
    match item {
        Value::String(contents) => contents.to_string(),
        _ => "".to_string(),
    }
}

pub fn as_f64(item: &Value) -> f64 {
    match item {
        Value::String(contents) => match contents.parse::<f64>() {
            Ok(value) => value,
            Err(_) => {
                println!("Failed to parse string as i32");
                0.0
            }
        },
        _ => 0.0,
    }
}
