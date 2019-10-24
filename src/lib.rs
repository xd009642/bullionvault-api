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
pub enum WeightUnit {
    Kilogram,
    TroyOunce,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Currency {
    USD,
    GBP,
    EUR,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ClassNarrative {
    Bullion(Metal),
    Money(Currency),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Location {
    Zurich,
    London,
    NewYork,
    Toronto,
    Singapore,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SecurityId(String);

impl From<(Metal, Location)> for SecurityId {
    fn from(params: (Metal, Location)) -> Self {
        let mut result = match params.0 {
            Metal::Gold => "AU",
            Metal::Silver => "AG",
            Metal::Platinum => "PT",
        }
        .to_string();

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
    AgipEnabled,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TradeType {
    OrderBoardTrade,
    OrderAtFix,
    ClientOrder,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ViewMarketRequest {
    consideration_currency: Option<Currency>,
    security_id: SecurityId,
    quantity: f64,
    market_width: usize,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ViewMarketResponse {
    buy_price: Vec<Price>,
    sell_price: Vec<Price>,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Price {
    action: ActionIndicator,
    quantity: f64,
    limit: usize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CancelOrderRequest {
    order_id: u64,
    confirmed: bool,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CancelOrderResponse {
    order_id: u64,
    client_transfer_reference: String,
    action: ActionIndicator,
    security_id: SecurityId,
    consideration_currency: Option<Currency>,
    quantity: f64,
    quantity_matched: f64,
    total_consideration: f64,
    total_commission: f64,
    limit: usize,
    order_type: OrderCode,
    order_time: DateTime<Utc>,
    good_until: Option<DateTime<Utc>>,
    last_modified: DateTime<Utc>,
    status_code: StatusCode,
    trade_type: TradeType,
    order_value: f64,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PlaceOrderRequest {
    action: ActionIndicator,
    client_transfer_ref: String,
    security_id: SecurityId,
    consideration_currency: Option<Currency>,
    quantity: f64,
    limit: usize,
    type_code: OrderCode,
    good_until: Option<DateTime<Utc>>,
    trade_type: TradeType,
    confirmed: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ViewBalanceRequest {
    simple: bool,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ViewBalanceResponse {
    client_positions: Vec<ClientPosition>,
    pending_settlements: Vec<PendingSettlement>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ClientPosition {
    security: SecurityId,
    available: f64,
    total: f64,
    narrative: ClassNarrative,
    total_valuation: f64,
    currency: Currency,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PendingSettlement {
    security: SecurityId,
    total: f64,
    narrative: ClassNarrative,
    total_valuation: f64,
    currency: Currency,
    transfers: Vec<PendingTransfer>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PendingTransfer {
    transfer_type: String,
    lowest_ledger: String,
    balance: f64,
    due_date: DateTime<Utc>,
    valuation: f64,
    currency: Currency,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ViewOrdersRequest {
    security: SecurityId,
    consideration_currency: Currency,
    status: StatusCode,
    from_data: Option<DateTime<Utc>>,
    to_date: Option<DateTime<Utc>>,
    page: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ViewOrderRequest {
    order_id: String,
    client_transfer_reference: Option<String>,
}
