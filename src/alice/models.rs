extern crate serde;
extern crate serde_json;

use crate::alice::intents::Intents;
use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
pub struct BackendRequest {
        pub version: String,
        pub request: Request
}

#[derive(Debug, Deserialize)]
pub struct Request {
        pub type_: String,
        pub command: String,
        pub nlu: NLU
}

#[derive(Debug, Deserialize)]
pub struct NLU {
        pub tokens: Vec<String>,
        pub intents: Intents
}

#[derive(Debug, Serialize)]
pub struct BackendResponse {
        pub version: String,
        pub response: Response
}

#[derive(Debug, Serialize)]
pub struct Response {
        pub text: String,
        pub tts: String,
        pub end_session: bool
}



