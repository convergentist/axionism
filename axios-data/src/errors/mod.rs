pub mod exchange_errors;
pub use exchange_errors::ExchangeError;

pub mod wss_errors;
pub use wss_errors::WssError;

pub mod orderbook_errors;
pub use orderbook_errors::{LevelError, OrderError};

pub mod onchain_errors;
pub use onchain_errors::MorphoError;
