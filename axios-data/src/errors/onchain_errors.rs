#[derive(Debug)]
pub enum MorphoError {
    Http(reqwest::Error),
    GraphQL(Vec<String>),
    NoData,
}

impl From<reqwest::Error> for MorphoError {
    fn from(e: reqwest::Error) -> Self {
        MorphoError::Http(e)
    }
}
