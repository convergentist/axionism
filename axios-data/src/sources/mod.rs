pub mod bybit;
pub mod morpho;
pub use morpho::MorphoClient;

#[derive(Debug, Clone)]
pub enum Exchange {
    Bybit,
}

#[derive(Debug, Clone)]
pub enum Protocol {
    Morpho,
}
