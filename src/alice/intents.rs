extern crate serde;
extern crate serde_json;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Intents{
        exchange_rate: ExchangeRateIntent
}

#[derive(Debug, Serialize, Deserialize)]
struct ExchangeRateIntent {
        slots: ExchangeRateSlots
}

#[derive(Debug, Serialize, Deserialize)]
struct ExchangeRateSlots {
        currency: Slot 
}

#[derive(Debug, Serialize, Deserialize)]
struct Slot {
        type_: String,
        value: serde_json::Value
}

impl Slot {
        fn string(&self) -> String {
                let val = self.value.as_str().unwrap();

                String::from(val)
        }

        fn number(&self) -> i64 {
                let val = self.value.as_i64().unwrap();

                val
        } 
}
