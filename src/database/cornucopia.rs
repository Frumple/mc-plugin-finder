// This file was generated with `cornucopia`. Do not modify.

#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod types { pub mod public { #[derive( Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)] pub enum IngestLogAction { Populate,Update,Refresh,}impl<'a> postgres_types::ToSql for IngestLogAction
{
    fn
    to_sql(&self, ty: &postgres_types::Type, buf: &mut
    postgres_types::private::BytesMut,) -> Result<postgres_types::IsNull,
    Box<dyn std::error::Error + Sync + Send>,>
    {
        let s = match *self { IngestLogAction::Populate => "Populate",IngestLogAction::Update => "Update",IngestLogAction::Refresh => "Refresh",};
        buf.extend_from_slice(s.as_bytes());
        std::result::Result::Ok(postgres_types::IsNull::No)
    } fn accepts(ty: &postgres_types::Type) -> bool
    {
        if ty.name() != "ingest_log_action" { return false; } match *ty.kind()
        {
            postgres_types::Kind::Enum(ref variants) =>
            {
                if variants.len() != 3 { return false; }
                variants.iter().all(|v| match &**v
                { "Populate" => true,"Update" => true,"Refresh" => true,_ => false, })
            } _ => false,
        }
    } fn
    to_sql_checked(&self, ty: &postgres_types::Type, out: &mut
    postgres_types::private::BytesMut,) -> Result<postgres_types::IsNull,
    Box<dyn std::error::Error + Sync + Send>>
    { postgres_types::__to_sql_checked(self, ty, out) }
} impl<'a> postgres_types::FromSql<'a> for IngestLogAction
{
    fn from_sql(ty: &postgres_types::Type, buf: &'a [u8],) ->
    Result<IngestLogAction, Box<dyn std::error::Error + Sync + Send>,>
    {
        match std::str::from_utf8(buf)?
        {
            "Populate" => Ok(IngestLogAction::Populate),"Update" => Ok(IngestLogAction::Update),"Refresh" => Ok(IngestLogAction::Refresh),s =>
            Result::Err(Into::into(format!("invalid variant `{}`", s))),
        }
    } fn accepts(ty: &postgres_types::Type) -> bool
    {
        if ty.name() != "ingest_log_action" { return false; } match *ty.kind()
        {
            postgres_types::Kind::Enum(ref variants) =>
            {
                if variants.len() != 3 { return false; }
                variants.iter().all(|v| match &**v
                { "Populate" => true,"Update" => true,"Refresh" => true,_ => false, })
            } _ => false,
        }
    }
}#[derive( Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)] pub enum IngestLogRepository { Spigot,Modrinth,Hangar,Common,}impl<'a> postgres_types::ToSql for IngestLogRepository
{
    fn
    to_sql(&self, ty: &postgres_types::Type, buf: &mut
    postgres_types::private::BytesMut,) -> Result<postgres_types::IsNull,
    Box<dyn std::error::Error + Sync + Send>,>
    {
        let s = match *self { IngestLogRepository::Spigot => "Spigot",IngestLogRepository::Modrinth => "Modrinth",IngestLogRepository::Hangar => "Hangar",IngestLogRepository::Common => "Common",};
        buf.extend_from_slice(s.as_bytes());
        std::result::Result::Ok(postgres_types::IsNull::No)
    } fn accepts(ty: &postgres_types::Type) -> bool
    {
        if ty.name() != "ingest_log_repository" { return false; } match *ty.kind()
        {
            postgres_types::Kind::Enum(ref variants) =>
            {
                if variants.len() != 4 { return false; }
                variants.iter().all(|v| match &**v
                { "Spigot" => true,"Modrinth" => true,"Hangar" => true,"Common" => true,_ => false, })
            } _ => false,
        }
    } fn
    to_sql_checked(&self, ty: &postgres_types::Type, out: &mut
    postgres_types::private::BytesMut,) -> Result<postgres_types::IsNull,
    Box<dyn std::error::Error + Sync + Send>>
    { postgres_types::__to_sql_checked(self, ty, out) }
} impl<'a> postgres_types::FromSql<'a> for IngestLogRepository
{
    fn from_sql(ty: &postgres_types::Type, buf: &'a [u8],) ->
    Result<IngestLogRepository, Box<dyn std::error::Error + Sync + Send>,>
    {
        match std::str::from_utf8(buf)?
        {
            "Spigot" => Ok(IngestLogRepository::Spigot),"Modrinth" => Ok(IngestLogRepository::Modrinth),"Hangar" => Ok(IngestLogRepository::Hangar),"Common" => Ok(IngestLogRepository::Common),s =>
            Result::Err(Into::into(format!("invalid variant `{}`", s))),
        }
    } fn accepts(ty: &postgres_types::Type) -> bool
    {
        if ty.name() != "ingest_log_repository" { return false; } match *ty.kind()
        {
            postgres_types::Kind::Enum(ref variants) =>
            {
                if variants.len() != 4 { return false; }
                variants.iter().all(|v| match &**v
                { "Spigot" => true,"Modrinth" => true,"Hangar" => true,"Common" => true,_ => false, })
            } _ => false,
        }
    }
}#[derive( Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)] pub enum IngestLogItem { Author,Resource,Project,Version,}impl<'a> postgres_types::ToSql for IngestLogItem
{
    fn
    to_sql(&self, ty: &postgres_types::Type, buf: &mut
    postgres_types::private::BytesMut,) -> Result<postgres_types::IsNull,
    Box<dyn std::error::Error + Sync + Send>,>
    {
        let s = match *self { IngestLogItem::Author => "Author",IngestLogItem::Resource => "Resource",IngestLogItem::Project => "Project",IngestLogItem::Version => "Version",};
        buf.extend_from_slice(s.as_bytes());
        std::result::Result::Ok(postgres_types::IsNull::No)
    } fn accepts(ty: &postgres_types::Type) -> bool
    {
        if ty.name() != "ingest_log_item" { return false; } match *ty.kind()
        {
            postgres_types::Kind::Enum(ref variants) =>
            {
                if variants.len() != 4 { return false; }
                variants.iter().all(|v| match &**v
                { "Author" => true,"Resource" => true,"Project" => true,"Version" => true,_ => false, })
            } _ => false,
        }
    } fn
    to_sql_checked(&self, ty: &postgres_types::Type, out: &mut
    postgres_types::private::BytesMut,) -> Result<postgres_types::IsNull,
    Box<dyn std::error::Error + Sync + Send>>
    { postgres_types::__to_sql_checked(self, ty, out) }
} impl<'a> postgres_types::FromSql<'a> for IngestLogItem
{
    fn from_sql(ty: &postgres_types::Type, buf: &'a [u8],) ->
    Result<IngestLogItem, Box<dyn std::error::Error + Sync + Send>,>
    {
        match std::str::from_utf8(buf)?
        {
            "Author" => Ok(IngestLogItem::Author),"Resource" => Ok(IngestLogItem::Resource),"Project" => Ok(IngestLogItem::Project),"Version" => Ok(IngestLogItem::Version),s =>
            Result::Err(Into::into(format!("invalid variant `{}`", s))),
        }
    } fn accepts(ty: &postgres_types::Type) -> bool
    {
        if ty.name() != "ingest_log_item" { return false; } match *ty.kind()
        {
            postgres_types::Kind::Enum(ref variants) =>
            {
                if variants.len() != 4 { return false; }
                variants.iter().all(|v| match &**v
                { "Author" => true,"Resource" => true,"Project" => true,"Version" => true,_ => false, })
            } _ => false,
        }
    }
} }}#[allow(clippy::all, clippy::pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod queries
{ pub mod common_project
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;pub struct I64Query<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> i64,
    mapper: fn(i64) -> T,
} impl<'a, C, T:'a, const N: usize> I64Query<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(i64) -> R) ->
    I64Query<'a,C,R,N>
    {
        I64Query
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
}#[derive( Debug, Clone, PartialEq,)] pub struct CommonProjectEntity
{ pub spigot_id : Option<i32>,pub spigot_slug : Option<String>,pub spigot_name : Option<String>,pub spigot_description : Option<String>,pub spigot_author : Option<String>,pub spigot_version : Option<String>,pub spigot_premium : Option<bool>,pub spigot_abandoned : Option<bool>,pub spigot_icon_data : Option<String>,pub spigot_date_created : Option<time::OffsetDateTime>,pub spigot_date_updated : Option<time::OffsetDateTime>,pub spigot_latest_minecraft_version : Option<String>,pub spigot_downloads : Option<i32>,pub spigot_likes : Option<i32>,pub modrinth_id : Option<String>,pub modrinth_slug : Option<String>,pub modrinth_name : Option<String>,pub modrinth_description : Option<String>,pub modrinth_author : Option<String>,pub modrinth_version : Option<String>,pub modrinth_status : Option<String>,pub modrinth_icon_url : Option<String>,pub modrinth_date_created : Option<time::OffsetDateTime>,pub modrinth_date_updated : Option<time::OffsetDateTime>,pub modrinth_latest_minecraft_version : Option<String>,pub modrinth_downloads : Option<i32>,pub modrinth_follows : Option<i32>,pub hangar_slug : Option<String>,pub hangar_name : Option<String>,pub hangar_description : Option<String>,pub hangar_author : Option<String>,pub hangar_version : Option<String>,pub hangar_icon_url : Option<String>,pub hangar_date_created : Option<time::OffsetDateTime>,pub hangar_date_updated : Option<time::OffsetDateTime>,pub hangar_latest_minecraft_version : Option<String>,pub hangar_downloads : Option<i32>,pub hangar_stars : Option<i32>,pub hangar_watchers : Option<i32>,pub source_repository_host : Option<String>,pub source_repository_name : Option<String>,pub source_repository_owner : Option<String>,}pub struct CommonProjectEntityBorrowed<'a> { pub spigot_id : Option<i32>,pub spigot_slug : Option<&'a str>,pub spigot_name : Option<&'a str>,pub spigot_description : Option<&'a str>,pub spigot_author : Option<&'a str>,pub spigot_version : Option<&'a str>,pub spigot_premium : Option<bool>,pub spigot_abandoned : Option<bool>,pub spigot_icon_data : Option<&'a str>,pub spigot_date_created : Option<time::OffsetDateTime>,pub spigot_date_updated : Option<time::OffsetDateTime>,pub spigot_latest_minecraft_version : Option<&'a str>,pub spigot_downloads : Option<i32>,pub spigot_likes : Option<i32>,pub modrinth_id : Option<&'a str>,pub modrinth_slug : Option<&'a str>,pub modrinth_name : Option<&'a str>,pub modrinth_description : Option<&'a str>,pub modrinth_author : Option<&'a str>,pub modrinth_version : Option<&'a str>,pub modrinth_status : Option<&'a str>,pub modrinth_icon_url : Option<&'a str>,pub modrinth_date_created : Option<time::OffsetDateTime>,pub modrinth_date_updated : Option<time::OffsetDateTime>,pub modrinth_latest_minecraft_version : Option<&'a str>,pub modrinth_downloads : Option<i32>,pub modrinth_follows : Option<i32>,pub hangar_slug : Option<&'a str>,pub hangar_name : Option<&'a str>,pub hangar_description : Option<&'a str>,pub hangar_author : Option<&'a str>,pub hangar_version : Option<&'a str>,pub hangar_icon_url : Option<&'a str>,pub hangar_date_created : Option<time::OffsetDateTime>,pub hangar_date_updated : Option<time::OffsetDateTime>,pub hangar_latest_minecraft_version : Option<&'a str>,pub hangar_downloads : Option<i32>,pub hangar_stars : Option<i32>,pub hangar_watchers : Option<i32>,pub source_repository_host : Option<&'a str>,pub source_repository_name : Option<&'a str>,pub source_repository_owner : Option<&'a str>,}
impl<'a> From<CommonProjectEntityBorrowed<'a>> for CommonProjectEntity
{
    fn from(CommonProjectEntityBorrowed { spigot_id,spigot_slug,spigot_name,spigot_description,spigot_author,spigot_version,spigot_premium,spigot_abandoned,spigot_icon_data,spigot_date_created,spigot_date_updated,spigot_latest_minecraft_version,spigot_downloads,spigot_likes,modrinth_id,modrinth_slug,modrinth_name,modrinth_description,modrinth_author,modrinth_version,modrinth_status,modrinth_icon_url,modrinth_date_created,modrinth_date_updated,modrinth_latest_minecraft_version,modrinth_downloads,modrinth_follows,hangar_slug,hangar_name,hangar_description,hangar_author,hangar_version,hangar_icon_url,hangar_date_created,hangar_date_updated,hangar_latest_minecraft_version,hangar_downloads,hangar_stars,hangar_watchers,source_repository_host,source_repository_name,source_repository_owner,}: CommonProjectEntityBorrowed<'a>) ->
    Self { Self { spigot_id,spigot_slug: spigot_slug.map(|v| v.into()),spigot_name: spigot_name.map(|v| v.into()),spigot_description: spigot_description.map(|v| v.into()),spigot_author: spigot_author.map(|v| v.into()),spigot_version: spigot_version.map(|v| v.into()),spigot_premium,spigot_abandoned,spigot_icon_data: spigot_icon_data.map(|v| v.into()),spigot_date_created,spigot_date_updated,spigot_latest_minecraft_version: spigot_latest_minecraft_version.map(|v| v.into()),spigot_downloads,spigot_likes,modrinth_id: modrinth_id.map(|v| v.into()),modrinth_slug: modrinth_slug.map(|v| v.into()),modrinth_name: modrinth_name.map(|v| v.into()),modrinth_description: modrinth_description.map(|v| v.into()),modrinth_author: modrinth_author.map(|v| v.into()),modrinth_version: modrinth_version.map(|v| v.into()),modrinth_status: modrinth_status.map(|v| v.into()),modrinth_icon_url: modrinth_icon_url.map(|v| v.into()),modrinth_date_created,modrinth_date_updated,modrinth_latest_minecraft_version: modrinth_latest_minecraft_version.map(|v| v.into()),modrinth_downloads,modrinth_follows,hangar_slug: hangar_slug.map(|v| v.into()),hangar_name: hangar_name.map(|v| v.into()),hangar_description: hangar_description.map(|v| v.into()),hangar_author: hangar_author.map(|v| v.into()),hangar_version: hangar_version.map(|v| v.into()),hangar_icon_url: hangar_icon_url.map(|v| v.into()),hangar_date_created,hangar_date_updated,hangar_latest_minecraft_version: hangar_latest_minecraft_version.map(|v| v.into()),hangar_downloads,hangar_stars,hangar_watchers,source_repository_host: source_repository_host.map(|v| v.into()),source_repository_name: source_repository_name.map(|v| v.into()),source_repository_owner: source_repository_owner.map(|v| v.into()),} }
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
}pub fn refresh_common_projects() -> RefreshCommonProjectsStmt
{ RefreshCommonProjectsStmt(cornucopia_async::private::Stmt::new("REFRESH MATERIALIZED VIEW common_project")) } pub struct
RefreshCommonProjectsStmt(cornucopia_async::private::Stmt); impl RefreshCommonProjectsStmt
{ pub async fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[]).await
} }pub fn get_common_project_count() -> GetCommonProjectCountStmt
{ GetCommonProjectCountStmt(cornucopia_async::private::Stmt::new("SELECT COUNT(*) FROM common_project")) } pub struct
GetCommonProjectCountStmt(cornucopia_async::private::Stmt); impl GetCommonProjectCountStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
) -> I64Query<'a,C,
i64, 0>
{
    I64Query
    {
        client, params: [], stmt: &mut self.0, extractor:
        |row| { row.get(0) }, mapper: |it| { it },
    }
} }pub fn get_common_projects() -> GetCommonProjectsStmt
{ GetCommonProjectsStmt(cornucopia_async::private::Stmt::new("SELECT
  spigot_id,
  spigot_slug,
  spigot_name,
  spigot_description,
  spigot_author,
  spigot_version,
  spigot_premium,
  spigot_abandoned,
  spigot_icon_data,
  spigot_date_created,
  spigot_date_updated,
  spigot_latest_minecraft_version,
  spigot_downloads,
  spigot_likes,

  modrinth_id,
  modrinth_slug,
  modrinth_name,
  modrinth_description,
  modrinth_author,
  modrinth_version,
  modrinth_status,
  modrinth_icon_url,
  modrinth_date_created,
  modrinth_date_updated,
  modrinth_latest_minecraft_version,
  modrinth_downloads,
  modrinth_follows,

  hangar_slug,
  hangar_name,
  hangar_description,
  hangar_author,
  hangar_version,
  hangar_icon_url,
  hangar_date_created,
  hangar_date_updated,
  hangar_latest_minecraft_version,
  hangar_downloads,
  hangar_stars,
  hangar_watchers,

  source_repository_host,
  source_repository_name,
  source_repository_owner
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
        |row| { CommonProjectEntityBorrowed { spigot_id: row.get(0),spigot_slug: row.get(1),spigot_name: row.get(2),spigot_description: row.get(3),spigot_author: row.get(4),spigot_version: row.get(5),spigot_premium: row.get(6),spigot_abandoned: row.get(7),spigot_icon_data: row.get(8),spigot_date_created: row.get(9),spigot_date_updated: row.get(10),spigot_latest_minecraft_version: row.get(11),spigot_downloads: row.get(12),spigot_likes: row.get(13),modrinth_id: row.get(14),modrinth_slug: row.get(15),modrinth_name: row.get(16),modrinth_description: row.get(17),modrinth_author: row.get(18),modrinth_version: row.get(19),modrinth_status: row.get(20),modrinth_icon_url: row.get(21),modrinth_date_created: row.get(22),modrinth_date_updated: row.get(23),modrinth_latest_minecraft_version: row.get(24),modrinth_downloads: row.get(25),modrinth_follows: row.get(26),hangar_slug: row.get(27),hangar_name: row.get(28),hangar_description: row.get(29),hangar_author: row.get(30),hangar_version: row.get(31),hangar_icon_url: row.get(32),hangar_date_created: row.get(33),hangar_date_updated: row.get(34),hangar_latest_minecraft_version: row.get(35),hangar_downloads: row.get(36),hangar_stars: row.get(37),hangar_watchers: row.get(38),source_repository_host: row.get(39),source_repository_name: row.get(40),source_repository_owner: row.get(41),} }, mapper: |it| { <CommonProjectEntity>::from(it) },
    }
} }}pub mod fix_upstream_errors
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;pub fn remove_incorrect_source_repository_host_owner_and_name_from_spigot_resources() -> RemoveIncorrectSourceRepositoryHostOwnerAndNameFromSpigotResourcesStmt
{ RemoveIncorrectSourceRepositoryHostOwnerAndNameFromSpigotResourcesStmt(cornucopia_async::private::Stmt::new("UPDATE spigot_resource
SET source_repository_host = NULL, source_repository_owner = NULL, source_repository_name = NULL
WHERE id IN (25773, 82123, 97659, 119724)")) } pub struct
RemoveIncorrectSourceRepositoryHostOwnerAndNameFromSpigotResourcesStmt(cornucopia_async::private::Stmt); impl RemoveIncorrectSourceRepositoryHostOwnerAndNameFromSpigotResourcesStmt
{ pub async fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[]).await
} }pub fn add_source_repository_id_to_noble_whitelist_discord_spigot_resource() -> AddSourceRepositoryIdToNobleWhitelistDiscordSpigotResourceStmt
{ AddSourceRepositoryIdToNobleWhitelistDiscordSpigotResourceStmt(cornucopia_async::private::Stmt::new("UPDATE spigot_resource
SET source_repository_id = 'noble-whitelist-discord'
WHERE id = 113896")) } pub struct
AddSourceRepositoryIdToNobleWhitelistDiscordSpigotResourceStmt(cornucopia_async::private::Stmt); impl AddSourceRepositoryIdToNobleWhitelistDiscordSpigotResourceStmt
{ pub async fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[]).await
} }pub fn add_source_repository_id_to_noble_whitelist_discord_modrinth_project() -> AddSourceRepositoryIdToNobleWhitelistDiscordModrinthProjectStmt
{ AddSourceRepositoryIdToNobleWhitelistDiscordModrinthProjectStmt(cornucopia_async::private::Stmt::new("UPDATE modrinth_project
SET source_repository_id = 'noble-whitelist-discord'
WHERE id = 'WWbtvBwl'")) } pub struct
AddSourceRepositoryIdToNobleWhitelistDiscordModrinthProjectStmt(cornucopia_async::private::Stmt); impl AddSourceRepositoryIdToNobleWhitelistDiscordModrinthProjectStmt
{ pub async fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[]).await
} }pub fn add_source_repository_id_to_noble_whitelist_discord_hangar_project() -> AddSourceRepositoryIdToNobleWhitelistDiscordHangarProjectStmt
{ AddSourceRepositoryIdToNobleWhitelistDiscordHangarProjectStmt(cornucopia_async::private::Stmt::new("UPDATE hangar_project
SET source_repository_id = 'noble-whitelist-discord'
WHERE slug = 'NobleWhitelistDiscord'")) } pub struct
AddSourceRepositoryIdToNobleWhitelistDiscordHangarProjectStmt(cornucopia_async::private::Stmt); impl AddSourceRepositoryIdToNobleWhitelistDiscordHangarProjectStmt
{ pub async fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[]).await
} }pub fn add_source_repository_id_to_essentialsx_addon_modrinth_projects() -> AddSourceRepositoryIdToEssentialsxAddonModrinthProjectsStmt
{ AddSourceRepositoryIdToEssentialsxAddonModrinthProjectsStmt(cornucopia_async::private::Stmt::new("UPDATE modrinth_project
SET source_repository_id = id
WHERE id IN ('Vem8mYeH', 'lyP3EhLg', 'IWjhyNzg', 'KPfTOjGm', '2qgyQbO1', 'sYpvDxGJ', 'cj1AijZw', '3yb40IgO')")) } pub struct
AddSourceRepositoryIdToEssentialsxAddonModrinthProjectsStmt(cornucopia_async::private::Stmt); impl AddSourceRepositoryIdToEssentialsxAddonModrinthProjectsStmt
{ pub async fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[]).await
} }}pub mod hangar_project
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct UpsertHangarProjectParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,T5: cornucopia_async::StringSql,T6: cornucopia_async::StringSql,T7: cornucopia_async::StringSql,T8: cornucopia_async::StringSql,T9: cornucopia_async::StringSql,T10: cornucopia_async::StringSql,T11: cornucopia_async::StringSql,T12: cornucopia_async::StringSql,> { pub slug: T1,pub author: T2,pub name: T3,pub description: T4,pub date_created: time::OffsetDateTime,pub date_updated: time::OffsetDateTime,pub latest_minecraft_version: Option<T5>,pub downloads: i32,pub stars: i32,pub watchers: i32,pub visibility: T6,pub icon_url: T7,pub version_name: Option<T8>,pub source_url: Option<T9>,pub source_repository_host: Option<T10>,pub source_repository_owner: Option<T11>,pub source_repository_name: Option<T12>,}#[derive( Debug, Clone, PartialEq,)] pub struct HangarProjectEntity
{ pub slug : String,pub author : String,pub name : String,pub description : String,pub latest_minecraft_version : Option<String>,pub date_created : time::OffsetDateTime,pub date_updated : time::OffsetDateTime,pub downloads : i32,pub stars : i32,pub watchers : i32,pub visibility : String,pub icon_url : String,pub version_name : Option<String>,pub source_url : Option<String>,pub source_repository_host : Option<String>,pub source_repository_owner : Option<String>,pub source_repository_name : Option<String>,pub source_repository_id : Option<String>,}pub struct HangarProjectEntityBorrowed<'a> { pub slug : &'a str,pub author : &'a str,pub name : &'a str,pub description : &'a str,pub latest_minecraft_version : Option<&'a str>,pub date_created : time::OffsetDateTime,pub date_updated : time::OffsetDateTime,pub downloads : i32,pub stars : i32,pub watchers : i32,pub visibility : &'a str,pub icon_url : &'a str,pub version_name : Option<&'a str>,pub source_url : Option<&'a str>,pub source_repository_host : Option<&'a str>,pub source_repository_owner : Option<&'a str>,pub source_repository_name : Option<&'a str>,pub source_repository_id : Option<&'a str>,}
impl<'a> From<HangarProjectEntityBorrowed<'a>> for HangarProjectEntity
{
    fn from(HangarProjectEntityBorrowed { slug,author,name,description,latest_minecraft_version,date_created,date_updated,downloads,stars,watchers,visibility,icon_url,version_name,source_url,source_repository_host,source_repository_owner,source_repository_name,source_repository_id,}: HangarProjectEntityBorrowed<'a>) ->
    Self { Self { slug: slug.into(),author: author.into(),name: name.into(),description: description.into(),latest_minecraft_version: latest_minecraft_version.map(|v| v.into()),date_created,date_updated,downloads,stars,watchers,visibility: visibility.into(),icon_url: icon_url.into(),version_name: version_name.map(|v| v.into()),source_url: source_url.map(|v| v.into()),source_repository_host: source_repository_host.map(|v| v.into()),source_repository_owner: source_repository_owner.map(|v| v.into()),source_repository_name: source_repository_name.map(|v| v.into()),source_repository_id: source_repository_id.map(|v| v.into()),} }
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
{ UpsertHangarProjectStmt(cornucopia_async::private::Stmt::new("INSERT INTO hangar_project (slug, author, name, description, date_created, date_updated, latest_minecraft_version, downloads, stars, watchers, visibility, icon_url, version_name, source_url, source_repository_host, source_repository_owner, source_repository_name)
  VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
  ON CONFLICT (slug)
  DO UPDATE SET
    author = EXCLUDED.author,
    name = EXCLUDED.name,
    description = EXCLUDED.description,
    date_created = EXCLUDED.date_created,
    date_updated = EXCLUDED.date_updated,
    latest_minecraft_version = EXCLUDED.latest_minecraft_version,
    downloads = EXCLUDED.downloads,
    stars = EXCLUDED.stars,
    watchers = EXCLUDED.watchers,
    visibility = EXCLUDED.visibility,
    icon_url = EXCLUDED.icon_url,
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
cornucopia_async::StringSql,T12:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
slug: &'a T1,author: &'a T2,name: &'a T3,description: &'a T4,date_created: &'a time::OffsetDateTime,date_updated: &'a time::OffsetDateTime,latest_minecraft_version: &'a Option<T5>,downloads: &'a i32,stars: &'a i32,watchers: &'a i32,visibility: &'a T6,icon_url: &'a T7,version_name: &'a Option<T8>,source_url: &'a Option<T9>,source_repository_host: &'a Option<T10>,source_repository_owner: &'a Option<T11>,source_repository_name: &'a Option<T12>,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[slug,author,name,description,date_created,date_updated,latest_minecraft_version,downloads,stars,watchers,visibility,icon_url,version_name,source_url,source_repository_host,source_repository_owner,source_repository_name,]).await
} }impl <'a, C: GenericClient + Send + Sync, T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,T5: cornucopia_async::StringSql,T6: cornucopia_async::StringSql,T7: cornucopia_async::StringSql,T8: cornucopia_async::StringSql,T9: cornucopia_async::StringSql,T10: cornucopia_async::StringSql,T11: cornucopia_async::StringSql,T12: cornucopia_async::StringSql,>
cornucopia_async::Params<'a, UpsertHangarProjectParams<T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,>, std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
tokio_postgres::Error>> + Send + 'a>>, C> for UpsertHangarProjectStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    UpsertHangarProjectParams<T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
    tokio_postgres::Error>> + Send + 'a>>
    { Box::pin(self.bind(client, &params.slug,&params.author,&params.name,&params.description,&params.date_created,&params.date_updated,&params.latest_minecraft_version,&params.downloads,&params.stars,&params.watchers,&params.visibility,&params.icon_url,&params.version_name,&params.source_url,&params.source_repository_host,&params.source_repository_owner,&params.source_repository_name,)) }
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
        |row| { HangarProjectEntityBorrowed { slug: row.get(0),author: row.get(1),name: row.get(2),description: row.get(3),latest_minecraft_version: row.get(4),date_created: row.get(5),date_updated: row.get(6),downloads: row.get(7),stars: row.get(8),watchers: row.get(9),visibility: row.get(10),icon_url: row.get(11),version_name: row.get(12),source_url: row.get(13),source_repository_host: row.get(14),source_repository_owner: row.get(15),source_repository_name: row.get(16),source_repository_id: row.get(17),} }, mapper: |it| { <HangarProjectEntity>::from(it) },
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
} }}pub mod ingest_log
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive(Clone,Copy, Debug)] pub struct InsertIngestLogParams<> { pub action: super::super::types::public::IngestLogAction,pub repository: super::super::types::public::IngestLogRepository,pub item: super::super::types::public::IngestLogItem,pub date_started: time::OffsetDateTime,pub date_finished: time::OffsetDateTime,pub items_processed: i32,pub success: bool,}#[derive( Debug, Clone, PartialEq,Copy)] pub struct IngestLogEntity
{ pub id : i32,pub action : super::super::types::public::IngestLogAction,pub repository : super::super::types::public::IngestLogRepository,pub item : super::super::types::public::IngestLogItem,pub date_started : time::OffsetDateTime,pub date_finished : time::OffsetDateTime,pub items_processed : i32,pub success : bool,}pub struct IngestLogEntityQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> IngestLogEntity,
    mapper: fn(IngestLogEntity) -> T,
} impl<'a, C, T:'a, const N: usize> IngestLogEntityQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(IngestLogEntity) -> R) ->
    IngestLogEntityQuery<'a,C,R,N>
    {
        IngestLogEntityQuery
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
}pub fn insert_ingest_log() -> InsertIngestLogStmt
{ InsertIngestLogStmt(cornucopia_async::private::Stmt::new("INSERT INTO ingest_log (action, repository, item, date_started, date_finished, items_processed, success)
  VALUES ($1, $2, $3, $4, $5, $6, $7)")) } pub struct
InsertIngestLogStmt(cornucopia_async::private::Stmt); impl InsertIngestLogStmt
{ pub async fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
action: &'a super::super::types::public::IngestLogAction,repository: &'a super::super::types::public::IngestLogRepository,item: &'a super::super::types::public::IngestLogItem,date_started: &'a time::OffsetDateTime,date_finished: &'a time::OffsetDateTime,items_processed: &'a i32,success: &'a bool,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[action,repository,item,date_started,date_finished,items_processed,success,]).await
} }impl <'a, C: GenericClient + Send + Sync, >
cornucopia_async::Params<'a, InsertIngestLogParams<>, std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
tokio_postgres::Error>> + Send + 'a>>, C> for InsertIngestLogStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    InsertIngestLogParams<>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
    tokio_postgres::Error>> + Send + 'a>>
    { Box::pin(self.bind(client, &params.action,&params.repository,&params.item,&params.date_started,&params.date_finished,&params.items_processed,&params.success,)) }
}pub fn get_last_successful_ingest_log() -> GetLastSuccessfulIngestLogStmt
{ GetLastSuccessfulIngestLogStmt(cornucopia_async::private::Stmt::new("SELECT *
FROM ingest_log
WHERE success = TRUE
ORDER BY id DESC
LIMIT 1")) } pub struct
GetLastSuccessfulIngestLogStmt(cornucopia_async::private::Stmt); impl GetLastSuccessfulIngestLogStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
) -> IngestLogEntityQuery<'a,C,
IngestLogEntity, 0>
{
    IngestLogEntityQuery
    {
        client, params: [], stmt: &mut self.0, extractor:
        |row| { IngestLogEntity { id: row.get(0),action: row.get(1),repository: row.get(2),item: row.get(3),date_started: row.get(4),date_finished: row.get(5),items_processed: row.get(6),success: row.get(7),} }, mapper: |it| { <IngestLogEntity>::from(it) },
    }
} }pub fn get_ingest_logs() -> GetIngestLogsStmt
{ GetIngestLogsStmt(cornucopia_async::private::Stmt::new("SELECT *
FROM ingest_log")) } pub struct
GetIngestLogsStmt(cornucopia_async::private::Stmt); impl GetIngestLogsStmt
{ pub fn bind<'a, C:
GenericClient,>(&'a mut self, client: &'a  C,
) -> IngestLogEntityQuery<'a,C,
IngestLogEntity, 0>
{
    IngestLogEntityQuery
    {
        client, params: [], stmt: &mut self.0, extractor:
        |row| { IngestLogEntity { id: row.get(0),action: row.get(1),repository: row.get(2),item: row.get(3),date_started: row.get(4),date_finished: row.get(5),items_processed: row.get(6),success: row.get(7),} }, mapper: |it| { <IngestLogEntity>::from(it) },
    }
} }}pub mod modrinth_project
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct UpsertModrinthProjectParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,T5: cornucopia_async::StringSql,T6: cornucopia_async::StringSql,T7: cornucopia_async::StringSql,T8: cornucopia_async::StringSql,T9: cornucopia_async::StringSql,T10: cornucopia_async::StringSql,T11: cornucopia_async::StringSql,T12: cornucopia_async::StringSql,T13: cornucopia_async::StringSql,T14: cornucopia_async::StringSql,> { pub id: T1,pub slug: T2,pub name: T3,pub description: T4,pub author: T5,pub date_created: time::OffsetDateTime,pub date_updated: time::OffsetDateTime,pub latest_minecraft_version: Option<T6>,pub downloads: i32,pub follows: i32,pub version_id: Option<T7>,pub version_name: Option<T8>,pub status: T9,pub icon_url: Option<T10>,pub source_url: Option<T11>,pub source_repository_host: Option<T12>,pub source_repository_owner: Option<T13>,pub source_repository_name: Option<T14>,}#[derive( Debug, Clone, PartialEq,)] pub struct ModrinthProjectEntity
{ pub id : String,pub slug : String,pub name : String,pub description : String,pub author : String,pub date_created : time::OffsetDateTime,pub date_updated : time::OffsetDateTime,pub latest_minecraft_version : Option<String>,pub downloads : i32,pub follows : i32,pub version_id : Option<String>,pub version_name : Option<String>,pub status : String,pub icon_url : Option<String>,pub source_url : Option<String>,pub source_repository_host : Option<String>,pub source_repository_owner : Option<String>,pub source_repository_name : Option<String>,pub source_repository_id : Option<String>,}pub struct ModrinthProjectEntityBorrowed<'a> { pub id : &'a str,pub slug : &'a str,pub name : &'a str,pub description : &'a str,pub author : &'a str,pub date_created : time::OffsetDateTime,pub date_updated : time::OffsetDateTime,pub latest_minecraft_version : Option<&'a str>,pub downloads : i32,pub follows : i32,pub version_id : Option<&'a str>,pub version_name : Option<&'a str>,pub status : &'a str,pub icon_url : Option<&'a str>,pub source_url : Option<&'a str>,pub source_repository_host : Option<&'a str>,pub source_repository_owner : Option<&'a str>,pub source_repository_name : Option<&'a str>,pub source_repository_id : Option<&'a str>,}
impl<'a> From<ModrinthProjectEntityBorrowed<'a>> for ModrinthProjectEntity
{
    fn from(ModrinthProjectEntityBorrowed { id,slug,name,description,author,date_created,date_updated,latest_minecraft_version,downloads,follows,version_id,version_name,status,icon_url,source_url,source_repository_host,source_repository_owner,source_repository_name,source_repository_id,}: ModrinthProjectEntityBorrowed<'a>) ->
    Self { Self { id: id.into(),slug: slug.into(),name: name.into(),description: description.into(),author: author.into(),date_created,date_updated,latest_minecraft_version: latest_minecraft_version.map(|v| v.into()),downloads,follows,version_id: version_id.map(|v| v.into()),version_name: version_name.map(|v| v.into()),status: status.into(),icon_url: icon_url.map(|v| v.into()),source_url: source_url.map(|v| v.into()),source_repository_host: source_repository_host.map(|v| v.into()),source_repository_owner: source_repository_owner.map(|v| v.into()),source_repository_name: source_repository_name.map(|v| v.into()),source_repository_id: source_repository_id.map(|v| v.into()),} }
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
{ UpsertModrinthProjectStmt(cornucopia_async::private::Stmt::new("INSERT INTO modrinth_project (id, slug, name, description, author, date_created, date_updated, latest_minecraft_version, downloads, follows, version_id, version_name, status, icon_url, source_url, source_repository_host, source_repository_owner, source_repository_name)
  VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
  ON CONFLICT(id)
  DO UPDATE SET
    id = EXCLUDED.id,
    slug = EXCLUDED.slug,
    name = EXCLUDED.name,
    description = EXCLUDED.description,
    author = EXCLUDED.author,
    date_created = EXCLUDED.date_created,
    date_updated = EXCLUDED.date_updated,
    latest_minecraft_version = EXCLUDED.latest_minecraft_version,
    downloads = EXCLUDED.downloads,
    follows = EXCLUDED.follows,
    version_id = EXCLUDED.version_id,
    version_name = EXCLUDED.version_name,
    status = EXCLUDED.status,
    icon_url = EXCLUDED.icon_url,
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
cornucopia_async::StringSql,T14:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
id: &'a T1,slug: &'a T2,name: &'a T3,description: &'a T4,author: &'a T5,date_created: &'a time::OffsetDateTime,date_updated: &'a time::OffsetDateTime,latest_minecraft_version: &'a Option<T6>,downloads: &'a i32,follows: &'a i32,version_id: &'a Option<T7>,version_name: &'a Option<T8>,status: &'a T9,icon_url: &'a Option<T10>,source_url: &'a Option<T11>,source_repository_host: &'a Option<T12>,source_repository_owner: &'a Option<T13>,source_repository_name: &'a Option<T14>,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[id,slug,name,description,author,date_created,date_updated,latest_minecraft_version,downloads,follows,version_id,version_name,status,icon_url,source_url,source_repository_host,source_repository_owner,source_repository_name,]).await
} }impl <'a, C: GenericClient + Send + Sync, T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,T5: cornucopia_async::StringSql,T6: cornucopia_async::StringSql,T7: cornucopia_async::StringSql,T8: cornucopia_async::StringSql,T9: cornucopia_async::StringSql,T10: cornucopia_async::StringSql,T11: cornucopia_async::StringSql,T12: cornucopia_async::StringSql,T13: cornucopia_async::StringSql,T14: cornucopia_async::StringSql,>
cornucopia_async::Params<'a, UpsertModrinthProjectParams<T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,>, std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
tokio_postgres::Error>> + Send + 'a>>, C> for UpsertModrinthProjectStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    UpsertModrinthProjectParams<T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,T13,T14,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
    tokio_postgres::Error>> + Send + 'a>>
    { Box::pin(self.bind(client, &params.id,&params.slug,&params.name,&params.description,&params.author,&params.date_created,&params.date_updated,&params.latest_minecraft_version,&params.downloads,&params.follows,&params.version_id,&params.version_name,&params.status,&params.icon_url,&params.source_url,&params.source_repository_host,&params.source_repository_owner,&params.source_repository_name,)) }
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
        |row| { ModrinthProjectEntityBorrowed { id: row.get(0),slug: row.get(1),name: row.get(2),description: row.get(3),author: row.get(4),date_created: row.get(5),date_updated: row.get(6),latest_minecraft_version: row.get(7),downloads: row.get(8),follows: row.get(9),version_id: row.get(10),version_name: row.get(11),status: row.get(12),icon_url: row.get(13),source_url: row.get(14),source_repository_host: row.get(15),source_repository_owner: row.get(16),source_repository_name: row.get(17),source_repository_id: row.get(18),} }, mapper: |it| { <ModrinthProjectEntity>::from(it) },
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
} }}pub mod search_result
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct SearchProjectsParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,> { pub spigot: bool,pub modrinth: bool,pub hangar: bool,pub query: T1,pub name: bool,pub description: bool,pub author: bool,pub sort: T2,pub limit: i64,pub offset: i64,}#[derive( Debug, Clone, PartialEq,)] pub struct SearchResultEntity
{ pub full_count : i64,pub date_created : time::OffsetDateTime,pub date_updated : time::OffsetDateTime,pub latest_minecraft_version : Option<String>,pub downloads : i32,pub likes_and_stars : i32,pub follows_and_watchers : i32,pub spigot_id : Option<i32>,pub spigot_slug : Option<String>,pub spigot_name : Option<String>,pub spigot_description : Option<String>,pub spigot_author : Option<String>,pub spigot_version : Option<String>,pub spigot_premium : Option<bool>,pub spigot_abandoned : Option<bool>,pub spigot_icon_data : Option<String>,pub modrinth_id : Option<String>,pub modrinth_slug : Option<String>,pub modrinth_name : Option<String>,pub modrinth_description : Option<String>,pub modrinth_author : Option<String>,pub modrinth_version : Option<String>,pub modrinth_status : Option<String>,pub modrinth_icon_url : Option<String>,pub hangar_slug : Option<String>,pub hangar_name : Option<String>,pub hangar_description : Option<String>,pub hangar_author : Option<String>,pub hangar_version : Option<String>,pub hangar_icon_url : Option<String>,pub source_repository_host : Option<String>,pub source_repository_owner : Option<String>,pub source_repository_name : Option<String>,pub source_repository_id : Option<String>,}pub struct SearchResultEntityBorrowed<'a> { pub full_count : i64,pub date_created : time::OffsetDateTime,pub date_updated : time::OffsetDateTime,pub latest_minecraft_version : Option<&'a str>,pub downloads : i32,pub likes_and_stars : i32,pub follows_and_watchers : i32,pub spigot_id : Option<i32>,pub spigot_slug : Option<&'a str>,pub spigot_name : Option<&'a str>,pub spigot_description : Option<&'a str>,pub spigot_author : Option<&'a str>,pub spigot_version : Option<&'a str>,pub spigot_premium : Option<bool>,pub spigot_abandoned : Option<bool>,pub spigot_icon_data : Option<&'a str>,pub modrinth_id : Option<&'a str>,pub modrinth_slug : Option<&'a str>,pub modrinth_name : Option<&'a str>,pub modrinth_description : Option<&'a str>,pub modrinth_author : Option<&'a str>,pub modrinth_version : Option<&'a str>,pub modrinth_status : Option<&'a str>,pub modrinth_icon_url : Option<&'a str>,pub hangar_slug : Option<&'a str>,pub hangar_name : Option<&'a str>,pub hangar_description : Option<&'a str>,pub hangar_author : Option<&'a str>,pub hangar_version : Option<&'a str>,pub hangar_icon_url : Option<&'a str>,pub source_repository_host : Option<&'a str>,pub source_repository_owner : Option<&'a str>,pub source_repository_name : Option<&'a str>,pub source_repository_id : Option<&'a str>,}
impl<'a> From<SearchResultEntityBorrowed<'a>> for SearchResultEntity
{
    fn from(SearchResultEntityBorrowed { full_count,date_created,date_updated,latest_minecraft_version,downloads,likes_and_stars,follows_and_watchers,spigot_id,spigot_slug,spigot_name,spigot_description,spigot_author,spigot_version,spigot_premium,spigot_abandoned,spigot_icon_data,modrinth_id,modrinth_slug,modrinth_name,modrinth_description,modrinth_author,modrinth_version,modrinth_status,modrinth_icon_url,hangar_slug,hangar_name,hangar_description,hangar_author,hangar_version,hangar_icon_url,source_repository_host,source_repository_owner,source_repository_name,source_repository_id,}: SearchResultEntityBorrowed<'a>) ->
    Self { Self { full_count,date_created,date_updated,latest_minecraft_version: latest_minecraft_version.map(|v| v.into()),downloads,likes_and_stars,follows_and_watchers,spigot_id,spigot_slug: spigot_slug.map(|v| v.into()),spigot_name: spigot_name.map(|v| v.into()),spigot_description: spigot_description.map(|v| v.into()),spigot_author: spigot_author.map(|v| v.into()),spigot_version: spigot_version.map(|v| v.into()),spigot_premium,spigot_abandoned,spigot_icon_data: spigot_icon_data.map(|v| v.into()),modrinth_id: modrinth_id.map(|v| v.into()),modrinth_slug: modrinth_slug.map(|v| v.into()),modrinth_name: modrinth_name.map(|v| v.into()),modrinth_description: modrinth_description.map(|v| v.into()),modrinth_author: modrinth_author.map(|v| v.into()),modrinth_version: modrinth_version.map(|v| v.into()),modrinth_status: modrinth_status.map(|v| v.into()),modrinth_icon_url: modrinth_icon_url.map(|v| v.into()),hangar_slug: hangar_slug.map(|v| v.into()),hangar_name: hangar_name.map(|v| v.into()),hangar_description: hangar_description.map(|v| v.into()),hangar_author: hangar_author.map(|v| v.into()),hangar_version: hangar_version.map(|v| v.into()),hangar_icon_url: hangar_icon_url.map(|v| v.into()),source_repository_host: source_repository_host.map(|v| v.into()),source_repository_owner: source_repository_owner.map(|v| v.into()),source_repository_name: source_repository_name.map(|v| v.into()),source_repository_id: source_repository_id.map(|v| v.into()),} }
}pub struct SearchResultEntityQuery<'a, C: GenericClient, T, const N: usize>
{
    client: &'a  C, params:
    [&'a (dyn postgres_types::ToSql + Sync); N], stmt: &'a mut
    cornucopia_async::private::Stmt, extractor: fn(&tokio_postgres::Row) -> SearchResultEntityBorrowed,
    mapper: fn(SearchResultEntityBorrowed) -> T,
} impl<'a, C, T:'a, const N: usize> SearchResultEntityQuery<'a, C, T, N> where C:
GenericClient
{
    pub fn map<R>(self, mapper: fn(SearchResultEntityBorrowed) -> R) ->
    SearchResultEntityQuery<'a,C,R,N>
    {
        SearchResultEntityQuery
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
}pub fn search_projects() -> SearchProjectsStmt
{ SearchProjectsStmt(cornucopia_async::private::Stmt::new("SELECT
  COUNT(*) OVER() AS full_count,

  GREATEST(
    CASE WHEN $1 IS TRUE THEN spigot_date_created ELSE NULL END,
    CASE WHEN $2 IS TRUE THEN modrinth_date_created ELSE NULL END,
    CASE WHEN $3 IS TRUE THEN hangar_date_created ELSE NULL END
  ) AS date_created,

  GREATEST(
    CASE WHEN $1 IS TRUE THEN spigot_date_updated ELSE NULL END,
    CASE WHEN $2 IS TRUE THEN modrinth_date_updated ELSE NULL END,
    CASE WHEN $3 IS TRUE THEN hangar_date_updated ELSE NULL END
  ) AS date_updated,

  GREATEST(
    CASE WHEN $1 IS TRUE THEN spigot_latest_minecraft_version ELSE NULL END,
    CASE WHEN $2 IS TRUE THEN modrinth_latest_minecraft_version ELSE NULL END,
    CASE WHEN $3 IS TRUE THEN hangar_latest_minecraft_version ELSE NULL END
  ) AS latest_minecraft_version,

  CASE WHEN $1 IS TRUE THEN COALESCE(spigot_downloads, 0) ELSE 0 END +
  CASE WHEN $2 IS TRUE THEN COALESCE(modrinth_downloads, 0) ELSE 0 END +
  CASE WHEN $3 IS TRUE THEN COALESCE(hangar_downloads, 0) ELSE 0 END
  AS downloads,

  CASE WHEN $1 IS TRUE THEN COALESCE(spigot_likes, 0) ELSE 0 END +
  CASE WHEN $3 IS TRUE THEN COALESCE(hangar_stars, 0) ELSE 0 END
  AS likes_and_stars,

  CASE WHEN $2 IS TRUE THEN COALESCE(modrinth_follows, 0) ELSE 0 END +
  CASE WHEN $3 IS TRUE THEN COALESCE(hangar_watchers, 0) ELSE 0 END
  AS follows_and_watchers,

  (CASE WHEN $1 IS TRUE THEN spigot_id ELSE NULL END) AS spigot_id,
  (CASE WHEN $1 IS TRUE THEN spigot_slug ELSE NULL END) AS spigot_slug,
  (CASE WHEN $1 IS TRUE THEN spigot_name ELSE NULL END) AS spigot_name,
  (CASE WHEN $1 IS TRUE THEN spigot_description ELSE NULL END) AS spigot_description,
  (CASE WHEN $1 IS TRUE THEN spigot_author ELSE NULL END) AS spigot_author,
  (CASE WHEN $1 IS TRUE THEN spigot_version ELSE NULL END) AS spigot_version,
  (CASE WHEN $1 IS TRUE THEN spigot_premium ELSE NULL END) AS spigot_premium,
  (CASE WHEN $1 IS TRUE THEN spigot_abandoned ELSE NULL END) AS spigot_abandoned,
  (CASE WHEN $1 IS TRUE THEN spigot_icon_data ELSE NULL END) AS spigot_icon_data,

  (CASE WHEN $2 IS TRUE THEN modrinth_id ELSE NULL END) AS modrinth_id,
  (CASE WHEN $2 IS TRUE THEN modrinth_slug ELSE NULL END) AS modrinth_slug,
  (CASE WHEN $2 IS TRUE THEN modrinth_name ELSE NULL END) AS modrinth_name,
  (CASE WHEN $2 IS TRUE THEN modrinth_description ELSE NULL END) AS modrinth_description,
  (CASE WHEN $2 IS TRUE THEN modrinth_author ELSE NULL END) AS modrinth_author,
  (CASE WHEN $2 IS TRUE THEN modrinth_version ELSE NULL END) AS modrinth_version,
  (CASE WHEN $2 IS TRUE THEN modrinth_status ELSE NULL END) AS modrinth_status,
  (CASE WHEN $2 IS TRUE THEN modrinth_icon_url ELSE NULL END) AS modrinth_icon_url,

  (CASE WHEN $3 IS TRUE THEN hangar_slug ELSE NULL END) AS hangar_slug,
  (CASE WHEN $3 IS TRUE THEN hangar_name ELSE NULL END) AS hangar_name,
  (CASE WHEN $3 IS TRUE THEN hangar_description ELSE NULL END) AS hangar_description,
  (CASE WHEN $3 IS TRUE THEN hangar_author ELSE NULL END) AS hangar_author,
  (CASE WHEN $3 IS TRUE THEN hangar_version ELSE NULL END) AS hangar_version,
  (CASE WHEN $3 IS TRUE THEN hangar_icon_url ELSE NULL END) AS hangar_icon_url,

  source_repository_host,
  source_repository_owner,
  source_repository_name,
  source_repository_id
FROM
  common_project
WHERE
  CASE $1 IS TRUE AND $4 = ''
    WHEN TRUE THEN spigot_id IS NOT NULL
    ELSE FALSE
  END

  OR

  CASE $1 IS TRUE AND $5 IS TRUE
    WHEN TRUE THEN $4 <% spigot_name
    ELSE FALSE
  END

  OR

  CASE $1 IS TRUE AND $6 IS TRUE
    WHEN TRUE THEN $4 <% spigot_description
    ELSE FALSE
  END

  OR

  CASE $1 IS TRUE AND $7 IS TRUE
    WHEN TRUE THEN $4 <% spigot_author
    ELSE FALSE
  END

  OR

  CASE $2 IS TRUE AND $4 = ''
    WHEN TRUE THEN modrinth_id IS NOT NULL
    ELSE FALSE
  END

  OR

  CASE $2 IS TRUE AND $5 IS TRUE
    WHEN TRUE THEN $4 <% modrinth_name
    ELSE FALSE
  END

  OR

  CASE $2 IS TRUE AND $6 IS TRUE
    WHEN TRUE THEN $4 <% modrinth_description
    ELSE FALSE
  END

  OR

  CASE $2 IS TRUE AND $7 IS TRUE
    WHEN TRUE THEN $4 <% modrinth_author
    ELSE FALSE
  END

  OR

  CASE $3 IS TRUE AND $4 = ''
    WHEN TRUE THEN hangar_slug IS NOT NULL
    ELSE FALSE
  END

  OR

  CASE $3 IS TRUE AND $5 IS TRUE
    WHEN TRUE THEN $4 <% hangar_name
    ELSE FALSE
  END

  OR

  CASE $3 IS TRUE AND $6 IS TRUE
    WHEN TRUE THEN $4 <% hangar_description
    ELSE FALSE
  END

  OR

  CASE $3 IS TRUE AND $7 IS TRUE
    WHEN TRUE THEN $4 <% hangar_author
    ELSE FALSE
  END

  ORDER BY
    -- Sorts on 'real' type
    CASE
      WHEN $8 = 'relevance' AND $4 != '' THEN
        GREATEST(
          CASE WHEN $1 IS TRUE THEN
            GREATEST(
              CASE WHEN $5 IS TRUE THEN $4 <<-> spigot_name ELSE NULL END,
              CASE WHEN $6 IS TRUE THEN $4 <<-> spigot_description ELSE NULL END,
              CASE WHEN $7 IS TRUE THEN $4 <<-> spigot_author ELSE NULL END
            )
          ELSE NULL END,
          CASE WHEN $2 IS TRUE THEN
            GREATEST(
              CASE WHEN $5 IS TRUE THEN $4 <<-> modrinth_name ELSE NULL END,
              CASE WHEN $6 IS TRUE THEN $4 <<-> modrinth_description ELSE NULL END,
              CASE WHEN $7 IS TRUE THEN $4 <<-> modrinth_author ELSE NULL END
            )
          ELSE NULL END,
          CASE WHEN $3 IS TRUE THEN
            GREATEST(
              CASE WHEN $5 IS TRUE THEN $4 <<-> hangar_name ELSE NULL END,
              CASE WHEN $6 IS TRUE THEN $4 <<-> hangar_description ELSE NULL END,
              CASE WHEN $7 IS TRUE THEN $4 <<-> hangar_author ELSE NULL END
            )
          ELSE NULL END
        )
    END ASC NULLS LAST,

    -- Sorts on 'timestamptz' type
    CASE
      WHEN $8 = 'date_created' THEN
        GREATEST(
          CASE WHEN $1 IS TRUE THEN spigot_date_created ELSE NULL END,
          CASE WHEN $2 IS TRUE THEN modrinth_date_created ELSE NULL END,
          CASE WHEN $3 IS TRUE THEN hangar_date_created ELSE NULL END
        )

      WHEN $8 = 'date_updated' THEN
        GREATEST(
          CASE WHEN $1 IS TRUE THEN spigot_date_updated ELSE NULL END,
          CASE WHEN $2 IS TRUE THEN modrinth_date_updated ELSE NULL END,
          CASE WHEN $3 IS TRUE THEN hangar_date_updated ELSE NULL END
        )
    END DESC NULLS LAST,

    -- Sorts on 'text' type
    CASE
      WHEN $8 = 'latest_minecraft_version' THEN
        GREATEST(
          CASE WHEN $1 IS TRUE THEN spigot_latest_minecraft_version ELSE NULL END,
          CASE WHEN $2 IS TRUE THEN modrinth_latest_minecraft_version ELSE NULL END,
          CASE WHEN $3 IS TRUE THEN hangar_latest_minecraft_version ELSE NULL END
        )
    END DESC NULLS LAST,

    -- Sorts on 'integer' type
    CASE
      WHEN $8 = 'likes_and_stars' THEN
        CASE WHEN $1 IS TRUE THEN COALESCE(spigot_likes, 0) ELSE 0 END +
        CASE WHEN $3 IS TRUE THEN COALESCE(hangar_stars, 0) ELSE 0 END

      WHEN $8 = 'follows_and_watchers' THEN
        CASE WHEN $2 IS TRUE THEN COALESCE(modrinth_follows, 0) ELSE 0 END +
        CASE WHEN $3 IS TRUE THEN COALESCE(hangar_watchers, 0) ELSE 0 END
    END DESC NULLS LAST,

    -- Fallback to sort by downloads when no sort is specified or as a secondary sort
    CASE WHEN $1 IS TRUE THEN COALESCE(spigot_downloads, 0) ELSE 0 END +
    CASE WHEN $2 IS TRUE THEN COALESCE(modrinth_downloads, 0) ELSE 0 END +
    CASE WHEN $3 IS TRUE THEN COALESCE(hangar_downloads, 0) ELSE 0 END
    DESC NULLS LAST

LIMIT $9
OFFSET $10")) } pub struct
SearchProjectsStmt(cornucopia_async::private::Stmt); impl SearchProjectsStmt
{ pub fn bind<'a, C:
GenericClient,T1:
cornucopia_async::StringSql,T2:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
spigot: &'a bool,modrinth: &'a bool,hangar: &'a bool,query: &'a T1,name: &'a bool,description: &'a bool,author: &'a bool,sort: &'a T2,limit: &'a i64,offset: &'a i64,) -> SearchResultEntityQuery<'a,C,
SearchResultEntity, 10>
{
    SearchResultEntityQuery
    {
        client, params: [spigot,modrinth,hangar,query,name,description,author,sort,limit,offset,], stmt: &mut self.0, extractor:
        |row| { SearchResultEntityBorrowed { full_count: row.get(0),date_created: row.get(1),date_updated: row.get(2),latest_minecraft_version: row.get(3),downloads: row.get(4),likes_and_stars: row.get(5),follows_and_watchers: row.get(6),spigot_id: row.get(7),spigot_slug: row.get(8),spigot_name: row.get(9),spigot_description: row.get(10),spigot_author: row.get(11),spigot_version: row.get(12),spigot_premium: row.get(13),spigot_abandoned: row.get(14),spigot_icon_data: row.get(15),modrinth_id: row.get(16),modrinth_slug: row.get(17),modrinth_name: row.get(18),modrinth_description: row.get(19),modrinth_author: row.get(20),modrinth_version: row.get(21),modrinth_status: row.get(22),modrinth_icon_url: row.get(23),hangar_slug: row.get(24),hangar_name: row.get(25),hangar_description: row.get(26),hangar_author: row.get(27),hangar_version: row.get(28),hangar_icon_url: row.get(29),source_repository_host: row.get(30),source_repository_owner: row.get(31),source_repository_name: row.get(32),source_repository_id: row.get(33),} }, mapper: |it| { <SearchResultEntity>::from(it) },
    }
} }impl <'a, C: GenericClient,T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,> cornucopia_async::Params<'a,
SearchProjectsParams<T1,T2,>, SearchResultEntityQuery<'a, C,
SearchResultEntity, 10>, C> for SearchProjectsStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    SearchProjectsParams<T1,T2,>) -> SearchResultEntityQuery<'a, C,
    SearchResultEntity, 10>
    { self.bind(client, &params.spigot,&params.modrinth,&params.hangar,&params.query,&params.name,&params.description,&params.author,&params.sort,&params.limit,&params.offset,) }
}}pub mod spigot_author
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
}pub fn insert_spigot_author() -> InsertSpigotAuthorStmt
{ InsertSpigotAuthorStmt(cornucopia_async::private::Stmt::new("INSERT INTO spigot_author (id, name)
  VALUES ($1, $2)
  ON CONFLICT DO NOTHING")) } pub struct
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
} }}pub mod spigot_resource
{ use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;#[derive( Debug)] pub struct UpsertSpigotResourceParams<T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,T5: cornucopia_async::StringSql,T6: cornucopia_async::StringSql,T7: cornucopia_async::StringSql,T8: cornucopia_async::StringSql,T9: cornucopia_async::StringSql,T10: cornucopia_async::StringSql,T11: cornucopia_async::StringSql,T12: cornucopia_async::StringSql,> { pub id: i32,pub name: T1,pub parsed_name: Option<T2>,pub description: T3,pub slug: T4,pub date_created: time::OffsetDateTime,pub date_updated: time::OffsetDateTime,pub latest_minecraft_version: Option<T5>,pub downloads: i32,pub likes: i32,pub author_id: i32,pub version_id: i32,pub version_name: Option<T6>,pub premium: bool,pub abandoned: bool,pub icon_url: Option<T7>,pub icon_data: Option<T8>,pub source_url: Option<T9>,pub source_repository_host: Option<T10>,pub source_repository_owner: Option<T11>,pub source_repository_name: Option<T12>,}#[derive( Debug, Clone, PartialEq,)] pub struct SpigotResourceEntity
{ pub id : i32,pub name : String,pub parsed_name : Option<String>,pub description : String,pub slug : String,pub date_created : time::OffsetDateTime,pub date_updated : time::OffsetDateTime,pub latest_minecraft_version : Option<String>,pub downloads : i32,pub likes : i32,pub author_id : i32,pub version_id : i32,pub version_name : Option<String>,pub premium : bool,pub abandoned : bool,pub icon_url : Option<String>,pub icon_data : Option<String>,pub source_url : Option<String>,pub source_repository_host : Option<String>,pub source_repository_owner : Option<String>,pub source_repository_name : Option<String>,pub source_repository_id : Option<String>,}pub struct SpigotResourceEntityBorrowed<'a> { pub id : i32,pub name : &'a str,pub parsed_name : Option<&'a str>,pub description : &'a str,pub slug : &'a str,pub date_created : time::OffsetDateTime,pub date_updated : time::OffsetDateTime,pub latest_minecraft_version : Option<&'a str>,pub downloads : i32,pub likes : i32,pub author_id : i32,pub version_id : i32,pub version_name : Option<&'a str>,pub premium : bool,pub abandoned : bool,pub icon_url : Option<&'a str>,pub icon_data : Option<&'a str>,pub source_url : Option<&'a str>,pub source_repository_host : Option<&'a str>,pub source_repository_owner : Option<&'a str>,pub source_repository_name : Option<&'a str>,pub source_repository_id : Option<&'a str>,}
impl<'a> From<SpigotResourceEntityBorrowed<'a>> for SpigotResourceEntity
{
    fn from(SpigotResourceEntityBorrowed { id,name,parsed_name,description,slug,date_created,date_updated,latest_minecraft_version,downloads,likes,author_id,version_id,version_name,premium,abandoned,icon_url,icon_data,source_url,source_repository_host,source_repository_owner,source_repository_name,source_repository_id,}: SpigotResourceEntityBorrowed<'a>) ->
    Self { Self { id,name: name.into(),parsed_name: parsed_name.map(|v| v.into()),description: description.into(),slug: slug.into(),date_created,date_updated,latest_minecraft_version: latest_minecraft_version.map(|v| v.into()),downloads,likes,author_id,version_id,version_name: version_name.map(|v| v.into()),premium,abandoned,icon_url: icon_url.map(|v| v.into()),icon_data: icon_data.map(|v| v.into()),source_url: source_url.map(|v| v.into()),source_repository_host: source_repository_host.map(|v| v.into()),source_repository_owner: source_repository_owner.map(|v| v.into()),source_repository_name: source_repository_name.map(|v| v.into()),source_repository_id: source_repository_id.map(|v| v.into()),} }
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
{ UpsertSpigotResourceStmt(cornucopia_async::private::Stmt::new("INSERT INTO spigot_resource (id, name, parsed_name, description, slug, date_created, date_updated, latest_minecraft_version, downloads, likes, author_id, version_id, version_name, premium, abandoned, icon_url, icon_data, source_url, source_repository_host, source_repository_owner, source_repository_name)
  VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21)
  ON CONFLICT (id)
  DO UPDATE SET
    name = EXCLUDED.name,
    parsed_name = EXCLUDED.parsed_name,
    description = EXCLUDED.description,
    slug = EXCLUDED.slug,
    date_created = EXCLUDED.date_created,
    date_updated = EXCLUDED.date_updated,
    latest_minecraft_version = EXCLUDED.latest_minecraft_version,
    downloads = EXCLUDED.downloads,
    likes = EXCLUDED.likes,
    author_id = EXCLUDED.author_id,
    version_id = EXCLUDED.version_id,
    version_name = EXCLUDED.version_name,
    premium = EXCLUDED.premium,
    abandoned = EXCLUDED.abandoned,
    icon_url = EXCLUDED.icon_url,
    icon_data = EXCLUDED.icon_data,
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
cornucopia_async::StringSql,T10:
cornucopia_async::StringSql,T11:
cornucopia_async::StringSql,T12:
cornucopia_async::StringSql,>(&'a mut self, client: &'a  C,
id: &'a i32,name: &'a T1,parsed_name: &'a Option<T2>,description: &'a T3,slug: &'a T4,date_created: &'a time::OffsetDateTime,date_updated: &'a time::OffsetDateTime,latest_minecraft_version: &'a Option<T5>,downloads: &'a i32,likes: &'a i32,author_id: &'a i32,version_id: &'a i32,version_name: &'a Option<T6>,premium: &'a bool,abandoned: &'a bool,icon_url: &'a Option<T7>,icon_data: &'a Option<T8>,source_url: &'a Option<T9>,source_repository_host: &'a Option<T10>,source_repository_owner: &'a Option<T11>,source_repository_name: &'a Option<T12>,) -> Result<u64, tokio_postgres::Error>
{
    let stmt = self.0.prepare(client).await?;
    client.execute(stmt, &[id,name,parsed_name,description,slug,date_created,date_updated,latest_minecraft_version,downloads,likes,author_id,version_id,version_name,premium,abandoned,icon_url,icon_data,source_url,source_repository_host,source_repository_owner,source_repository_name,]).await
} }impl <'a, C: GenericClient + Send + Sync, T1: cornucopia_async::StringSql,T2: cornucopia_async::StringSql,T3: cornucopia_async::StringSql,T4: cornucopia_async::StringSql,T5: cornucopia_async::StringSql,T6: cornucopia_async::StringSql,T7: cornucopia_async::StringSql,T8: cornucopia_async::StringSql,T9: cornucopia_async::StringSql,T10: cornucopia_async::StringSql,T11: cornucopia_async::StringSql,T12: cornucopia_async::StringSql,>
cornucopia_async::Params<'a, UpsertSpigotResourceParams<T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,>, std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
tokio_postgres::Error>> + Send + 'a>>, C> for UpsertSpigotResourceStmt
{
    fn
    params(&'a mut self, client: &'a  C, params: &'a
    UpsertSpigotResourceParams<T1,T2,T3,T4,T5,T6,T7,T8,T9,T10,T11,T12,>) -> std::pin::Pin<Box<dyn futures::Future<Output = Result<u64,
    tokio_postgres::Error>> + Send + 'a>>
    { Box::pin(self.bind(client, &params.id,&params.name,&params.parsed_name,&params.description,&params.slug,&params.date_created,&params.date_updated,&params.latest_minecraft_version,&params.downloads,&params.likes,&params.author_id,&params.version_id,&params.version_name,&params.premium,&params.abandoned,&params.icon_url,&params.icon_data,&params.source_url,&params.source_repository_host,&params.source_repository_owner,&params.source_repository_name,)) }
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
        |row| { SpigotResourceEntityBorrowed { id: row.get(0),name: row.get(1),parsed_name: row.get(2),description: row.get(3),slug: row.get(4),date_created: row.get(5),date_updated: row.get(6),latest_minecraft_version: row.get(7),downloads: row.get(8),likes: row.get(9),author_id: row.get(10),version_id: row.get(11),version_name: row.get(12),premium: row.get(13),abandoned: row.get(14),icon_url: row.get(15),icon_data: row.get(16),source_url: row.get(17),source_repository_host: row.get(18),source_repository_owner: row.get(19),source_repository_name: row.get(20),source_repository_id: row.get(21),} }, mapper: |it| { <SpigotResourceEntity>::from(it) },
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