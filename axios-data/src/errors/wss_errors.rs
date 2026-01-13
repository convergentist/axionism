use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum WssError {
    // Connection Error
    #[error("Connection Failed")]
    WssConnection,
}

