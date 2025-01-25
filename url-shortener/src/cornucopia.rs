// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod types { }#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod queries
{ pub mod url_queries
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct InsertUrlParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,> { pub id: uuid::Uuid,pub short_code: T1,pub url: T2,pub created_at: time::PrimitiveDateTime,pub updated_at: time::PrimitiveDateTime,}#[derive( Debug)] pub struct UpdateUrlParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,> { pub url: T1,pub code: T2,}#[derive( Debug, Clone, PartialEq,)] pub struct Url
{ pub id : uuid::Uuid,pub short_code : String,pub url : String,pub created_at : time::PrimitiveDateTime,pub updated_at : time::PrimitiveDateTime,}pub struct UrlBorrowed<'a> { pub id : uuid::Uuid,pub short_code : &'a str,pub url : &'a str,pub created_at : time::PrimitiveDateTime,pub updated_at : time::PrimitiveDateTime,}
impl<'a> From<UrlBorrowed<'a>> for Url
{
    fn from(UrlBorrowed { id,short_code,url,created_at,updated_at,}: UrlBorrowed<'a>) ->
    Self { Self { id,short_code: short_code.into(),url: url.into(),created_at,updated_at,} }
}pub struct UrlQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> UrlBorrowed,
    mapper: fn(UrlBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> UrlQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(UrlBorrowed) -> R) ->
    UrlQuery<'a,C,R,N>
    {
        UrlQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}#[derive( Debug, Clone, PartialEq,)] pub struct UrlWithStats
{ pub id : uuid::Uuid,pub short_code : String,pub url : String,pub created_at : time::PrimitiveDateTime,pub updated_at : time::PrimitiveDateTime,pub access_count : i32,}pub struct UrlWithStatsBorrowed<'a> { pub id : uuid::Uuid,pub short_code : &'a str,pub url : &'a str,pub created_at : time::PrimitiveDateTime,pub updated_at : time::PrimitiveDateTime,pub access_count : i32,}
impl<'a> From<UrlWithStatsBorrowed<'a>> for UrlWithStats
{
    fn from(UrlWithStatsBorrowed { id,short_code,url,created_at,updated_at,access_count,}: UrlWithStatsBorrowed<'a>) ->
    Self { Self { id,short_code: short_code.into(),url: url.into(),created_at,updated_at,access_count,} }
}pub struct UrlWithStatsQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> UrlWithStatsBorrowed,
    mapper: fn(UrlWithStatsBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> UrlWithStatsQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(UrlWithStatsBorrowed) -> R) ->
    UrlWithStatsQuery<'a,C,R,N>
    {
        UrlWithStatsQuery
        {
            client: self.client, params: self.params, stmt: self.stmt,
            extractor: self.extractor, mapper,
        }
    } pub async fn one(self) -> Result<T, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let row =
        self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)))
    } pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error>
    { self.iter().await?.try_collect().await } pub async fn opt(self) ->
    Result<Option<T>, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self.client.query_opt(stmt, &self.params) .await?
        .map(|row| (self.mapper)((self.extractor)(&row))))
    } pub async fn iter(self,) -> Result<impl futures::Stream<Item = Result<T,
    tokio_postgres::Error>> + 'a, tokio_postgres::Error>
    {
        let stmt = self.stmt.prepare(self.client).await?; let it =
        self.client.query_raw(stmt,
        cornucopia_async::private::slice_iter(&self.params)) .await?
        .map(move |res|
        res.map(|row| (self.mapper)((self.extractor)(&row)))) .into_stream();
        Ok(it)
    }
}pub fn get_mapping() -> GetMappingStmt
{ GetMappingStmt(cornucopia_async::private::Stmt::new("SELECT id, short_code, url, created_at, updated_at
FROM urls
WHERE urls.short_code = $1")) } pub struct
GetMappingStmt(cornucopia_async::private::Stmt); impl GetMappingStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
short_code: &'a T1,) -> UrlQuery<'a,C,
Url, 1>
{
    UrlQuery
    {
        client, params: [short_code,], stmt: &mut self.0, extractor:
        |row| { UrlBorrowed { id: row.get(0),short_code: row.get(1),url: row.get(2),created_at: row.get(3),updated_at: row.get(4),} }, mapper: |it| { <Url>::from(it) },
    }
} }pub fn get_url_with_stats() -> GetUrlWithStatsStmt
{ GetUrlWithStatsStmt(cornucopia_async::private::Stmt::new("SELECT u.id as id, short_code, url, created_at, updated_at, amount as access_count
FROM urls u
LEFT JOIN visits v ON u.id = v.url_id
WHERE u.short_code = $1")) } pub struct
GetUrlWithStatsStmt(cornucopia_async::private::Stmt); impl GetUrlWithStatsStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
short_code: &'a T1,) -> UrlWithStatsQuery<'a,C,
UrlWithStats, 1>
{
    UrlWithStatsQuery
    {
        client, params: [short_code,], stmt: &mut self.0, extractor:
        |row| { UrlWithStatsBorrowed { id: row.get(0),short_code: row.get(1),url: row.get(2),created_at: row.get(3),updated_at: row.get(4),access_count: row.get(5),} }, mapper: |it| { <UrlWithStats>::from(it) },
    }
} }pub fn insert_url() -> InsertUrlStmt
{ InsertUrlStmt(cornucopia_async::private::Stmt::new("INSERT INTO urls (id, short_code, url, created_at, updated_at)
VALUES ($1, $2, $3, $4, $5)")) } pub struct
InsertUrlStmt(cornucopia_async::private::Stmt); impl InsertUrlStmt
{ pub async fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
id: &'a uuid::Uuid,short_code: &'a T1,url: &'a T2,created_at: &'a time::PrimitiveDateTime,updated_at: &'a time::PrimitiveDateTime,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[id,short_code,url,created_at,updated_at,]).await
} }impl <'a, C: GenericClient + Send + Sync, T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,>
cornucopia_async::Params<'a, InsertUrlParams<T1,T2,>, std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
tokio_postgres::Error>> + Send + 'a>>, C> for InsertUrlStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    InsertUrlParams<T1,T2,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
    tokio_postgres::Error>> + Send + 'a>>
    { Box::pin(self.bind(client, &params.id,&params.short_code,&params.url,&params.created_at,&params.updated_at,)) }
}pub fn update_url() -> UpdateUrlStmt
{ UpdateUrlStmt(cornucopia_async::private::Stmt::new("UPDATE urls
SET url = $1
WHERE short_code = $2
RETURNING *")) } pub struct
UpdateUrlStmt(cornucopia_async::private::Stmt); impl UpdateUrlStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
url: &'a T1,code: &'a T2,) -> UrlQuery<'a,C,
Url, 2>
{
    UrlQuery
    {
        client, params: [url,code,], stmt: &mut self.0, extractor:
        |row| { UrlBorrowed { id: row.get(0),short_code: row.get(1),url: row.get(2),created_at: row.get(3),updated_at: row.get(4),} }, mapper: |it| { <Url>::from(it) },
    }
} }impl <'a, C: GenericClient,T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,> cornucopia_async::Params<'a,
UpdateUrlParams<T1,T2,>, UrlQuery<'a, C,
Url, 2>, C> for UpdateUrlStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    UpdateUrlParams<T1,T2,>) -> UrlQuery<'a, C,
    Url, 2>
    { self.bind(client, &params.url,&params.code,) }
}pub fn delete_url() -> DeleteUrlStmt
{ DeleteUrlStmt(cornucopia_async::private::Stmt::new("DELETE FROM urls
WHERE short_code = $1")) } pub struct
DeleteUrlStmt(cornucopia_async::private::Stmt); impl DeleteUrlStmt
{ pub async fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
short_code: &'a T1,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[short_code,]).await
} }}}