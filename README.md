# closing_auction

This project is an implementation of a high-performance TSX Market-On-Close (MOC) auction system in Rust. The system supports real-time order matching, imbalance handling, and auction price determination, optimized for low latency and high throughput using efficient data structures and parallel processing.

## Features

1. **Order Book Management**
   - Efficient handling of MOC orders.
   - Support for bid and ask orders.
   - Price-time priority maintenance.
   - Real-time updates and fast order matching.

2. **Auction Matching Logic**
   - Auction price calculation based on maximum executable volume, minimum surplus, and reference price alignment.
   - Support for uncrossing at the auction price at market close.

3. **Imbalance Handling**
   - Real-time tracking and display of order imbalances.
   - Dynamic adjustment of matching logic to account for order imbalances.
   - Transparency into imbalance data for market participants.

4. **Order Types and Priority**
   - Support for Market-On-Close (MOC) Orders, Limit-On-Close (LOC) Orders, and Imbalance-Only (IO) Orders.
   - Enforcement of priority based on price and time of submission.

5. **Auction Timing and Phases**
   - Implementation of Order Entry Period, Imbalance Period, Auction Execution, and Post-Auction Period.

6. **Performance and Data Structures**
   - Use of high-performance data structures like `BTreeMap` and `SkipList`.
   - Use of `Arc` and `RwLock` for thread-safe access to shared data.
   - Parallel processing using `tokio` or `rayon`.

7. **Fairness and Transparency**
   - Fair price formation.
   - Real-time order and imbalance data for all participants.
   - Logging and monitoring of auction performance and order execution quality.

8. **Testing and Error Handling**
   - Comprehensive unit and integration tests.
   - Recovery from partial failures.
   - Handling of edge cases such as large order imbalances, network delays, and data consistency issues.
