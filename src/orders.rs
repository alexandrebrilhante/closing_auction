use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum OrderType {
    MOC,
    LOC,
    IO,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum OrderSide {
    Bid,
    Ask,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Order {
    pub id: u64,
    pub price: OrderedFloat<f64>,
    pub quantity: u64,
    pub timestamp: u64,
    pub order_type: OrderType,
    pub side: OrderSide,
}
