pub mod liquidations;
pub mod orderbooks;
pub mod tickers;
pub mod trades;

use crate::sources::bybit::responses::{
    liquidations::BybitLiquidationData, orderbooks::BybitOrderbookData,
    tickers::BybitTickerData, trades::BybitTradeData,
};

#[derive(Debug)]
pub enum BybitWssEvent {
    LiquidationData(BybitLiquidationData),
    TradeData(BybitTradeData),
    OrderbookData(BybitOrderbookData),
    TickerData(BybitTickerData),
}
