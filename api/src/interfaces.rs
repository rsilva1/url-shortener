use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use serde::Deserialize;
use tokio_postgres::NoTls;
use url_shortener::Url;

pub type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

#[derive(Debug, Deserialize)]
pub struct CreateMappingParams {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct PutMappingParams {
    pub url: Url,
}

