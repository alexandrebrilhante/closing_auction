use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

mod auction;
mod imbalance;
mod order_book;
mod orders;
mod phases;

#[cfg(test)]
mod tests;

use auction::{start_auction_system, AuctionSystem};
use imbalance::ImbalanceHandler;
use order_book::OrderBook;

#[tokio::main]
async fn main() {
    env_logger::init();

    let order_book = Arc::new(RwLock::new(OrderBook::new()));
    let auction_system = Arc::new(RwLock::new(AuctionSystem::new()));
    let imbalance_handler = Arc::new(RwLock::new(ImbalanceHandler::new()));

    phases::order_entry_period(order_book.clone()).await;

    phases::imbalance_period(order_book.clone(), imbalance_handler.clone()).await;

    phases::auction_execution(
        order_book.clone(),
        auction_system.clone(),
        imbalance_handler.clone(),
    )
    .await;

    phases::post_auction_period(order_book.clone()).await;

    start_auction_system(order_book, auction_system, imbalance_handler).await;

    sleep(Duration::from_secs(1)).await;
}
