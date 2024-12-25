--: CommonProjectEntity(spigot_id?, spigot_slug?, spigot_name?, spigot_description?, spigot_author?, spigot_version?, spigot_premium?, spigot_abandoned?, spigot_icon_data?, spigot_date_created?, spigot_date_updated?, spigot_latest_minecraft_version?, spigot_downloads?, spigot_likes?, modrinth_id?, modrinth_slug?, modrinth_name?, modrinth_description?, modrinth_author?, modrinth_version?, modrinth_icon_url?, modrinth_date_created?, modrinth_date_updated?, modrinth_latest_minecraft_version?, modrinth_downloads?, modrinth_follows?, hangar_slug?, hangar_name?, hangar_description?, hangar_author?, hangar_version?, hangar_icon_url?, hangar_date_created?, hangar_date_updated?, hangar_latest_minecraft_version?, hangar_downloads?, hangar_stars?, hangar_watchers?, source_repository_host?, source_repository_owner?, source_repository_name?)

--! refresh_common_projects
REFRESH MATERIALIZED VIEW common_project;

--! get_common_projects : CommonProjectEntity
SELECT
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
  common_project;