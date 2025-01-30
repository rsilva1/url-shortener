pub use error::{Error, Result};

use axum::{
    extract::{Path, State}, http::StatusCode, response::Redirect, routing::{get, post, put}, Json, Router
};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use interfaces::{ConnectionPool, CreateMappingParams, PutMappingParams};
use tokio_postgres::NoTls;
use url_shortener::{ShortCode, UrlMapping, UrlMappingWithStats};

mod error;
mod interfaces;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    // set up connection pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let manager = PostgresConnectionManager::new_from_stringlike(&database_url, NoTls).unwrap();
    let pool = Pool::builder().build(manager).await.unwrap();

    let app = Router::new()
        .route("/health", get(health))
        .route("/shorten", post(create_mapping))
        .route(
            "/shorten/{code}",
            put(put_mapping).get(get_mapping).delete(delete_mapping),
        )
        .route("/shorten/{code}/stats", get(get_mapping_stats))
        .route("/{code}", get(try_redirect))
        .with_state(pool);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening on PORT 3000");
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn health() -> &'static str {
    "alive"
}

async fn get_mapping(
    State(pool): State<ConnectionPool>,
    Path(code): Path<ShortCode>,
) -> Result<Json<UrlMapping>> {
    let client = pool.get().await.map_err(|_| Error::InternalServerError)?;
    let url_mapping = url_shortener::UrlShortenerApi::new()
        .get_mapping(&client, &code)
        .await
        .unwrap();
    Ok(Json(url_mapping))
}

async fn get_mapping_stats(
    State(pool): State<ConnectionPool>,
    Path(code): Path<ShortCode>,
) -> Result<Json<UrlMappingWithStats>> {
    let client = pool.get().await.map_err(|_| Error::InternalServerError)?;
    let url_mapping_with_stats = url_shortener::UrlShortenerApi::new()
        .get_mapping_with_stats(&client, &code)
        .await
        .unwrap();
    Ok(Json(url_mapping_with_stats))
}

async fn put_mapping(
    State(pool): State<ConnectionPool>,
    Path(code): Path<ShortCode>,
    Json(PutMappingParams { url }): Json<PutMappingParams>,
) -> Result<Json<UrlMapping>> {
    let client = pool.get().await.map_err(|_| Error::InternalServerError)?;
    let url_mapping = url_shortener::UrlShortenerApi::new()
        .update_mapping(&client, &code, &url)
        .await
        .unwrap();
    Ok(Json(url_mapping))
}

async fn delete_mapping(
    State(pool): State<ConnectionPool>,
    Path(code): Path<ShortCode>,
) -> Result<StatusCode> {
    let client = pool.get().await.map_err(|_| Error::InternalServerError)?;
    url_shortener::UrlShortenerApi::new()
        .delete_mapping(&client, &code)
        .await
        .unwrap();
    Ok(StatusCode::OK)
}

async fn create_mapping(
    State(pool): State<ConnectionPool>,
    Json(body): Json<CreateMappingParams>,
) -> Result<Json<ShortCode>> {
    let client = pool.get().await.map_err(|_| Error::InternalServerError)?;
    let mapping = url_shortener::UrlShortenerApi::new()
        .create_new_mapping(&client, body.url)
        .await
        .unwrap();
    Ok(Json(mapping.short_code))
}

async fn try_redirect(
    State(pool): State<ConnectionPool>,
    Path(code): Path<ShortCode>,
) -> Result<Redirect> {
    let client = pool.get().await.map_err(|_| Error::InternalServerError)?;
    let service = url_shortener::UrlShortenerApi::new();
    let url_mapping = service
        .get_mapping(&client, &code)
        .await
        .unwrap();
    service
        .increase_access_count(&client, &url_mapping.id)
        .await
        .unwrap();
    Ok(Redirect::temporary(url_mapping.url.as_str()))
}
