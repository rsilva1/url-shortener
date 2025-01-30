use cornucopia_async::Params;
use uuid::Uuid;

use crate::models::{ShortCode, Url, UrlMapping, UrlMappingWithStats};
use crate::{Error, Result};

use crate::cornucopia::queries::url_queries::{
    delete_url, get_url, get_url_with_stats, increase_access_count, insert_url, update_url,
    InsertUrlParams, UpdateUrlParams,
};

pub struct UrlService {
    client: tokio_postgres::Client,
}

impl UrlService {
    pub fn new(client: tokio_postgres::Client) -> Self {
        Self { client }
    }

    pub async fn get_mapping(
        &self,
        code: &ShortCode,
    ) -> Result<UrlMapping> {
        let url_mapping = get_url()
            .bind(&self.client, &code.as_str())
            .map(|row| row.into())
            .one()
            .await
            // todo consider other errors like connection issues
            .map_err(|_| Error::CodeNotFound {
                code: code.as_str().to_string(),
            })?;
        Ok(url_mapping)
    }

    pub async fn get_mapping_with_stats(
        &self,
        client: &tokio_postgres::Client,
        code: &ShortCode,
    ) -> Result<UrlMappingWithStats> {
        get_url_with_stats()
            .bind(client, &code.as_str())
            .map(|row| row.into())
            .one()
            .await
            .map_err(|_| Error::CodeNotFound {
                code: code.as_str().to_string(),
            })
    }

    pub async fn create_new_mapping(
        &self,
        client: &tokio_postgres::Client,
        url: String,
    ) -> Result<UrlMapping> {
        let id = Uuid::new_v4();
        let url = Url::new(url)?;
        let code = ShortCode::generate();
        let url_mapping = UrlMapping::new(id, url, code);
        let result = insert_url()
            .params(
                client,
                &InsertUrlParams {
                    id: url_mapping.id,
                    short_code: url_mapping.short_code.as_str(),
                    url: url_mapping.url.as_str(),
                    created_at: url_mapping.created_at,
                    updated_at: url_mapping.updated_at,
                },
            )
            .await;
        match result {
            Ok(_) => Ok(url_mapping),
            Err(e) => {
                println!("Error on query: {}", e);
                Err(Error::DbError)
            }
        }
    }

    pub async fn update_mapping(
        &self,
        client: &tokio_postgres::Client,
        code: &ShortCode,
        url: &Url,
    ) -> Result<UrlMapping> {
        update_url()
            .params(
                client,
                &UpdateUrlParams {
                    url: url.as_str(),
                    code: code.as_str(),
                },
            )
            .map(|row| row.into())
            .one()
            .await
            .map_err(|e| {
                println!("Error on query: {e}");
                Error::DbError
            })
    }

    pub async fn delete_mapping(
        &self,
        client: &tokio_postgres::Client,
        code: &ShortCode,
    ) -> Result<()> {
        delete_url()
            .bind(client, &code.as_str().to_string())
            .await
            .map(|_| ())
            .map_err(|e| {
                println!("Error: {}", e);
                Error::DbError
            })
    }

    pub async fn increase_access_count(
        &self,
        client: &tokio_postgres::Client,
        id: &Uuid,
    ) -> Result<()> {
        increase_access_count()
            .bind(client, id)
            .await
            .map(|_| ())
            .map_err(|e| {
                println!("Error: {}", e);
                Error::DbError
            })
    }
}
