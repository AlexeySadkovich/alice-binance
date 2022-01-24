use crate::alice::models::{
    BackendRequest,
    BackendResponse,
    Response
};

use crate::binance::APIv3;

#[derive(Clone)]
pub struct Alice {
    exchange_api: APIv3
}

impl Alice {
    pub fn new(exchange_api: APIv3) -> Alice {
        Alice{ exchange_api }
    }

    pub async fn handle(&self, req: BackendRequest) -> Result<BackendResponse, String> {
        let rate = self
            .exchange_api
            .get_exchange_rate(String::from("BTCUSD"))
            .await
            .unwrap();

        let response = BackendResponse{
            version: req.version,
            response: Response{
                text: String::from(format!("Сейчас курс {}", rate)),
                tts: String::from(format!("Сейч+ас к+урс {}", rate)),
                end_session: true
            }
        };

        Ok(response)
    }
}