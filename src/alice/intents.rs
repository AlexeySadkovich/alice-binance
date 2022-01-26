extern crate serde;
extern crate serde_json;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Intents{
        pub exchange_rate: Option<ExchangeRateIntent>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeRateIntent {
        pub slots: ExchangeRateSlots
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeRateSlots {
        pub symbol_1: Slot,
        pub symbol_2: Slot
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Slot {
        pub type_: String,
        pub value: serde_json::Value
}

impl Slot {
        pub fn string(&self) -> String {
                let val = self.value.as_str().unwrap();

                String::from(val)
        }

        pub fn number(&self) -> i64 {
                let val = self.value.as_i64().unwrap();

                val
        } 
}
