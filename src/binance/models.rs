use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
pub struct TicketPriceData {
    pub symbol: String,
    pub price: String
}