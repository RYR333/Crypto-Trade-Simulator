use std::collections::VecDeque;
use std::time::Duration;
use reqwest;
use tokio;
use serde_json;
use chrono;

#[derive(Debug)]
struct Trade {
    timestamp: u64,
    trade_type: String,
    price: f64,
    quantity: f64,
}

struct CircularBuffer {
    buffer: VecDeque<f64>,
    capacity: usize,
}

impl CircularBuffer {
    fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    fn add(&mut self, value: f64) {
        if self.buffer.len() == self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(value);
    }

    fn average(&self) -> Option<f64> {
        if self.buffer.is_empty() {
            None
        } else {
            Some(self.buffer.iter().sum::<f64>() / self.buffer.len() as f64)
        }
    }
}

async fn fetch_price() -> Result<f64, reqwest::Error> {
    let response = reqwest::get("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd")
        .await?
        .json::<serde_json::Value>()
        .await?;
    
    Ok(response["bitcoin"]["usd"].as_f64().unwrap_or(0.0))
}

#[tokio::main]
async fn main() {
    println!("Starting Crypto Trade Simulator...");

    let mut short_term_sma = CircularBuffer::new(5);
    let mut long_term_sma = CircularBuffer::new(20);
    let mut trades: Vec<Trade> = Vec::new();

    loop {
        match fetch_price().await {
            Ok(price) => {
                println!("Fetched price: ${}", price);

                short_term_sma.add(price);
                long_term_sma.add(price);

                let short_sma = short_term_sma.average().unwrap_or(0.0);
                let long_sma = long_term_sma.average().unwrap_or(0.0);

                println!("Short SMA: {}, Long SMA: {}", short_sma, long_sma);

                if short_sma > long_sma {
                    println!("Buy Signal!");
                    trades.push(Trade { 
                        timestamp: chrono::Utc::now().timestamp() as u64, 
                        trade_type: "Buy".to_string(), 
                        price, 
                        quantity: 1.0 
                    });
                } else if short_sma < long_sma {
                    println!("Sell Signal!");
                    trades.push(Trade { 
                        timestamp: chrono::Utc::now().timestamp() as u64, 
                        trade_type: "Sell".to_string(), 
                        price, 
                        quantity: 1.0 
                    });
                }
            }
            Err(err) => eprintln!("Failed to fetch price: {:?}", err),
        }

        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular_buffer_addition() {
        let mut buffer = CircularBuffer::new(3);
        buffer.add(1.0);
        buffer.add(2.0);
        buffer.add(3.0);
        assert_eq!(buffer.buffer.len(), 3);

        buffer.add(4.0);
        assert_eq!(buffer.buffer.len(), 3);
        assert_eq!(buffer.buffer.front(), Some(&2.0)); // Oldest value removed
    }

    #[test]
    fn test_circular_buffer_average() {
        let mut buffer = CircularBuffer::new(3);
        buffer.add(1.0);
        buffer.add(2.0);
        buffer.add(3.0);
        assert_eq!(buffer.average().unwrap(), 2.0);
    }

    #[test]
    fn test_empty_buffer_average() {
        let buffer = CircularBuffer::new(3);
        assert_eq!(buffer.average(), None);
    }
}
