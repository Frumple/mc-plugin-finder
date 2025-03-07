DROP MATERIALIZED VIEW common_project;

CREATE MATERIALIZED VIEW common_project AS
SELECT
  s.id AS spigot_id,
  s.slug AS spigot_slug,
  s.parsed_name AS spigot_name,
  s.description AS spigot_description,
  a.name AS spigot_author,
  s.version_name AS spigot_version,
  s.premium AS spigot_premium,
  s.abandoned AS spigot_abandoned,
  s.icon_data AS spigot_icon_data,
  s.date_created AS spigot_date_created,
  s.date_updated AS spigot_date_updated,
  s.latest_minecraft_version AS spigot_latest_minecraft_version,
  s.downloads AS spigot_downloads,
  s.likes AS spigot_likes,

  m.id AS modrinth_id,
  m.slug AS modrinth_slug,
  m.name AS modrinth_name,
  m.description AS modrinth_description,
  m.author AS modrinth_author,
  m.version_name AS modrinth_version,
  m.status AS modrinth_status,
  m.icon_url AS modrinth_icon_url,
  m.date_created AS modrinth_date_created,
  m.date_updated AS modrinth_date_updated,
  m.latest_minecraft_version AS modrinth_latest_minecraft_version,
  m.downloads AS modrinth_downloads,
  m.follows AS modrinth_follows,

  h.slug AS hangar_slug,
  h.name AS hangar_name,
  h.description AS hangar_description,
  h.author AS hangar_author,
  h.version_name AS hangar_version,
  h.icon_url AS hangar_icon_url,
  h.date_created AS hangar_date_created,
  h.date_updated AS hangar_date_updated,
  h.latest_minecraft_version AS hangar_latest_minecraft_version,
  h.downloads AS hangar_downloads,
  h.stars AS hangar_stars,
  h.watchers AS hangar_watchers,

  COALESCE(s.source_repository_host, m.source_repository_host, h.source_repository_host) AS source_repository_host,
  COALESCE(s.source_repository_owner, m.source_repository_owner, h.source_repository_owner) AS source_repository_owner,
  COALESCE(s.source_repository_name, m.source_repository_name, h.source_repository_name) AS source_repository_name
FROM
  spigot_resource s
  INNER JOIN spigot_author a
  ON  s.author_id = a.id

  FULL JOIN modrinth_project m
  ON  LOWER(s.source_repository_host) = LOWER(m.source_repository_host)
  AND LOWER(s.source_repository_owner) = LOWER(m.source_repository_owner)
  AND LOWER(s.source_repository_name) = LOWER(m.source_repository_name)

  FULL JOIN hangar_project h
  ON  LOWER(COALESCE(s.source_repository_host, m.source_repository_host)) = LOWER(h.source_repository_host)
  AND LOWER(COALESCE(s.source_repository_owner, m.source_repository_owner)) = LOWER(h.source_repository_owner)
  AND LOWER(COALESCE(s.source_repository_name, m.source_repository_name)) = LOWER(h.source_repository_name);