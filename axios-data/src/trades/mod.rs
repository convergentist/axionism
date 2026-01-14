use rand::prelude::IndexedRandom;
use rand::{Rng, rng};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Trade {
    pub trade_ts: u64,
    pub symbol: String,
    pub side: String,
    pub amount: f64,
    pub price: f64,
    pub exchange: String,
    pub id: String,
}

impl Trade {
    pub fn builder() -> TradeBuilder {
        TradeBuilder::new()
    }

    pub fn random() -> Self {
        let r_ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let sides = ["Buy", "Sell"];
        let mut rng = rng();
        let r_side = sides
            .choose(&mut rng)
            .expect("Error in random side choice")
            .to_string();

        let exchanges = ["bybit", "kraken", "coinbase", "binance"];

        let r_symbol = "".to_string();
        let r_amount = rng.random_range(0.01..1.10);
        let r_price = rng.random_range(100_000.0..110_000.0);
        let r_exchange = exchanges
            .choose(&mut rng)
            .expect("Error in random side choice")
            .to_string();
        let r_id = "".to_string();

        Self {
            trade_ts: r_ts,
            symbol: r_symbol,
            side: r_side,
            amount: r_amount,
            price: r_price,
            exchange: r_exchange,
            id: r_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TradeBuilder {
    pub trade_ts: Option<u64>,
    pub symbol: Option<String>,
    pub side: Option<String>,
    pub amount: Option<f64>,
    pub price: Option<f64>,
    pub exchange: Option<String>,
    pub id: Option<String>,
}

impl Default for TradeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TradeBuilder {
    pub fn new() -> Self {
        TradeBuilder {
            trade_ts: None,
            symbol: None,
            side: None,
            amount: None,
            price: None,
            exchange: None,
            id: None,
        }
    }

    pub fn trade_ts(mut self, trade_ts: u64) -> Self {
        self.trade_ts = Some(trade_ts);
        self
    }

    pub fn symbol(mut self, symbol: String) -> Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn side(mut self, side: String) -> Self {
        self.side = Some(side);
        self
    }

    pub fn amount(mut self, amount: f64) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }

    pub fn exchange(mut self, exchange: String) -> Self {
        self.exchange = Some(exchange);
        self
    }

    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn build(self) -> Result<Trade, String> {
        let trade_ts = self.trade_ts.ok_or("Missing trade_ts")?;
        let symbol = self.symbol.ok_or("Missing symbol")?;
        let side = self.side.ok_or("Missing side")?;
        let amount = self.amount.ok_or("Missing amount")?;
        let price = self.price.ok_or("Missing price")?;
        let exchange = self.exchange.ok_or("Missing exchange")?;
        let id = self.id.ok_or("Missing id")?;

        Ok(Trade {
            trade_ts,
            symbol,
            side,
            amount,
            price,
            exchange,
            id,
        })
    }
}
