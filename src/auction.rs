use crate::imbalance::ImbalanceHandler;
use crate::order_book::OrderBook;

use log::{info, warn};
use std::cmp::Ordering;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AuctionSystem {
    pub last_clearing_price: Option<f64>,
}

impl AuctionSystem {
    pub fn new() -> Self {
        Self {
            last_clearing_price: None,
        }
    }

    pub async fn calculate_auction_price(
        &self,
        order_book: Arc<RwLock<OrderBook>>,
        _imbalance_handler: Arc<RwLock<ImbalanceHandler>>,
    ) -> Option<f64> {
        let ob = order_book.read().await;
        let candidates = ob.candidate_prices();

        let mut best_price = None;
        let mut best_volume = 0;
        let mut best_imbalance = u64::MAX;

        for price in candidates {
            let bid_qty = ob.total_bid_quantity_at_or_above(price);
            let ask_qty = ob.total_ask_quantity_at_or_below(price);
            let executable_volume = bid_qty.min(ask_qty);

            let imbalance = if bid_qty > ask_qty {
                bid_qty - ask_qty
            } else {
                ask_qty - bid_qty
            };

            if executable_volume > Some(best_volume)
                || (executable_volume == Some(best_volume) && imbalance < best_imbalance)
            {
                best_volume = executable_volume;
                best_imbalance = imbalance;
                best_price = Some(price);
            }
        }

        best_price
    }

    pub async fn execute_auction(
        &mut self,
        order_book: Arc<RwLock<OrderBook>>,
        imbalance_handler: Arc<RwLock<ImbalanceHandler>>,
    ) {
        let clearing_price_option = self
            .calculate_auction_price(order_book.clone(), imbalance_handler)
            .await;

        match clearing_price_option {
            Some(price) => {
                self.last_clearing_price = Some(price);

                let ob = order_book.read().await;
                let bid_qty = ob.total_bid_quantity_at_or_above(price);
                let ask_qty = ob.total_ask_quantity_at_or_below(price);
                let executed_qty = bid_qty.min(ask_qty);

                info!(
                    "Auction executed at price: {} with quantity: {}",
                    price, executed_qty
                );
            }
            None => {
                warn!("No clearing price could be determined; auction did not execute.");
            }
        }
    }
}

pub async fn start_auction_system(
    order_book: Arc<RwLock<OrderBook>>,
    auction_system: Arc<RwLock<AuctionSystem>>,
    imbalance_handler: Arc<RwLock<ImbalanceHandler>>,
) {
    {
        let mut auction = auction_system.write().await;
        auction
            .execute_auction(order_book.clone(), imbalance_handler.clone())
            .await;
    }
}
