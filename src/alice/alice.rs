use crate::alice::{
    intents::{
        ExchangeRateIntent
    },
    models::{
        BackendRequest,
        BackendResponse,
        Response,
    }};

use crate::binance::APIv3;

#[derive(Clone)]
pub struct Alice {
    exchange_api: APIv3,
}

impl Alice {
    pub fn new(exchange_api: APIv3) -> Alice {
        Alice { exchange_api }
    }

    pub async fn handle(&self, req: BackendRequest) -> Result<BackendResponse, String> {
        if req.request.nlu.intents.exchange_rate.is_some() {
            let r = self
                .handle_exchange_rate_intent(req.request.nlu.intents.exchange_rate.unwrap())
                .await;

            let response = BackendResponse {
                version: req.version,
                response: r,
            };

           return Ok(response)
        }

        let response = BackendResponse {
            version: req.version.to_owned(),
            response: Response {
                text: String::from("Не знаю, что ответить"),
                tts: String::from("Не зн+аю что отв+етить"),
                end_session: false,
            }
        };

        Ok(response)
    }

    async fn handle_exchange_rate_intent(&self, intent: ExchangeRateIntent) -> Response {
        let symbol = format!(
            "{}{}",
            intent.slots.symbol_1.string(),
            intent.slots.symbol_2.string()
        );

        let rate = self
            .exchange_api
            .get_exchange_rate(symbol)
            .await
            .unwrap();

        Response {
            text: String::from(format!("Сейчас курс {}", rate)),
            tts: String::from(format!("Сейч+ас к+урс {}", rate)),
            end_session: false,
        }
    }
}