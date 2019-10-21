//! Taken from reference https://bullionvault.com/help/xml_api.html
//!


#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Currency {
    USD,
    GBP,
    EUR,
}

pub struct ViewMarketRequest {
    consideration_currency: Option<Currency>,
    security_id: String,
    quantity: f64,
    market_width: usize,
}


pub struct ViewMarketResponse {

}


pub struct CancelOrder {
    order_id: u64,
    confirmed: bool,
}
