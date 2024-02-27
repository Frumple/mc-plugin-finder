// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy :: all, clippy :: pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod types { }#[allow(clippy :: all, clippy :: pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod queries
{ pub mod spigot_author
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct InsertSpigotAuthorParams < T1 : cornucopia_async::StringSql,> { pub id : i32,pub name : T1,}#[derive( Debug, Clone, PartialEq, )] pub struct SpigotAuthorEntity
{ pub id : i32,pub name : String,}pub struct SpigotAuthorEntityBorrowed < 'a >
{ pub id : i32,pub name : &'a str,} impl < 'a > From < SpigotAuthorEntityBorrowed <
'a >> for SpigotAuthorEntity
{
    fn
    from(SpigotAuthorEntityBorrowed { id,name,} : SpigotAuthorEntityBorrowed < 'a >)
    -> Self { Self { id,name: name.into(),} }
}pub struct SpigotAuthorEntityQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> SpigotAuthorEntityBorrowed,
    mapper : fn(SpigotAuthorEntityBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > SpigotAuthorEntityQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(SpigotAuthorEntityBorrowed) -> R) -> SpigotAuthorEntityQuery
    < 'a, C, R, N >
    {
        SpigotAuthorEntityQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}pub struct I32Query < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> i32,
    mapper : fn(i32) -> T,
} impl < 'a, C, T : 'a, const N : usize > I32Query < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(i32) -> R) -> I32Query
    < 'a, C, R, N >
    {
        I32Query
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}pub fn insert_spigot_author() -> InsertSpigotAuthorStmt
{ InsertSpigotAuthorStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO spigot_author (id, name) VALUES ($1, $2)")) } pub
struct InsertSpigotAuthorStmt(cornucopia_async :: private :: Stmt) ; impl
InsertSpigotAuthorStmt { pub async fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
id : & 'a i32,name : & 'a T1,) -> Result < u64, tokio_postgres :: Error >
{
    let stmt = self.0.prepare(client) .await ? ;
    client.execute(stmt, & [id,name,]) .await
} }impl < 'a, C : GenericClient + Send + Sync, T1 : cornucopia_async::StringSql,>
cornucopia_async :: Params < 'a, InsertSpigotAuthorParams < T1,>, std::pin::Pin<Box<dyn futures::Future<Output = Result <
u64, tokio_postgres :: Error > > + Send + 'a>>, C > for InsertSpigotAuthorStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    InsertSpigotAuthorParams < T1,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result < u64, tokio_postgres ::
    Error > > + Send + 'a>> { Box::pin(self.bind(client, & params.id,& params.name,) ) }
}pub fn get_spigot_authors() -> GetSpigotAuthorsStmt
{ GetSpigotAuthorsStmt(cornucopia_async :: private :: Stmt :: new("SELECT id, name FROM spigot_author")) } pub
struct GetSpigotAuthorsStmt(cornucopia_async :: private :: Stmt) ; impl
GetSpigotAuthorsStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
) -> SpigotAuthorEntityQuery < 'a, C,
SpigotAuthorEntity, 0 >
{
    SpigotAuthorEntityQuery
    {
        client, params : [], stmt : & mut self.0, extractor :
        | row | { SpigotAuthorEntityBorrowed { id : row.get(0),name : row.get(1),} }, mapper : | it | { <SpigotAuthorEntity>::from(it) },
    }
} }pub fn get_highest_spigot_author_id() -> GetHighestSpigotAuthorIdStmt
{ GetHighestSpigotAuthorIdStmt(cornucopia_async :: private :: Stmt :: new("SELECT max(id) from spigot_author")) } pub
struct GetHighestSpigotAuthorIdStmt(cornucopia_async :: private :: Stmt) ; impl
GetHighestSpigotAuthorIdStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
) -> I32Query < 'a, C,
i32, 0 >
{
    I32Query
    {
        client, params : [], stmt : & mut self.0, extractor :
        | row | { row.get(0) }, mapper : | it | { it },
    }
} }}pub mod spigot_resource
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct UpsertSpigotResourceParams < T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,T3 : cornucopia_async::StringSql,T4 : cornucopia_async::StringSql,T5 : cornucopia_async::StringSql,T6 : cornucopia_async::StringSql,T7 : cornucopia_async::StringSql,T8 : cornucopia_async::StringSql,> { pub id : i32,pub name : T1,pub tag : T2,pub slug : T3,pub release_date : time::OffsetDateTime,pub update_date : time::OffsetDateTime,pub author_id : i32,pub version_id : i32,pub version_name : Option<T4>,pub premium : Option<bool>,pub source_code_link : Option<T5>,pub source_repository_host : Option<T6>,pub source_repository_owner : Option<T7>,pub source_repository_name : Option<T8>,}#[derive( Debug, Clone, PartialEq, )] pub struct SpigotResourceEntity
{ pub id : i32,pub name : String,pub tag : String,pub slug : String,pub release_date : time::OffsetDateTime,pub update_date : time::OffsetDateTime,pub author_id : i32,pub version_id : i32,pub version_name : Option<String>,pub premium : Option<bool>,pub source_code_link : Option<String>,pub source_repository_host : Option<String>,pub source_repository_owner : Option<String>,pub source_repository_name : Option<String>,}pub struct SpigotResourceEntityBorrowed < 'a >
{ pub id : i32,pub name : &'a str,pub tag : &'a str,pub slug : &'a str,pub release_date : time::OffsetDateTime,pub update_date : time::OffsetDateTime,pub author_id : i32,pub version_id : i32,pub version_name : Option<&'a str>,pub premium : Option<bool>,pub source_code_link : Option<&'a str>,pub source_repository_host : Option<&'a str>,pub source_repository_owner : Option<&'a str>,pub source_repository_name : Option<&'a str>,} impl < 'a > From < SpigotResourceEntityBorrowed <
'a >> for SpigotResourceEntity
{
    fn
    from(SpigotResourceEntityBorrowed { id,name,tag,slug,release_date,update_date,author_id,version_id,version_name,premium,source_code_link,source_repository_host,source_repository_owner,source_repository_name,} : SpigotResourceEntityBorrowed < 'a >)
    -> Self { Self { id,name: name.into(),tag: tag.into(),slug: slug.into(),release_date,update_date,author_id,version_id,version_name: version_name.map(|v| v.into()),premium,source_code_link: source_code_link.map(|v| v.into()),source_repository_host: source_repository_host.map(|v| v.into()),source_repository_owner: source_repository_owner.map(|v| v.into()),source_repository_name: source_repository_name.map(|v| v.into()),} }
}pub struct SpigotResourceEntityQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> SpigotResourceEntityBorrowed,
    mapper : fn(SpigotResourceEntityBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > SpigotResourceEntityQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(SpigotResourceEntityBorrowed) -> R) -> SpigotResourceEntityQuery
    < 'a, C, R, N >
    {
        SpigotResourceEntityQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}pub struct TimeOffsetDateTimeQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> time::OffsetDateTime,
    mapper : fn(time::OffsetDateTime) -> T,
} impl < 'a, C, T : 'a, const N : usize > TimeOffsetDateTimeQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(time::OffsetDateTime) -> R) -> TimeOffsetDateTimeQuery
    < 'a, C, R, N >
    {
        TimeOffsetDateTimeQuery
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub async fn one(self) -> Result < T, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let row =
        self.client.query_one(stmt, & self.params) .await ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub async fn all(self) -> Result < Vec < T >, tokio_postgres :: Error >
    { self.iter() .await ?.try_collect().await } pub async fn opt(self) -> Result
    < Option < T >, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ;
        Ok(self.client.query_opt(stmt, & self.params) .await
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub async fn iter(self,) -> Result < impl futures::Stream < Item = Result
    < T, tokio_postgres :: Error >> + 'a, tokio_postgres :: Error >
    {
        let stmt = self.stmt.prepare(self.client) .await ? ; let it =
        self.client.query_raw(stmt, cornucopia_async :: private ::
        slice_iter(& self.params)) .await ?
        .map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) .into_stream() ;
        Ok(it)
    }
}pub fn upsert_spigot_resource() -> UpsertSpigotResourceStmt
{ UpsertSpigotResourceStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO spigot_resource (id, name, tag, slug, release_date, update_date, author_id, version_id, version_name, premium, source_code_link, source_repository_host, source_repository_owner, source_repository_name)
  VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
  ON CONFLICT (id)
  DO UPDATE SET
    name = EXCLUDED.name,
    tag = EXCLUDED.tag,
    slug = EXCLUDED.slug,
    release_date = EXCLUDED.release_date,
    update_date = EXCLUDED.update_date,
    author_id = EXCLUDED.author_id,
    version_id = EXCLUDED.version_id,
    version_name = EXCLUDED.version_name,
    premium = EXCLUDED.premium,
    source_code_link = EXCLUDED.source_code_link,
    source_repository_host = EXCLUDED.source_repository_host,
    source_repository_owner = EXCLUDED.source_repository_owner,
    source_repository_name = EXCLUDED.source_repository_name")) } pub
struct UpsertSpigotResourceStmt(cornucopia_async :: private :: Stmt) ; impl
UpsertSpigotResourceStmt { pub async fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,T3 : cornucopia_async::StringSql,T4 : cornucopia_async::StringSql,T5 : cornucopia_async::StringSql,T6 : cornucopia_async::StringSql,T7 : cornucopia_async::StringSql,T8 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
id : & 'a i32,name : & 'a T1,tag : & 'a T2,slug : & 'a T3,release_date : & 'a time::OffsetDateTime,update_date : & 'a time::OffsetDateTime,author_id : & 'a i32,version_id : & 'a i32,version_name : & 'a Option<T4>,premium : & 'a Option<bool>,source_code_link : & 'a Option<T5>,source_repository_host : & 'a Option<T6>,source_repository_owner : & 'a Option<T7>,source_repository_name : & 'a Option<T8>,) -> Result < u64, tokio_postgres :: Error >
{
    let stmt = self.0.prepare(client) .await ? ;
    client.execute(stmt, & [id,name,tag,slug,release_date,update_date,author_id,version_id,version_name,premium,source_code_link,source_repository_host,source_repository_owner,source_repository_name,]) .await
} }impl < 'a, C : GenericClient + Send + Sync, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,T3 : cornucopia_async::StringSql,T4 : cornucopia_async::StringSql,T5 : cornucopia_async::StringSql,T6 : cornucopia_async::StringSql,T7 : cornucopia_async::StringSql,T8 : cornucopia_async::StringSql,>
cornucopia_async :: Params < 'a, UpsertSpigotResourceParams < T1,T2,T3,T4,T5,T6,T7,T8,>, std::pin::Pin<Box<dyn futures::Future<Output = Result <
u64, tokio_postgres :: Error > > + Send + 'a>>, C > for UpsertSpigotResourceStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    UpsertSpigotResourceParams < T1,T2,T3,T4,T5,T6,T7,T8,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result < u64, tokio_postgres ::
    Error > > + Send + 'a>> { Box::pin(self.bind(client, & params.id,& params.name,& params.tag,& params.slug,& params.release_date,& params.update_date,& params.author_id,& params.version_id,& params.version_name,& params.premium,& params.source_code_link,& params.source_repository_host,& params.source_repository_owner,& params.source_repository_name,) ) }
}pub fn get_spigot_resources() -> GetSpigotResourcesStmt
{ GetSpigotResourcesStmt(cornucopia_async :: private :: Stmt :: new("SELECT * FROM spigot_resource")) } pub
struct GetSpigotResourcesStmt(cornucopia_async :: private :: Stmt) ; impl
GetSpigotResourcesStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
) -> SpigotResourceEntityQuery < 'a, C,
SpigotResourceEntity, 0 >
{
    SpigotResourceEntityQuery
    {
        client, params : [], stmt : & mut self.0, extractor :
        | row | { SpigotResourceEntityBorrowed { id : row.get(0),name : row.get(1),tag : row.get(2),slug : row.get(3),release_date : row.get(4),update_date : row.get(5),author_id : row.get(6),version_id : row.get(7),version_name : row.get(8),premium : row.get(9),source_code_link : row.get(10),source_repository_host : row.get(11),source_repository_owner : row.get(12),source_repository_name : row.get(13),} }, mapper : | it | { <SpigotResourceEntity>::from(it) },
    }
} }pub fn get_latest_spigot_resource_update_date() -> GetLatestSpigotResourceUpdateDateStmt
{ GetLatestSpigotResourceUpdateDateStmt(cornucopia_async :: private :: Stmt :: new("SELECT max(update_date) from spigot_resource")) } pub
struct GetLatestSpigotResourceUpdateDateStmt(cornucopia_async :: private :: Stmt) ; impl
GetLatestSpigotResourceUpdateDateStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
) -> TimeOffsetDateTimeQuery < 'a, C,
time::OffsetDateTime, 0 >
{
    TimeOffsetDateTimeQuery
    {
        client, params : [], stmt : & mut self.0, extractor :
        | row | { row.get(0) }, mapper : | it | { it },
    }
} }}}