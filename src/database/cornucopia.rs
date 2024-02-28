// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy :: all, clippy :: pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod types { }#[allow(clippy :: all, clippy :: pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod queries
{ pub mod hangar_project
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct UpsertHangarProjectParams < T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,T3 : cornucopia_async::StringSql,T4 : cornucopia_async::StringSql,T5 : cornucopia_async::StringSql,T6 : cornucopia_async::StringSql,T7 : cornucopia_async::StringSql,T8 : cornucopia_async::StringSql,T9 : cornucopia_async::StringSql,T10 : cornucopia_async::StringSql,T11 : cornucopia_async::StringSql,> { pub slug : T1,pub owner : T2,pub name : T3,pub description : T4,pub created_at : time::OffsetDateTime,pub last_updated : time::OffsetDateTime,pub visibility : T5,pub avatar_url : T6,pub version : T7,pub source_code_link : Option<T8>,pub source_repository_host : Option<T9>,pub source_repository_owner : Option<T10>,pub source_repository_name : Option<T11>,}#[derive( Debug, Clone, PartialEq, )] pub struct HangarProjectEntity
{ pub slug : String,pub owner : String,pub name : String,pub description : String,pub created_at : time::OffsetDateTime,pub last_updated : time::OffsetDateTime,pub visibility : String,pub avatar_url : String,pub version : String,pub source_code_link : Option<String>,pub source_repository_host : Option<String>,pub source_repository_owner : Option<String>,pub source_repository_name : Option<String>,}pub struct HangarProjectEntityBorrowed < 'a >
{ pub slug : &'a str,pub owner : &'a str,pub name : &'a str,pub description : &'a str,pub created_at : time::OffsetDateTime,pub last_updated : time::OffsetDateTime,pub visibility : &'a str,pub avatar_url : &'a str,pub version : &'a str,pub source_code_link : Option<&'a str>,pub source_repository_host : Option<&'a str>,pub source_repository_owner : Option<&'a str>,pub source_repository_name : Option<&'a str>,} impl < 'a > From < HangarProjectEntityBorrowed <
'a >> for HangarProjectEntity
{
    fn
    from(HangarProjectEntityBorrowed { slug,owner,name,description,created_at,last_updated,visibility,avatar_url,version,source_code_link,source_repository_host,source_repository_owner,source_repository_name,} : HangarProjectEntityBorrowed < 'a >)
    -> Self { Self { slug: slug.into(),owner: owner.into(),name: name.into(),description: description.into(),created_at,last_updated,visibility: visibility.into(),avatar_url: avatar_url.into(),version: version.into(),source_code_link: source_code_link.map(|v| v.into()),source_repository_host: source_repository_host.map(|v| v.into()),source_repository_owner: source_repository_owner.map(|v| v.into()),source_repository_name: source_repository_name.map(|v| v.into()),} }
}pub struct HangarProjectEntityQuery < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a  C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut cornucopia_async
    :: private :: Stmt, extractor : fn(& tokio_postgres :: Row) -> HangarProjectEntityBorrowed,
    mapper : fn(HangarProjectEntityBorrowed) -> T,
} impl < 'a, C, T : 'a, const N : usize > HangarProjectEntityQuery < 'a, C, T, N >
where C : GenericClient
{
    pub fn map < R > (self, mapper : fn(HangarProjectEntityBorrowed) -> R) -> HangarProjectEntityQuery
    < 'a, C, R, N >
    {
        HangarProjectEntityQuery
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
}pub fn upsert_hangar_project() -> UpsertHangarProjectStmt
{ UpsertHangarProjectStmt(cornucopia_async :: private :: Stmt :: new("INSERT INTO hangar_project (slug, owner, name, description, created_at, last_updated, visibility, avatar_url, version, source_code_link, source_repository_host, source_repository_owner, source_repository_name)
  VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
  ON CONFLICT (slug)
  DO UPDATE SET
    owner = EXCLUDED.owner,
    name = EXCLUDED.name,
    description = EXCLUDED.description,
    created_at = EXCLUDED.created_at,
    last_updated = EXCLUDED.last_updated,
    visibility = EXCLUDED.visibility,
    avatar_url = EXCLUDED.avatar_url,
    version = EXCLUDED.version,
    source_code_link = EXCLUDED.source_code_link,
    source_repository_host = EXCLUDED.source_repository_host,
    source_repository_owner = EXCLUDED.source_repository_owner,
    source_repository_name = EXCLUDED.source_repository_name")) } pub
struct UpsertHangarProjectStmt(cornucopia_async :: private :: Stmt) ; impl
UpsertHangarProjectStmt { pub async fn bind < 'a, C : GenericClient, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,T3 : cornucopia_async::StringSql,T4 : cornucopia_async::StringSql,T5 : cornucopia_async::StringSql,T6 : cornucopia_async::StringSql,T7 : cornucopia_async::StringSql,T8 : cornucopia_async::StringSql,T9 : cornucopia_async::StringSql,T10 : cornucopia_async::StringSql,T11 : cornucopia_async::StringSql,>
(& 'a mut self, client : & 'a  C,
slug : & 'a T1,owner : & 'a T2,name : & 'a T3,description : & 'a T4,created_at : & 'a time::OffsetDateTime,last_updated : & 'a time::OffsetDateTime,visibility : & 'a T5,avatar_url : & 'a T6,version : & 'a T7,source_code_link : & 'a Option<T8>,source_repository_host : & 'a Option<T9>,source_repository_owner : & 'a Option<T10>,source_repository_name : & 'a Option<T11>,) -> Result < u64, tokio_postgres :: Error >
{
    let stmt = self.0.prepare(client) .await ? ;
    client.execute(stmt, & [slug,owner,name,description,created_at,last_updated,visibility,avatar_url,version,source_code_link,source_repository_host,source_repository_owner,source_repository_name,]) .await
} }impl < 'a, C : GenericClient + Send + Sync, T1 : cornucopia_async::StringSql,T2 : cornucopia_async::StringSql,T3 : cornucopia_async::StringSql,T4 : cornucopia_async::StringSql,T5 : cornucopia_async::StringSql,T6 : cornucopia_async::StringSql,T7 : cornucopia_async::StringSql,T8 : cornucopia_async::StringSql,T9 : cornucopia_async::StringSql,T10 : cornucopia_async::StringSql,T11 : cornucopia_async::StringSql,>
cornucopia_async :: Params < 'a, UpsertHangarProjectParams < T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,>, std::pin::Pin<Box<dyn futures::Future<Output = Result <
u64, tokio_postgres :: Error > > + Send + 'a>>, C > for UpsertHangarProjectStmt
{
    fn
    params(& 'a mut self, client : & 'a  C, params : & 'a
    UpsertHangarProjectParams < T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result < u64, tokio_postgres ::
    Error > > + Send + 'a>> { Box::pin(self.bind(client, & params.slug,& params.owner,& params.name,& params.description,& params.created_at,& params.last_updated,& params.visibility,& params.avatar_url,& params.version,& params.source_code_link,& params.source_repository_host,& params.source_repository_owner,& params.source_repository_name,) ) }
}pub fn get_hangar_projects() -> GetHangarProjectsStmt
{ GetHangarProjectsStmt(cornucopia_async :: private :: Stmt :: new("SELECT * FROM hangar_project")) } pub
struct GetHangarProjectsStmt(cornucopia_async :: private :: Stmt) ; impl
GetHangarProjectsStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
) -> HangarProjectEntityQuery < 'a, C,
HangarProjectEntity, 0 >
{
    HangarProjectEntityQuery
    {
        client, params : [], stmt : & mut self.0, extractor :
        | row | { HangarProjectEntityBorrowed { slug : row.get(0),owner : row.get(1),name : row.get(2),description : row.get(3),created_at : row.get(4),last_updated : row.get(5),visibility : row.get(6),avatar_url : row.get(7),version : row.get(8),source_code_link : row.get(9),source_repository_host : row.get(10),source_repository_owner : row.get(11),source_repository_name : row.get(12),} }, mapper : | it | { <HangarProjectEntity>::from(it) },
    }
} }pub fn get_latest_hangar_project_update_date() -> GetLatestHangarProjectUpdateDateStmt
{ GetLatestHangarProjectUpdateDateStmt(cornucopia_async :: private :: Stmt :: new("SELECT max(last_updated) FROM hangar_project")) } pub
struct GetLatestHangarProjectUpdateDateStmt(cornucopia_async :: private :: Stmt) ; impl
GetLatestHangarProjectUpdateDateStmt { pub fn bind < 'a, C : GenericClient, >
(& 'a mut self, client : & 'a  C,
) -> TimeOffsetDateTimeQuery < 'a, C,
time::OffsetDateTime, 0 >
{
    TimeOffsetDateTimeQuery
    {
        client, params : [], stmt : & mut self.0, extractor :
        | row | { row.get(0) }, mapper : | it | { it },
    }
} }}pub mod spigot_author
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
{ GetLatestSpigotResourceUpdateDateStmt(cornucopia_async :: private :: Stmt :: new("SELECT max(update_date) FROM spigot_resource")) } pub
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