// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod types { }#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod queries
{ pub mod common_project
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct UpsertCommonProjectParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,T5: cornucopia_async::StringSql,T6: cornucopia_async::StringSql,T7: cornucopia_async::StringSql,T8: cornucopia_async::StringSql,T9: cornucopia_async::StringSql,T10: cornucopia_async::StringSql,T11: cornucopia_async::StringSql,> { pub id: Option<i64>,pub date_created: time::OffsetDateTime,pub date_updated: time::OffsetDateTime,pub spigot_id: Option<i32>,pub spigot_name: Option<T1>,pub spigot_description: Option<T2>,pub spigot_author: Option<T3>,pub modrinth_id: Option<T4>,pub modrinth_name: Option<T5>,pub modrinth_description: Option<T6>,pub modrinth_author: Option<T7>,pub hangar_slug: Option<T8>,pub hangar_name: Option<T9>,pub hangar_description: Option<T10>,pub hangar_author: Option<T11>,}#[derive( Debug)] pub struct SearchCommonProjectsParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,> { pub spigot: bool,pub name: bool,pub query: T1,pub description: bool,pub author: bool,pub modrinth: bool,pub hangar: bool,pub sort_field: T2,}#[derive( Debug, Clone, PartialEq,)] pub struct CommonProjectEntity
{ pub id : Option<i32>,pub date_created : time::OffsetDateTime,pub date_updated : time::OffsetDateTime,pub spigot_id : Option<i32>,pub spigot_slug : Option<String>,pub spigot_name : Option<String>,pub spigot_description : Option<String>,pub spigot_author : Option<String>,pub spigot_version : Option<String>,pub spigot_premium : Option<bool>,pub modrinth_id : Option<String>,pub modrinth_slug : Option<String>,pub modrinth_name : Option<String>,pub modrinth_description : Option<String>,pub modrinth_author : Option<String>,pub modrinth_version : Option<String>,pub hangar_slug : Option<String>,pub hangar_name : Option<String>,pub hangar_description : Option<String>,pub hangar_author : Option<String>,pub hangar_version : Option<String>,pub source_repository_host : Option<String>,pub source_repository_owner : Option<String>,pub source_repository_name : Option<String>,}pub struct CommonProjectEntityBorrowed<'a> { pub id : Option<i32>,pub date_created : time::OffsetDateTime,pub date_updated : time::OffsetDateTime,pub spigot_id : Option<i32>,pub spigot_slug : Option<&'a str>,pub spigot_name : Option<&'a str>,pub spigot_description : Option<&'a str>,pub spigot_author : Option<&'a str>,pub spigot_version : Option<&'a str>,pub spigot_premium : Option<bool>,pub modrinth_id : Option<&'a str>,pub modrinth_slug : Option<&'a str>,pub modrinth_name : Option<&'a str>,pub modrinth_description : Option<&'a str>,pub modrinth_author : Option<&'a str>,pub modrinth_version : Option<&'a str>,pub hangar_slug : Option<&'a str>,pub hangar_name : Option<&'a str>,pub hangar_description : Option<&'a str>,pub hangar_author : Option<&'a str>,pub hangar_version : Option<&'a str>,pub source_repository_host : Option<&'a str>,pub source_repository_owner : Option<&'a str>,pub source_repository_name : Option<&'a str>,}
impl<'a> From<CommonProjectEntityBorrowed<'a>> for CommonProjectEntity
{
    fn from(CommonProjectEntityBorrowed { id,date_created,date_updated,spigot_id,spigot_slug,spigot_name,spigot_description,spigot_author,spigot_version,spigot_premium,modrinth_id,modrinth_slug,modrinth_name,modrinth_description,modrinth_author,modrinth_version,hangar_slug,hangar_name,hangar_description,hangar_author,hangar_version,source_repository_host,source_repository_owner,source_repository_name,}: CommonProjectEntityBorrowed<'a>) ->
    Self { Self { id,date_created,date_updated,spigot_id,spigot_slug: spigot_slug.map(|v| v.into()),spigot_name: spigot_name.map(|v| v.into()),spigot_description: spigot_description.map(|v| v.into()),spigot_author: spigot_author.map(|v| v.into()),spigot_version: spigot_version.map(|v| v.into()),spigot_premium,modrinth_id: modrinth_id.map(|v| v.into()),modrinth_slug: modrinth_slug.map(|v| v.into()),modrinth_name: modrinth_name.map(|v| v.into()),modrinth_description: modrinth_description.map(|v| v.into()),modrinth_author: modrinth_author.map(|v| v.into()),modrinth_version: modrinth_version.map(|v| v.into()),hangar_slug: hangar_slug.map(|v| v.into()),hangar_name: hangar_name.map(|v| v.into()),hangar_description: hangar_description.map(|v| v.into()),hangar_author: hangar_author.map(|v| v.into()),hangar_version: hangar_version.map(|v| v.into()),source_repository_host: source_repository_host.map(|v| v.into()),source_repository_owner: source_repository_owner.map(|v| v.into()),source_repository_name: source_repository_name.map(|v| v.into()),} }
}pub struct CommonProjectEntityQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> CommonProjectEntityBorrowed,
    mapper: fn(CommonProjectEntityBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> CommonProjectEntityQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(CommonProjectEntityBorrowed) -> R) ->
    CommonProjectEntityQuery<'a,C,R,N>
    {
        CommonProjectEntityQuery
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
}pub fn get_merged_common_projects() -> GetMergedCommonProjectsStmt
{ GetMergedCommonProjectsStmt(cornucopia_async::private::Stmt::new("SELECT
  COALESCE(cs.id, cm.id, ch.id) AS id,
  GREATEST(s.date_created, m.date_created, h.date_created) AS date_created,
  GREATEST(s.date_updated, m.date_updated, h.date_updated) AS date_updated,

  s.id AS spigot_id,
  s.slug AS spigot_slug,
  s.parsed_name AS spigot_name,
  s.description AS spigot_description,
  a.name AS spigot_author,
  s.version_name AS spigot_version,
  s.premium AS spigot_premium,

  m.id AS modrinth_id,
  m.slug AS modrinth_slug,
  m.name AS modrinth_name,
  m.description AS modrinth_description,
  m.author AS modrinth_author,
  m.version_name AS modrinth_version,

  h.slug AS hangar_slug,
  h.name AS hangar_name,
  h.description AS hangar_description,
  h.author AS hangar_author,
  h.version_name AS hangar_version,

  COALESCE(s.source_repository_host, m.source_repository_host, h.source_repository_host) AS source_repository_host,
  COALESCE(s.source_repository_owner, m.source_repository_owner, h.source_repository_owner) AS source_repository_owner,
  COALESCE(s.source_repository_name, m.source_repository_name, h.source_repository_name) AS source_repository_name
FROM
  spigot_resource s
  INNER JOIN spigot_author a
  ON  s.author_id = a.id

  FULL JOIN modrinth_project m
  ON  s.source_repository_host = m.source_repository_host
  AND s.source_repository_owner = m.source_repository_owner
  AND s.source_repository_name = m.source_repository_name

  FULL JOIN hangar_project h
  ON  COALESCE(s.source_repository_host, m.source_repository_host) = h.source_repository_host
  AND COALESCE(s.source_repository_owner, m.source_repository_owner) = h.source_repository_owner
  AND COALESCE(s.source_repository_name, m.source_repository_name) = h.source_repository_name

  LEFT JOIN common_project cs
  ON  s.id = cs.spigot_id

  LEFT JOIN common_project cm
  ON  m.id = cm.modrinth_id

  LEFT JOIN common_project ch
  ON  h.slug = ch.hangar_slug

WHERE
  GREATEST(s.date_updated, m.date_updated, h.date_updated) > $1")) } pub struct
GetMergedCommonProjectsStmt(cornucopia_async::private::Stmt); impl GetMergedCommonProjectsStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
date_updated: &'a time::OffsetDateTime,) -> CommonProjectEntityQuery<'a,C,
CommonProjectEntity, 1>
{
    CommonProjectEntityQuery
    {
        client, params: [date_updated,], stmt: &mut self.0, extractor:
        |row| { CommonProjectEntityBorrowed { id: row.get(0),date_created: row.get(1),date_updated: row.get(2),spigot_id: row.get(3),spigot_slug: row.get(4),spigot_name: row.get(5),spigot_description: row.get(6),spigot_author: row.get(7),spigot_version: row.get(8),spigot_premium: row.get(9),modrinth_id: row.get(10),modrinth_slug: row.get(11),modrinth_name: row.get(12),modrinth_description: row.get(13),modrinth_author: row.get(14),modrinth_version: row.get(15),hangar_slug: row.get(16),hangar_name: row.get(17),hangar_description: row.get(18),hangar_author: row.get(19),hangar_version: row.get(20),source_repository_host: row.get(21),source_repository_owner: row.get(22),source_repository_name: row.get(23),} }, mapper: |it| { <CommonProjectEntity>::from(it) },
    }
} }pub fn upsert_common_project() -> UpsertCommonProjectStmt
{ UpsertCommonProjectStmt(cornucopia_async::private::Stmt::new("INSERT INTO common_project (id, date_created, date_updated, spigot_id, spigot_name, spigot_description, spigot_author, modrinth_id, modrinth_name, modrinth_description, modrinth_author, hangar_slug, hangar_name, hangar_description, hangar_author)
  VALUES (COALESCE($1, nextval('common_project_id_seq')), $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
  ON CONFLICT (id)
  DO UPDATE SET
    date_created = EXCLUDED.date_created,
    date_updated = EXCLUDED.date_updated,
    spigot_id = EXCLUDED.spigot_id,
    spigot_name = EXCLUDED.spigot_name,
    spigot_description = EXCLUDED.spigot_description,
    spigot_author = EXCLUDED.spigot_author,
    modrinth_id = EXCLUDED.modrinth_id,
    modrinth_name = EXCLUDED.modrinth_name,
    modrinth_description = EXCLUDED.modrinth_description,
    modrinth_author = EXCLUDED.modrinth_author,
    hangar_slug = EXCLUDED.hangar_slug,
    hangar_name = EXCLUDED.hangar_name,
    hangar_description = EXCLUDED.hangar_description,
    hangar_author = EXCLUDED.hangar_author")) } pub struct
UpsertCommonProjectStmt(cornucopia_async::private::Stmt); impl UpsertCommonProjectStmt
{ pub async fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::StringSql,T3:
cornucopia_async::StringSql,T4:
cornucopia_async::StringSql,T5:
cornucopia_async::StringSql,T6:
cornucopia_async::StringSql,T7:
cornucopia_async::StringSql,T8:
cornucopia_async::StringSql,T9:
cornucopia_async::StringSql,T10:
cornucopia_async::StringSql,T11:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
id: &'a Option<i64>,date_created: &'a time::OffsetDateTime,date_updated: &'a time::OffsetDateTime,spigot_id: &'a Option<i32>,spigot_name: &'a Option<T1>,spigot_description: &'a Option<T2>,spigot_author: &'a Option<T3>,modrinth_id: &'a Option<T4>,modrinth_name: &'a Option<T5>,modrinth_description: &'a Option<T6>,modrinth_author: &'a Option<T7>,hangar_slug: &'a Option<T8>,hangar_name: &'a Option<T9>,hangar_description: &'a Option<T10>,hangar_author: &'a Option<T11>,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[id,date_created,date_updated,spigot_id,spigot_name,spigot_description,spigot_author,modrinth_id,modrinth_name,modrinth_description,modrinth_author,hangar_slug,hangar_name,hangar_description,hangar_author,]).await
} }impl <'a, C: GenericClient + Send + Sync, T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,T5: cornucopia_async::StringSql,T6: cornucopia_async::StringSql,T7: cornucopia_async::StringSql,T8: cornucopia_async::StringSql,T9: cornucopia_async::StringSql,T10: cornucopia_async::StringSql,T11: cornucopia_async::StringSql,>
cornucopia_async::Params<'a, UpsertCommonProjectParams<T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,>, std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
tokio_postgres::Error>> + Send + 'a>>, C> for UpsertCommonProjectStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    UpsertCommonProjectParams<T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
    tokio_postgres::Error>> + Send + 'a>>
    { Box::pin(self.bind(client, &params.id,&params.date_created,&params.date_updated,&params.spigot_id,&params.spigot_name,&params.spigot_description,&params.spigot_author,&params.modrinth_id,&params.modrinth_name,&params.modrinth_description,&params.modrinth_author,&params.hangar_slug,&params.hangar_name,&params.hangar_description,&params.hangar_author,)) }
}pub fn get_common_projects() -> GetCommonProjectsStmt
{ GetCommonProjectsStmt(cornucopia_async::private::Stmt::new("SELECT
  id,
  date_created,
  date_updated,

  spigot_id,
  NULL AS spigot_slug,
  spigot_name,
  spigot_description,
  spigot_author,
  NULL AS spigot_version,
  FALSE AS spigot_premium,

  modrinth_id,
  NULL AS modrinth_slug,
  modrinth_name,
  modrinth_description,
  modrinth_author,
  NULL AS modrinth_version,

  hangar_slug,
  hangar_name,
  hangar_description,
  hangar_author,
  NULL AS hangar_version,

  NULL AS source_repository_host,
  NULL AS source_repository_owner,
  NULL AS source_repository_name
FROM
  common_project")) } pub struct
GetCommonProjectsStmt(cornucopia_async::private::Stmt); impl GetCommonProjectsStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
) -> CommonProjectEntityQuery<'a,C,
CommonProjectEntity, 0>
{
    CommonProjectEntityQuery
    {
        client, params: [], stmt: &mut self.0, extractor:
        |row| { CommonProjectEntityBorrowed { id: row.get(0),date_created: row.get(1),date_updated: row.get(2),spigot_id: row.get(3),spigot_slug: row.get(4),spigot_name: row.get(5),spigot_description: row.get(6),spigot_author: row.get(7),spigot_version: row.get(8),spigot_premium: row.get(9),modrinth_id: row.get(10),modrinth_slug: row.get(11),modrinth_name: row.get(12),modrinth_description: row.get(13),modrinth_author: row.get(14),modrinth_version: row.get(15),hangar_slug: row.get(16),hangar_name: row.get(17),hangar_description: row.get(18),hangar_author: row.get(19),hangar_version: row.get(20),source_repository_host: row.get(21),source_repository_owner: row.get(22),source_repository_name: row.get(23),} }, mapper: |it| { <CommonProjectEntity>::from(it) },
    }
} }pub fn search_common_projects() -> SearchCommonProjectsStmt
{ SearchCommonProjectsStmt(cornucopia_async::private::Stmt::new("SELECT
  c.id,
  c.date_created,
  c.date_updated,

  c.spigot_id,
  s.slug AS spigot_slug,
  c.spigot_name,
  c.spigot_description,
  c.spigot_author,
  s.version_name AS spigot_version,
  s.premium AS spigot_premium,

  c.modrinth_id,
  m.slug AS modrinth_slug,
  c.modrinth_name,
  c.modrinth_description,
  c.modrinth_author,
  m.version_name AS modrinth_version,

  c.hangar_slug,
  c.hangar_name,
  c.hangar_description,
  c.hangar_author,
  h.version_name AS hangar_version,

  COALESCE(s.source_repository_host, m.source_repository_host, h.source_repository_host) AS source_repository_host,
  COALESCE(s.source_repository_owner, m.source_repository_owner, h.source_repository_owner) AS source_repository_owner,
  COALESCE(s.source_repository_name, m.source_repository_name, h.source_repository_name) AS source_repository_name
FROM
  common_project c
  LEFT JOIN spigot_resource s
  ON c.spigot_id = s.id

  LEFT JOIN modrinth_project m
  ON c.modrinth_id = m.id

  LEFT JOIN hangar_project h
  ON c.hangar_slug = h.slug
WHERE
  CASE $1 IS TRUE AND $2 IS TRUE
    WHEN TRUE THEN spigot_name ILIKE $3
    ELSE FALSE
  END

  OR

  CASE $1 IS TRUE AND $4 IS TRUE
    WHEN TRUE THEN spigot_description ILIKE $3
    ELSE FALSE
  END

  OR

  CASE $1 IS TRUE AND $5 IS TRUE
    WHEN TRUE THEN spigot_author ILIKE $3
    ELSE FALSE
  END

  OR

  CASE $6 IS TRUE AND $2 IS TRUE
    WHEN TRUE THEN modrinth_name ILIKE $3
    ELSE FALSE
  END

  OR

  CASE $6 IS TRUE AND $4 IS TRUE
    WHEN TRUE THEN modrinth_description ILIKE $3
    ELSE FALSE
  END

  OR

  CASE $6 IS TRUE AND $5 IS TRUE
    WHEN TRUE THEN modrinth_author ILIKE $3
    ELSE FALSE
  END

  OR

  CASE $7 IS TRUE AND $2 IS TRUE
    WHEN TRUE THEN hangar_name ILIKE $3
    ELSE FALSE
  END

  OR

  CASE $7 IS TRUE AND $4 IS TRUE
    WHEN TRUE THEN hangar_description ILIKE $3
    ELSE FALSE
  END

  OR

  CASE $7 IS TRUE AND $5 IS TRUE
    WHEN TRUE THEN hangar_author ILIKE $3
    ELSE FALSE
  END

ORDER BY
  (CASE WHEN $8 = 'date_created' THEN c.date_created END) DESC,
  (CASE WHEN $8 = 'date_updated' THEN c.date_updated END) DESC")) } pub struct
SearchCommonProjectsStmt(cornucopia_async::private::Stmt); impl SearchCommonProjectsStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
spigot: &'a bool,name: &'a bool,query: &'a T1,description: &'a bool,author: &'a bool,modrinth: &'a bool,hangar: &'a bool,sort_field: &'a T2,) -> CommonProjectEntityQuery<'a,C,
CommonProjectEntity, 8>
{
    CommonProjectEntityQuery
    {
        client, params: [spigot,name,query,description,author,modrinth,hangar,sort_field,], stmt: &mut self.0, extractor:
        |row| { CommonProjectEntityBorrowed { id: row.get(0),date_created: row.get(1),date_updated: row.get(2),spigot_id: row.get(3),spigot_slug: row.get(4),spigot_name: row.get(5),spigot_description: row.get(6),spigot_author: row.get(7),spigot_version: row.get(8),spigot_premium: row.get(9),modrinth_id: row.get(10),modrinth_slug: row.get(11),modrinth_name: row.get(12),modrinth_description: row.get(13),modrinth_author: row.get(14),modrinth_version: row.get(15),hangar_slug: row.get(16),hangar_name: row.get(17),hangar_description: row.get(18),hangar_author: row.get(19),hangar_version: row.get(20),source_repository_host: row.get(21),source_repository_owner: row.get(22),source_repository_name: row.get(23),} }, mapper: |it| { <CommonProjectEntity>::from(it) },
    }
} }impl <'a, C: GenericClient,T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,> cornucopia_async::Params<'a,
SearchCommonProjectsParams<T1,T2,>, CommonProjectEntityQuery<'a, C,
CommonProjectEntity, 8>, C> for SearchCommonProjectsStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    SearchCommonProjectsParams<T1,T2,>) -> CommonProjectEntityQuery<'a, C,
    CommonProjectEntity, 8>
    { self.bind(client, &params.spigot,&params.name,&params.query,&params.description,&params.author,&params.modrinth,&params.hangar,&params.sort_field,) }
}}pub mod hangar_project
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct UpsertHangarProjectParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,T5: cornucopia_async::StringSql,T6: cornucopia_async::StringSql,T7: cornucopia_async::StringSql,T8: cornucopia_async::StringSql,T9: cornucopia_async::StringSql,T10: cornucopia_async::StringSql,T11: cornucopia_async::StringSql,> { pub slug: T1,pub author: T2,pub name: T3,pub description: T4,pub date_created: time::OffsetDateTime,pub date_updated: time::OffsetDateTime,pub downloads: i32,pub visibility: T5,pub avatar_url: T6,pub version_name: Option<T7>,pub source_url: Option<T8>,pub source_repository_host: Option<T9>,pub source_repository_owner: Option<T10>,pub source_repository_name: Option<T11>,}#[derive( Debug, Clone, PartialEq,)] pub struct HangarProjectEntity
{ pub slug : String,pub author : String,pub name : String,pub description : String,pub date_created : time::OffsetDateTime,pub date_updated : time::OffsetDateTime,pub downloads : i32,pub visibility : String,pub avatar_url : String,pub version_name : Option<String>,pub source_url : Option<String>,pub source_repository_host : Option<String>,pub source_repository_owner : Option<String>,pub source_repository_name : Option<String>,}pub struct HangarProjectEntityBorrowed<'a> { pub slug : &'a str,pub author : &'a str,pub name : &'a str,pub description : &'a str,pub date_created : time::OffsetDateTime,pub date_updated : time::OffsetDateTime,pub downloads : i32,pub visibility : &'a str,pub avatar_url : &'a str,pub version_name : Option<&'a str>,pub source_url : Option<&'a str>,pub source_repository_host : Option<&'a str>,pub source_repository_owner : Option<&'a str>,pub source_repository_name : Option<&'a str>,}
impl<'a> From<HangarProjectEntityBorrowed<'a>> for HangarProjectEntity
{
    fn from(HangarProjectEntityBorrowed { slug,author,name,description,date_created,date_updated,downloads,visibility,avatar_url,version_name,source_url,source_repository_host,source_repository_owner,source_repository_name,}: HangarProjectEntityBorrowed<'a>) ->
    Self { Self { slug: slug.into(),author: author.into(),name: name.into(),description: description.into(),date_created,date_updated,downloads,visibility: visibility.into(),avatar_url: avatar_url.into(),version_name: version_name.map(|v| v.into()),source_url: source_url.map(|v| v.into()),source_repository_host: source_repository_host.map(|v| v.into()),source_repository_owner: source_repository_owner.map(|v| v.into()),source_repository_name: source_repository_name.map(|v| v.into()),} }
}pub struct HangarProjectEntityQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> HangarProjectEntityBorrowed,
    mapper: fn(HangarProjectEntityBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> HangarProjectEntityQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(HangarProjectEntityBorrowed) -> R) ->
    HangarProjectEntityQuery<'a,C,R,N>
    {
        HangarProjectEntityQuery
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
}pub struct TimeOffsetDateTimeQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> time::OffsetDateTime,
    mapper: fn(time::OffsetDateTime) -> T,
} impl<'a, C, T:'a, const N: usize> TimeOffsetDateTimeQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(time::OffsetDateTime) -> R) ->
    TimeOffsetDateTimeQuery<'a,C,R,N>
    {
        TimeOffsetDateTimeQuery
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
}pub fn upsert_hangar_project() -> UpsertHangarProjectStmt
{ UpsertHangarProjectStmt(cornucopia_async::private::Stmt::new("INSERT INTO hangar_project (slug, author, name, description, date_created, date_updated, downloads, visibility, avatar_url, version_name, source_url, source_repository_host, source_repository_owner, source_repository_name)
  VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
  ON CONFLICT (slug)
  DO UPDATE SET
    author = EXCLUDED.author,
    name = EXCLUDED.name,
    description = EXCLUDED.description,
    date_created = EXCLUDED.date_created,
    date_updated = EXCLUDED.date_updated,
    downloads = EXCLUDED.downloads,
    visibility = EXCLUDED.visibility,
    avatar_url = EXCLUDED.avatar_url,
    version_name = EXCLUDED.version_name,
    source_url = EXCLUDED.source_url,
    source_repository_host = EXCLUDED.source_repository_host,
    source_repository_owner = EXCLUDED.source_repository_owner,
    source_repository_name = EXCLUDED.source_repository_name")) } pub struct
UpsertHangarProjectStmt(cornucopia_async::private::Stmt); impl UpsertHangarProjectStmt
{ pub async fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::StringSql,T3:
cornucopia_async::StringSql,T4:
cornucopia_async::StringSql,T5:
cornucopia_async::StringSql,T6:
cornucopia_async::StringSql,T7:
cornucopia_async::StringSql,T8:
cornucopia_async::StringSql,T9:
cornucopia_async::StringSql,T10:
cornucopia_async::StringSql,T11:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
slug: &'a T1,author: &'a T2,name: &'a T3,description: &'a T4,date_created: &'a time::OffsetDateTime,date_updated: &'a time::OffsetDateTime,downloads: &'a i32,visibility: &'a T5,avatar_url: &'a T6,version_name: &'a Option<T7>,source_url: &'a Option<T8>,source_repository_host: &'a Option<T9>,source_repository_owner: &'a Option<T10>,source_repository_name: &'a Option<T11>,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[slug,author,name,description,date_created,date_updated,downloads,visibility,avatar_url,version_name,source_url,source_repository_host,source_repository_owner,source_repository_name,]).await
} }impl <'a, C: GenericClient + Send + Sync, T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,T5: cornucopia_async::StringSql,T6: cornucopia_async::StringSql,T7: cornucopia_async::StringSql,T8: cornucopia_async::StringSql,T9: cornucopia_async::StringSql,T10: cornucopia_async::StringSql,T11: cornucopia_async::StringSql,>
cornucopia_async::Params<'a, UpsertHangarProjectParams<T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,>, std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
tokio_postgres::Error>> + Send + 'a>>, C> for UpsertHangarProjectStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    UpsertHangarProjectParams<T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
    tokio_postgres::Error>> + Send + 'a>>
    { Box::pin(self.bind(client, &params.slug,&params.author,&params.name,&params.description,&params.date_created,&params.date_updated,&params.downloads,&params.visibility,&params.avatar_url,&params.version_name,&params.source_url,&params.source_repository_host,&params.source_repository_owner,&params.source_repository_name,)) }
}pub fn get_hangar_projects() -> GetHangarProjectsStmt
{ GetHangarProjectsStmt(cornucopia_async::private::Stmt::new("SELECT * FROM hangar_project")) } pub struct
GetHangarProjectsStmt(cornucopia_async::private::Stmt); impl GetHangarProjectsStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
) -> HangarProjectEntityQuery<'a,C,
HangarProjectEntity, 0>
{
    HangarProjectEntityQuery
    {
        client, params: [], stmt: &mut self.0, extractor:
        |row| { HangarProjectEntityBorrowed { slug: row.get(0),author: row.get(1),name: row.get(2),description: row.get(3),date_created: row.get(4),date_updated: row.get(5),downloads: row.get(6),visibility: row.get(7),avatar_url: row.get(8),version_name: row.get(9),source_url: row.get(10),source_repository_host: row.get(11),source_repository_owner: row.get(12),source_repository_name: row.get(13),} }, mapper: |it| { <HangarProjectEntity>::from(it) },
    }
} }pub fn get_latest_hangar_project_update_date() -> GetLatestHangarProjectUpdateDateStmt
{ GetLatestHangarProjectUpdateDateStmt(cornucopia_async::private::Stmt::new("SELECT max(date_updated) FROM hangar_project")) } pub struct
GetLatestHangarProjectUpdateDateStmt(cornucopia_async::private::Stmt); impl GetLatestHangarProjectUpdateDateStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
) -> TimeOffsetDateTimeQuery<'a,C,
time::OffsetDateTime, 0>
{
    TimeOffsetDateTimeQuery
    {
        client, params: [], stmt: &mut self.0, extractor:
        |row| { row.get(0) }, mapper: |it| { it },
    }
} }}pub mod modrinth_project
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct UpsertModrinthProjectParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,T5: cornucopia_async::StringSql,T6: cornucopia_async::StringSql,T7: cornucopia_async::StringSql,T8: cornucopia_async::StringSql,T9: cornucopia_async::StringSql,T10: cornucopia_async::StringSql,T11: cornucopia_async::StringSql,T12: cornucopia_async::StringSql,T13: cornucopia_async::StringSql,> { pub id: T1,pub slug: T2,pub name: T3,pub description: T4,pub author: T5,pub date_created: time::OffsetDateTime,pub date_updated: time::OffsetDateTime,pub downloads: i32,pub follows: i32,pub version_id: Option<T6>,pub version_name: Option<T7>,pub icon_url: Option<T8>,pub monetization_status: Option<T9>,pub source_url: Option<T10>,pub source_repository_host: Option<T11>,pub source_repository_owner: Option<T12>,pub source_repository_name: Option<T13>,}#[derive( Debug, Clone, PartialEq,)] pub struct ModrinthProjectEntity
{ pub id : String,pub slug : String,pub name : String,pub description : String,pub author : String,pub date_created : time::OffsetDateTime,pub date_updated : time::OffsetDateTime,pub downloads : i32,pub follows : i32,pub version_id : Option<String>,pub version_name : Option<String>,pub icon_url : Option<String>,pub monetization_status : Option<String>,pub source_url : Option<String>,pub source_repository_host : Option<String>,pub source_repository_owner : Option<String>,pub source_repository_name : Option<String>,}pub struct ModrinthProjectEntityBorrowed<'a> { pub id : &'a str,pub slug : &'a str,pub name : &'a str,pub description : &'a str,pub author : &'a str,pub date_created : time::OffsetDateTime,pub date_updated : time::OffsetDateTime,pub downloads : i32,pub follows : i32,pub version_id : Option<&'a str>,pub version_name : Option<&'a str>,pub icon_url : Option<&'a str>,pub monetization_status : Option<&'a str>,pub source_url : Option<&'a str>,pub source_repository_host : Option<&'a str>,pub source_repository_owner : Option<&'a str>,pub source_repository_name : Option<&'a str>,}
impl<'a> From<ModrinthProjectEntityBorrowed<'a>> for ModrinthProjectEntity
{
    fn from(ModrinthProjectEntityBorrowed { id,slug,name,description,author,date_created,date_updated,downloads,follows,version_id,version_name,icon_url,monetization_status,source_url,source_repository_host,source_repository_owner,source_repository_name,}: ModrinthProjectEntityBorrowed<'a>) ->
    Self { Self { id: id.into(),slug: slug.into(),name: name.into(),description: description.into(),author: author.into(),date_created,date_updated,downloads,follows,version_id: version_id.map(|v| v.into()),version_name: version_name.map(|v| v.into()),icon_url: icon_url.map(|v| v.into()),monetization_status: monetization_status.map(|v| v.into()),source_url: source_url.map(|v| v.into()),source_repository_host: source_repository_host.map(|v| v.into()),source_repository_owner: source_repository_owner.map(|v| v.into()),source_repository_name: source_repository_name.map(|v| v.into()),} }
}pub struct ModrinthProjectEntityQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> ModrinthProjectEntityBorrowed,
    mapper: fn(ModrinthProjectEntityBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> ModrinthProjectEntityQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(ModrinthProjectEntityBorrowed) -> R) ->
    ModrinthProjectEntityQuery<'a,C,R,N>
    {
        ModrinthProjectEntityQuery
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
}pub struct TimeOffsetDateTimeQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> time::OffsetDateTime,
    mapper: fn(time::OffsetDateTime) -> T,
} impl<'a, C, T:'a, const N: usize> TimeOffsetDateTimeQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(time::OffsetDateTime) -> R) ->
    TimeOffsetDateTimeQuery<'a,C,R,N>
    {
        TimeOffsetDateTimeQuery
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
}pub fn upsert_modrinth_project() -> UpsertModrinthProjectStmt
{ UpsertModrinthProjectStmt(cornucopia_async::private::Stmt::new("INSERT INTO modrinth_project (id, slug, name, description, author, date_created, date_updated, downloads, follows, version_id, version_name, icon_url, monetization_status, source_url, source_repository_host, source_repository_owner, source_repository_name)
  VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
  ON CONFLICT(id)
  DO UPDATE SET
    id = EXCLUDED.id,
    slug = EXCLUDED.slug,
    name = EXCLUDED.name,
    description = EXCLUDED.description,
    author = EXCLUDED.author,
    date_created = EXCLUDED.date_created,
    date_updated = EXCLUDED.date_updated,
    downloads = EXCLUDED.downloads,
    follows = EXCLUDED.follows,
    version_id = EXCLUDED.version_id,
    version_name = EXCLUDED.version_name,
    icon_url = EXCLUDED.icon_url,
    monetization_status = EXCLUDED.monetization_status,
    source_url = EXCLUDED.source_url,
    source_repository_host = EXCLUDED.source_repository_host,
    source_repository_owner = EXCLUDED.source_repository_owner,
    source_repository_name = EXCLUDED.source_repository_name")) } pub struct
UpsertModrinthProjectStmt(cornucopia_async::private::Stmt); impl UpsertModrinthProjectStmt
{ pub async fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::StringSql,T3:
cornucopia_async::StringSql,T4:
cornucopia_async::StringSql,T5:
cornucopia_async::StringSql,T6:
cornucopia_async::StringSql,T7:
cornucopia_async::StringSql,T8:
cornucopia_async::StringSql,T9:
cornucopia_async::StringSql,T10:
cornucopia_async::StringSql,T11:
cornucopia_async::StringSql,T12:
cornucopia_async::StringSql,T13:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
id: &'a T1,slug: &'a T2,name: &'a T3,description: &'a T4,author: &'a T5,date_created: &'a time::OffsetDateTime,date_updated: &'a time::OffsetDateTime,downloads: &'a i32,follows: &'a i32,version_id: &'a Option<T6>,version_name: &'a Option<T7>,icon_url: &'a Option<T8>,monetization_status: &'a Option<T9>,source_url: &'a Option<T10>,source_repository_host: &'a Option<T11>,source_repository_owner: &'a Option<T12>,source_repository_name: &'a Option<T13>,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[id,slug,name,description,author,date_created,date_updated,downloads,follows,version_id,version_name,icon_url,monetization_status,source_url,source_repository_host,source_repository_owner,source_repository_name,]).await
} }impl <'a, C: GenericClient + Send + Sync, T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,T5: cornucopia_async::StringSql,T6: cornucopia_async::StringSql,T7: cornucopia_async::StringSql,T8: cornucopia_async::StringSql,T9: cornucopia_async::StringSql,T10: cornucopia_async::StringSql,T11: cornucopia_async::StringSql,T12: cornucopia_async::StringSql,T13: cornucopia_async::StringSql,>
cornucopia_async::Params<'a, UpsertModrinthProjectParams<T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,>, std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
tokio_postgres::Error>> + Send + 'a>>, C> for UpsertModrinthProjectStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    UpsertModrinthProjectParams<T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
    tokio_postgres::Error>> + Send + 'a>>
    { Box::pin(self.bind(client, &params.id,&params.slug,&params.name,&params.description,&params.author,&params.date_created,&params.date_updated,&params.downloads,&params.follows,&params.version_id,&params.version_name,&params.icon_url,&params.monetization_status,&params.source_url,&params.source_repository_host,&params.source_repository_owner,&params.source_repository_name,)) }
}pub fn get_modrinth_projects() -> GetModrinthProjectsStmt
{ GetModrinthProjectsStmt(cornucopia_async::private::Stmt::new("SELECT * FROM modrinth_project")) } pub struct
GetModrinthProjectsStmt(cornucopia_async::private::Stmt); impl GetModrinthProjectsStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
) -> ModrinthProjectEntityQuery<'a,C,
ModrinthProjectEntity, 0>
{
    ModrinthProjectEntityQuery
    {
        client, params: [], stmt: &mut self.0, extractor:
        |row| { ModrinthProjectEntityBorrowed { id: row.get(0),slug: row.get(1),name: row.get(2),description: row.get(3),author: row.get(4),date_created: row.get(5),date_updated: row.get(6),downloads: row.get(7),follows: row.get(8),version_id: row.get(9),version_name: row.get(10),icon_url: row.get(11),monetization_status: row.get(12),source_url: row.get(13),source_repository_host: row.get(14),source_repository_owner: row.get(15),source_repository_name: row.get(16),} }, mapper: |it| { <ModrinthProjectEntity>::from(it) },
    }
} }pub fn get_latest_modrinth_project_update_date() -> GetLatestModrinthProjectUpdateDateStmt
{ GetLatestModrinthProjectUpdateDateStmt(cornucopia_async::private::Stmt::new("SELECT max(date_updated) FROM modrinth_project")) } pub struct
GetLatestModrinthProjectUpdateDateStmt(cornucopia_async::private::Stmt); impl GetLatestModrinthProjectUpdateDateStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
) -> TimeOffsetDateTimeQuery<'a,C,
time::OffsetDateTime, 0>
{
    TimeOffsetDateTimeQuery
    {
        client, params: [], stmt: &mut self.0, extractor:
        |row| { row.get(0) }, mapper: |it| { it },
    }
} }}pub mod spigot_author
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct InsertSpigotAuthorParams<T1: cornucopia_async::StringSql,> { pub id: i32,pub name: T1,}#[derive( Debug, Clone, PartialEq,)] pub struct SpigotAuthorEntity
{ pub id : i32,pub name : String,}pub struct SpigotAuthorEntityBorrowed<'a> { pub id : i32,pub name : &'a str,}
impl<'a> From<SpigotAuthorEntityBorrowed<'a>> for SpigotAuthorEntity
{
    fn from(SpigotAuthorEntityBorrowed { id,name,}: SpigotAuthorEntityBorrowed<'a>) ->
    Self { Self { id,name: name.into(),} }
}pub struct SpigotAuthorEntityQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> SpigotAuthorEntityBorrowed,
    mapper: fn(SpigotAuthorEntityBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> SpigotAuthorEntityQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(SpigotAuthorEntityBorrowed) -> R) ->
    SpigotAuthorEntityQuery<'a,C,R,N>
    {
        SpigotAuthorEntityQuery
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
}pub struct I32Query<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> i32,
    mapper: fn(i32) -> T,
} impl<'a, C, T:'a, const N: usize> I32Query<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(i32) -> R) ->
    I32Query<'a,C,R,N>
    {
        I32Query
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
}pub fn insert_spigot_author() -> InsertSpigotAuthorStmt
{ InsertSpigotAuthorStmt(cornucopia_async::private::Stmt::new("INSERT INTO spigot_author (id, name) VALUES ($1, $2)")) } pub struct
InsertSpigotAuthorStmt(cornucopia_async::private::Stmt); impl InsertSpigotAuthorStmt
{ pub async fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
id: &'a i32,name: &'a T1,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[id,name,]).await
} }impl <'a, C: GenericClient + Send + Sync, T1: cornucopia_async::StringSql,>
cornucopia_async::Params<'a, InsertSpigotAuthorParams<T1,>, std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
tokio_postgres::Error>> + Send + 'a>>, C> for InsertSpigotAuthorStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    InsertSpigotAuthorParams<T1,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
    tokio_postgres::Error>> + Send + 'a>>
    { Box::pin(self.bind(client, &params.id,&params.name,)) }
}pub fn get_spigot_authors() -> GetSpigotAuthorsStmt
{ GetSpigotAuthorsStmt(cornucopia_async::private::Stmt::new("SELECT id, name FROM spigot_author")) } pub struct
GetSpigotAuthorsStmt(cornucopia_async::private::Stmt); impl GetSpigotAuthorsStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
) -> SpigotAuthorEntityQuery<'a,C,
SpigotAuthorEntity, 0>
{
    SpigotAuthorEntityQuery
    {
        client, params: [], stmt: &mut self.0, extractor:
        |row| { SpigotAuthorEntityBorrowed { id: row.get(0),name: row.get(1),} }, mapper: |it| { <SpigotAuthorEntity>::from(it) },
    }
} }pub fn get_highest_spigot_author_id() -> GetHighestSpigotAuthorIdStmt
{ GetHighestSpigotAuthorIdStmt(cornucopia_async::private::Stmt::new("SELECT max(id) from spigot_author")) } pub struct
GetHighestSpigotAuthorIdStmt(cornucopia_async::private::Stmt); impl GetHighestSpigotAuthorIdStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
) -> I32Query<'a,C,
i32, 0>
{
    I32Query
    {
        client, params: [], stmt: &mut self.0, extractor:
        |row| { row.get(0) }, mapper: |it| { it },
    }
} }}pub mod spigot_resource
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct UpsertSpigotResourceParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,T5: cornucopia_async::StringSql,T6: cornucopia_async::StringSql,T7: cornucopia_async::StringSql,T8: cornucopia_async::StringSql,T9: cornucopia_async::StringSql,> { pub id: i32,pub name: T1,pub parsed_name: Option<T2>,pub description: T3,pub slug: T4,pub date_created: time::OffsetDateTime,pub date_updated: time::OffsetDateTime,pub downloads: i32,pub likes: i32,pub author_id: i32,pub version_id: i32,pub version_name: Option<T5>,pub premium: bool,pub source_url: Option<T6>,pub source_repository_host: Option<T7>,pub source_repository_owner: Option<T8>,pub source_repository_name: Option<T9>,}#[derive( Debug, Clone, PartialEq,)] pub struct SpigotResourceEntity
{ pub id : i32,pub name : String,pub parsed_name : Option<String>,pub description : String,pub slug : String,pub date_created : time::OffsetDateTime,pub date_updated : time::OffsetDateTime,pub downloads : i32,pub likes : i32,pub author_id : i32,pub version_id : i32,pub version_name : Option<String>,pub premium : bool,pub source_url : Option<String>,pub source_repository_host : Option<String>,pub source_repository_owner : Option<String>,pub source_repository_name : Option<String>,}pub struct SpigotResourceEntityBorrowed<'a> { pub id : i32,pub name : &'a str,pub parsed_name : Option<&'a str>,pub description : &'a str,pub slug : &'a str,pub date_created : time::OffsetDateTime,pub date_updated : time::OffsetDateTime,pub downloads : i32,pub likes : i32,pub author_id : i32,pub version_id : i32,pub version_name : Option<&'a str>,pub premium : bool,pub source_url : Option<&'a str>,pub source_repository_host : Option<&'a str>,pub source_repository_owner : Option<&'a str>,pub source_repository_name : Option<&'a str>,}
impl<'a> From<SpigotResourceEntityBorrowed<'a>> for SpigotResourceEntity
{
    fn from(SpigotResourceEntityBorrowed { id,name,parsed_name,description,slug,date_created,date_updated,downloads,likes,author_id,version_id,version_name,premium,source_url,source_repository_host,source_repository_owner,source_repository_name,}: SpigotResourceEntityBorrowed<'a>) ->
    Self { Self { id,name: name.into(),parsed_name: parsed_name.map(|v| v.into()),description: description.into(),slug: slug.into(),date_created,date_updated,downloads,likes,author_id,version_id,version_name: version_name.map(|v| v.into()),premium,source_url: source_url.map(|v| v.into()),source_repository_host: source_repository_host.map(|v| v.into()),source_repository_owner: source_repository_owner.map(|v| v.into()),source_repository_name: source_repository_name.map(|v| v.into()),} }
}pub struct SpigotResourceEntityQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> SpigotResourceEntityBorrowed,
    mapper: fn(SpigotResourceEntityBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> SpigotResourceEntityQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(SpigotResourceEntityBorrowed) -> R) ->
    SpigotResourceEntityQuery<'a,C,R,N>
    {
        SpigotResourceEntityQuery
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
}pub struct TimeOffsetDateTimeQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> time::OffsetDateTime,
    mapper: fn(time::OffsetDateTime) -> T,
} impl<'a, C, T:'a, const N: usize> TimeOffsetDateTimeQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(time::OffsetDateTime) -> R) ->
    TimeOffsetDateTimeQuery<'a,C,R,N>
    {
        TimeOffsetDateTimeQuery
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
}pub fn upsert_spigot_resource() -> UpsertSpigotResourceStmt
{ UpsertSpigotResourceStmt(cornucopia_async::private::Stmt::new("INSERT INTO spigot_resource (id, name, parsed_name, description, slug, date_created, date_updated, downloads, likes, author_id, version_id, version_name, premium, source_url, source_repository_host, source_repository_owner, source_repository_name)
  VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
  ON CONFLICT (id)
  DO UPDATE SET
    name = EXCLUDED.name,
    parsed_name = EXCLUDED.parsed_name,
    description = EXCLUDED.description,
    slug = EXCLUDED.slug,
    date_created = EXCLUDED.date_created,
    date_updated = EXCLUDED.date_updated,
    downloads = EXCLUDED.downloads,
    likes = EXCLUDED.likes,
    author_id = EXCLUDED.author_id,
    version_id = EXCLUDED.version_id,
    version_name = EXCLUDED.version_name,
    premium = EXCLUDED.premium,
    source_url = EXCLUDED.source_url,
    source_repository_host = EXCLUDED.source_repository_host,
    source_repository_owner = EXCLUDED.source_repository_owner,
    source_repository_name = EXCLUDED.source_repository_name")) } pub struct
UpsertSpigotResourceStmt(cornucopia_async::private::Stmt); impl UpsertSpigotResourceStmt
{ pub async fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::StringSql,T3:
cornucopia_async::StringSql,T4:
cornucopia_async::StringSql,T5:
cornucopia_async::StringSql,T6:
cornucopia_async::StringSql,T7:
cornucopia_async::StringSql,T8:
cornucopia_async::StringSql,T9:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
id: &'a i32,name: &'a T1,parsed_name: &'a Option<T2>,description: &'a T3,slug: &'a T4,date_created: &'a time::OffsetDateTime,date_updated: &'a time::OffsetDateTime,downloads: &'a i32,likes: &'a i32,author_id: &'a i32,version_id: &'a i32,version_name: &'a Option<T5>,premium: &'a bool,source_url: &'a Option<T6>,source_repository_host: &'a Option<T7>,source_repository_owner: &'a Option<T8>,source_repository_name: &'a Option<T9>,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[id,name,parsed_name,description,slug,date_created,date_updated,downloads,likes,author_id,version_id,version_name,premium,source_url,source_repository_host,source_repository_owner,source_repository_name,]).await
} }impl <'a, C: GenericClient + Send + Sync, T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,T5: cornucopia_async::StringSql,T6: cornucopia_async::StringSql,T7: cornucopia_async::StringSql,T8: cornucopia_async::StringSql,T9: cornucopia_async::StringSql,>
cornucopia_async::Params<'a, UpsertSpigotResourceParams<T1,T2,T3,T4,T5,T6,T7,T8,T9,>, std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
tokio_postgres::Error>> + Send + 'a>>, C> for UpsertSpigotResourceStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    UpsertSpigotResourceParams<T1,T2,T3,T4,T5,T6,T7,T8,T9,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
    tokio_postgres::Error>> + Send + 'a>>
    { Box::pin(self.bind(client, &params.id,&params.name,&params.parsed_name,&params.description,&params.slug,&params.date_created,&params.date_updated,&params.downloads,&params.likes,&params.author_id,&params.version_id,&params.version_name,&params.premium,&params.source_url,&params.source_repository_host,&params.source_repository_owner,&params.source_repository_name,)) }
}pub fn get_spigot_resources() -> GetSpigotResourcesStmt
{ GetSpigotResourcesStmt(cornucopia_async::private::Stmt::new("SELECT * FROM spigot_resource")) } pub struct
GetSpigotResourcesStmt(cornucopia_async::private::Stmt); impl GetSpigotResourcesStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
) -> SpigotResourceEntityQuery<'a,C,
SpigotResourceEntity, 0>
{
    SpigotResourceEntityQuery
    {
        client, params: [], stmt: &mut self.0, extractor:
        |row| { SpigotResourceEntityBorrowed { id: row.get(0),name: row.get(1),parsed_name: row.get(2),description: row.get(3),slug: row.get(4),date_created: row.get(5),date_updated: row.get(6),downloads: row.get(7),likes: row.get(8),author_id: row.get(9),version_id: row.get(10),version_name: row.get(11),premium: row.get(12),source_url: row.get(13),source_repository_host: row.get(14),source_repository_owner: row.get(15),source_repository_name: row.get(16),} }, mapper: |it| { <SpigotResourceEntity>::from(it) },
    }
} }pub fn get_latest_spigot_resource_update_date() -> GetLatestSpigotResourceUpdateDateStmt
{ GetLatestSpigotResourceUpdateDateStmt(cornucopia_async::private::Stmt::new("SELECT max(date_updated) FROM spigot_resource")) } pub struct
GetLatestSpigotResourceUpdateDateStmt(cornucopia_async::private::Stmt); impl GetLatestSpigotResourceUpdateDateStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
) -> TimeOffsetDateTimeQuery<'a,C,
time::OffsetDateTime, 0>
{
    TimeOffsetDateTimeQuery
    {
        client, params: [], stmt: &mut self.0, extractor:
        |row| { row.get(0) }, mapper: |it| { it },
    }
} }}}