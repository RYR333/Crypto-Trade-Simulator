# Crypto Trade Simulator

## Overview
This project is a **Crypto Trade Simulator** that fetches live cryptocurrency price data from a public API and simulates buy/sell trades using a simple moving average (SMA) strategy. It uses a circular buffer for efficient data management.

## Features
- Real-time price fetching using CoinGecko API
- Short-term (5 prices) and Long-term (20 prices) SMA calculation
- Buy and Sell signals based on SMA crossover strategy
- Trade simulation with timestamps and logging
- Efficient data management using a circular buffer

## Tech Stack
- **Language:** Rust
- **Async Runtime:** Tokio
- **HTTP Client:** Reqwest
- **JSON Parsing:** Serde
- **Time Management:** Chrono

## Prerequisites
Ensure the following are installed:
- Rust and Cargo: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Visual Studio Code (with Rust Analyzer Extension)
- WSL (for Windows users)

## Installation
1. Clone the repository:
    ```bash
    git clone <repository_url>
    cd crypto_trade_simulator
    ```
2. Install dependencies:
    ```bash
    cargo build
    ```

## Running the Application
1. Start the application:
    ```bash
    cargo run
    ```
2. The application will fetch Bitcoin prices every minute and simulate trades based on the SMA strategy.

## Running Tests
Unit tests are available to validate the functionality of the circular buffer and SMA calculations.
To run the tests:
```bash
cargo test
```

## Example Output
```
Starting Crypto Trade Simulator...
Fetched price: $65000
Short SMA: $64900, Long SMA: $64850
Buy Signal!
Trade executed: Buy 1.0 BTC at $65000
```

## Design Decisions
- **Circular Buffer**: Used for efficient data storage with constant-time complexity.
- **SMA Crossover**: A simple yet effective trading strategy.
- **Tokio**: Ensures efficient asynchronous processing.

## Assumptions
- Trades are simulated with a fixed quantity of **1.0 BTC**.
- The simulator only supports Bitcoin (BTC/USD) for simplicity.
- API errors are logged, and the program retries automatically.

## License
This project is licensed under the MIT License.

## Contact
For questions or issues, please create a GitHub issue or contact me. ThankQ

