use crate::auction::AuctionSystem;
use crate::imbalance::ImbalanceHandler;
use crate::order_book::OrderBook;
use crate::orders::{Order, OrderSide, OrderType};

use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;

pub async fn order_entry_period(order_book: Arc<RwLock<OrderBook>>) {
    log::info!("Order entry period...");

    let mut ob = order_book.write().await;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let orders = vec![
        Order {
            id: 1,
            price: 100.0,
            quantity: 500,
            timestamp: now,
            order_type: OrderType::MOC,
            side: OrderSide::Bid,
        },
        Order {
            id: 2,
            price: 101.0,
            quantity: 300,
            timestamp: now + 1,
            order_type: OrderType::LOC,
            side: OrderSide::Bid,
        },
        Order {
            id: 3,
            price: 99.0,
            quantity: 400,
            timestamp: now + 2,
            order_type: OrderType::MOC,
            side: OrderSide::Ask,
        },
        Order {
            id: 4,
            price: 98.5,
            quantity: 200,
            timestamp: now + 3,
            order_type: OrderType::LOC,
            side: OrderSide::Ask,
        },
    ];

    for order in orders {
        ob.add_order(order);
    }

    log::info!("Orders added during Order Entry Period...");
}

pub async fn imbalance_period(
    order_book: Arc<RwLock<OrderBook>>,
    imbalance_handler: Arc<RwLock<ImbalanceHandler>>,
) {
    log::info!("Imbalance period...");

    let ob = order_book.read().await;
    let total_bid: u64 = ob.bids.values().flatten().map(|o| o.quantity).sum();
    let total_ask: u64 = ob.asks.values().flatten().map(|o| o.quantity).sum();

    {
        let handler = imbalance_handler.read().await;

        handler.update(total_bid as i64, total_ask as i64);
        handler.track_imbalances().await;
    }
}

pub async fn auction_execution(
    order_book: Arc<RwLock<OrderBook>>,
    auction_system: Arc<RwLock<AuctionSystem>>,
    imbalance_handler: Arc<RwLock<ImbalanceHandler>>,
) {
    log::info!("Auction execution phase...");

    {
        let mut auction = auction_system.write().await;

        auction
            .execute_auction(order_book.clone(), imbalance_handler.clone())
            .await;
    }
}

pub async fn post_auction_period(order_book: Arc<RwLock<OrderBook>>) {
    log::info!("Post-auction period...");

    let mut ob = order_book.write().await;

    ob.clear();

    log::info!("Order book cleared for continuous trading...");
}
