//! Taken from reference https://bullionvault.com/help/xml_api.html
//!

use chrono::{DateTime, Utc};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Metal {
    Gold,
    Silver,
    Platinum,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Currency {
    USD,
    GBP,
    EUR,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Location {
    Zurich,
    London,
    NewYork,
    Toronto,
    Singapore,
}

pub struct SecurityId(String);

impl From<(Metal, Location)> for SecurityId {
    fn from(params: (Metal, Location)) -> Self {
        let mut result = match params.0 {
            Metal::Gold => "AU",
            Metal::Silver => "AG",
            Metal::Platinum => "PT",
        }.to_string();

        let location = match params.1 {
            Location::Zurich => "ZU",
            Location::London => "LN",
            Location::NewYork => "NY",
            Location::Toronto => "TR",
            Location::Singapore => "SG",
        };

        result.push_str(location);
        Self(result)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ActionIndicator {
    Buy,
    Sell,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OrderCode {
    TilCancel,
    TilTime,
    Immediate,
    FillKill,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StatusCode {
    Open,
    Done,
    Expired,
    Cancelled,
    Killed,
    NoFunds,
    BadLimit,
    SilverRestricted,
    Queued,
    AgipEnabled
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TradeType {
    OrderBoardTrade,
    OrderAtFix,
    ClientOrder,
}

pub struct ViewMarketRequest {
    consideration_currency: Option<Currency>,
    security_id: String,
    quantity: f64,
    market_width: usize,
}


pub struct ViewMarketResponse {
    buy_price: Vec<Price>,
    sell_price: Vec<Price>,
}

pub struct Price {
    action: ActionIndicator,
    quantity: f64,
    limit: usize,
}


pub struct CancelOrderRequest {
    order_id: u64,
    confirmed: bool,
}

pub struct CancelOrderResponse {
    order_id: u64,
    client_transfer_reference: String,
    action: ActionIndicator,
    security_id: String,
    consideration_currency: Option<Currency>,
    quantity: f64,
    quantity_matched: f64, 
    total_consideration: f64,
    total_commission: f64,
    limit: usize,
    type_code: OrderCode,
    order_time: DateTime<Utc>,
    good_until: Option<DateTime<Utc>>,
    last_modified: DateTime<Utc>,
    status_code: String,
    trade_type: String,
    order_value: f64,
}

pub struct PlaceOrderRequest {
    action: ActionIndicator,
    client_transfer_ref: String,
    security_id: String,
    consideration_currency: Option<Currency>,
    quantity: f64,
    limit: usize,
    type_code: OrderCode,
    good_until: Option<DateTime<Utc>>,
    trade_type: String,
    confirmed: bool,
}
