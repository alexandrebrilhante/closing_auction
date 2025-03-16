#[cfg(test)]
mod tests {
    use crate::auction::AuctionSystem;
    use crate::imbalance::ImbalanceHandler;
    use crate::order_book::OrderBook;
    use crate::orders::Order;
    use crate::orders::{OrderSide, OrderType};

    use std::sync::Arc;
    use std::time::{SystemTime, UNIX_EPOCH};
    use tokio::sync::RwLock;

    fn create_order(
        id: u64,
        price: f64,
        quantity: u64,
        side: OrderSide,
        order_type: OrderType,
        timestamp_offset: u64,
    ) -> Order {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Order {
            id,
            price,
            quantity,
            timestamp: now + timestamp_offset,
            order_type,
            side,
        }
    }

    #[tokio::test]
    async fn test_order_book_add_and_clear() {
        let mut ob = OrderBook::new();

        let order1 = create_order(1, 100.0, 500, OrderSide::Bid, OrderType::MOC, 0);
        let order2 = create_order(2, 99.0, 400, OrderSide::Ask, OrderType::LOC, 1);

        ob.add_order(order1);
        ob.add_order(order2);

        assert!(!ob.bids.is_empty());
        assert!(!ob.asks.is_empty());

        ob.clear();

        assert!(ob.bids.is_empty());
        assert!(ob.asks.is_empty());
    }

    #[tokio::test]
    async fn test_auction_clearing_price() {
        let order_book = Arc::new(RwLock::new(OrderBook::new()));
        let imbalance_handler = Arc::new(RwLock::new(ImbalanceHandler::new()));

        {
            let mut ob = order_book.write().await;

            ob.add_order(create_order(
                1,
                101.0,
                300,
                OrderSide::Bid,
                OrderType::MOC,
                0,
            ));

            ob.add_order(create_order(
                2,
                100.0,
                500,
                OrderSide::Bid,
                OrderType::LOC,
                1,
            ));

            ob.add_order(create_order(
                3,
                99.0,
                400,
                OrderSide::Ask,
                OrderType::MOC,
                2,
            ));

            ob.add_order(create_order(
                4,
                98.0,
                200,
                OrderSide::Ask,
                OrderType::LOC,
                3,
            ));
        }

        let auction = AuctionSystem::new();

        let clearing_price = auction
            .calculate_auction_price(order_book.clone(), imbalance_handler.clone())
            .await;

        assert!(clearing_price.is_some());

        let cp = clearing_price.unwrap();

        assert!(cp >= 98.0 && cp <= 101.0);
    }

    #[tokio::test]
    async fn test_imbalance_tracking() {
        let handler = ImbalanceHandler::new();

        handler.update(800, 600);

        assert_eq!(handler.get(), 200);
    }
}
