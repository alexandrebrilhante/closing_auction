use std::sync::atomic::{AtomicI64, Ordering as AtomicOrdering};

pub struct ImbalanceHandler {
    pub imbalance: AtomicI64,
}

impl ImbalanceHandler {
    pub fn new() -> Self {
        Self {
            imbalance: AtomicI64::new(0),
        }
    }

    pub fn update(&self, bid_qty: i64, ask_qty: i64) {
        let diff = bid_qty - ask_qty;

        self.imbalance.store(diff, AtomicOrdering::Relaxed);
    }

    pub fn get(&self) -> i64 {
        self.imbalance.load(AtomicOrdering::Relaxed)
    }

    pub async fn track_imbalances(&self) {
        let current = self.get();

        log::info!("Current Order Imbalance: {}", current);
    }

    pub async fn adjust_matching_logic(&self) {
        let current = self.get();
        log::info!("Adjusting matching logic for imbalance: {}...", current);
    }
}
