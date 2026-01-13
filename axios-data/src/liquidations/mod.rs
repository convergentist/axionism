use rand::prelude::IndexedRandom;
use rand::{rng, Rng};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Liquidation {
    pub liquidation_ts: u64,
    pub symbol: String,
    pub side: String,
    pub amount: f64,
    pub price: f64,
    pub exchange: String,
}

impl Liquidation {
    pub fn builder() -> LiquidationBuilder {
        LiquidationBuilder::new()
    }

    pub fn random() -> Self {
        let r_liquidation_ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let sides = ["Buy", "Sell"];
        let exchanges = ["bybit", "kraken", "coinbase", "binance"];
        let mut rng = rng();
        let r_symbol = "BTCUSDT".to_string();

        let r_side = sides
            .choose(&mut rng)
            .expect("Error in random side choice")
            .to_string();

        let r_amount = rng.random_range(0.01..1.10);
        let r_price = rng.random_range(100_000.0..110_000.0);
        let r_exchange = exchanges
            .choose(&mut rng)
            .expect("Error in random side choice")
            .to_string();

        Self {
            liquidation_ts: r_liquidation_ts,
            symbol: r_symbol,
            side: r_side,
            amount: r_amount,
            price: r_price,
            exchange: r_exchange,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LiquidationBuilder {
    pub liquidation_ts: Option<u64>,
    pub symbol: Option<String>,
    pub side: Option<String>,
    pub amount: Option<f64>,
    pub price: Option<f64>,
    pub exchange: Option<String>,
}

impl Default for LiquidationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl LiquidationBuilder {
    pub fn new() -> Self {
        LiquidationBuilder {
            liquidation_ts: None,
            symbol: None,
            side: None,
            amount: None,
            price: None,
            exchange: None,
        }
    }

    pub fn ts(mut self, ts: u64) -> Self {
        self.liquidation_ts = Some(ts);
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

    pub fn build(self) -> Result<Liquidation, String> {
        let liquidation_ts = self.liquidation_ts.ok_or("Missing ts")?;
        let symbol = self.symbol.ok_or("Missing symbol")?;
        let side = self.side.ok_or("Missing side")?;
        let amount = self.amount.ok_or("Missing amount")?;
        let price = self.price.ok_or("Missing price")?;
        let exchange = self.exchange.ok_or("Missing exchange")?;

        Ok(Liquidation {
            liquidation_ts,
            symbol,
            side,
            amount,
            price,
            exchange,
        })
    }
}
