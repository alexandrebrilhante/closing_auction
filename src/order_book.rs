use crate::orders::Order;
use crate::orders::OrderSide;

use std::collections::BTreeMap;

pub struct OrderBook {
    pub bids: BTreeMap<f64, Vec<Order>>,
    pub asks: BTreeMap<f64, Vec<Order>>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        match order.side {
            OrderSide::Bid => {
                self.bids.entry(order.price).or_default().push(order);

                if let Some(vec) = self.bids.get_mut(&order.price) {
                    vec.sort_by_key(|o| o.timestamp);
                }
            }
            OrderSide::Ask => {
                self.asks.entry(order.price).or_default().push(order);

                if let Some(vec) = self.asks.get_mut(&order.price) {
                    vec.sort_by_key(|o| o.timestamp);
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.bids.clear();
        self.asks.clear();
    }

    pub fn total_bid_quantity_at_or_above(&self, price: f64) -> u64 {
        let mut total = 0;

        for (&bid_price, orders) in self.bids.range(price..) {
            total += orders.iter().map(|o| o.quantity).sum::<u64>();
        }

        total
    }

    pub fn total_ask_quantity_at_or_below(&self, price: f64) -> u64 {
        let mut total = 0;

        for (&ask_price, orders) in self.asks.range(..=price) {
            total += orders.iter().map(|o| o.quantity).sum::<u64>();
        }

        total
    }

    pub fn candidate_prices(&self) -> Vec<f64> {
        let mut candidates: Vec<f64> = self.bids.keys().cloned().collect();

        for price in self.asks.keys() {
            if !candidates.contains(price) {
                candidates.push(*price);
            }
        }

        candidates.sort_by(|a, b| a.partial_cmp(b).unwrap());

        candidates
    }
}
