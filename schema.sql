-- Enable trigram module for text search
CREATE EXTENSION IF NOT EXISTS pg_trgm;

-- Enable collation for ordering Minecraft versions
CREATE COLLATION en_natural (
  LOCALE = 'en-US-u-kn-true',
  PROVIDER = 'icu'
);

-- Tables

-- Spigot
CREATE TABLE IF NOT EXISTS spigot_author (
  id SERIAL PRIMARY KEY,
  name text NOT NULL
);

CREATE TABLE IF NOT EXISTS spigot_resource (
  id SERIAL PRIMARY KEY,
  name text NOT NULL,
  parsed_name text,
  description text NOT NULL,
  slug text NOT NULL,
  date_created timestamptz NOT NULL,
  date_updated timestamptz NOT NULL,
  latest_minecraft_version text COLLATE en_natural,
  downloads integer NOT NULL,
  likes integer NOT NULL,
  author_id integer NOT NULL REFERENCES spigot_author,
  version_id integer NOT NULL,
  version_name text,
  premium boolean NOT NULL,
  abandoned boolean NOT NULL,
  icon_url text,
  icon_data text,
  source_url text,
  source_repository_host text,
  source_repository_owner text,
  source_repository_name text
);

-- Modrinth
CREATE TABLE IF NOT EXISTS modrinth_project (
  id text PRIMARY KEY,
  slug text NOT NULL,
  name text NOT NULL,
  description text NOT NULL,
  author text NOT NULL,
  date_created timestamptz NOT NULL,
  date_updated timestamptz NOT NULL,
  latest_minecraft_version text COLLATE en_natural,
  downloads integer NOT NULL,
  follows integer NOT NULL,
  version_id text,
  version_name text,
  status text NOT NULL,
  icon_url text,
  source_url text,
  source_repository_host text,
  source_repository_owner text,
  source_repository_name text
);

-- Hangar
CREATE TABLE IF NOT EXISTS hangar_project (
  slug text PRIMARY KEY,
  author text NOT NULL,
  name text NOT NULL,
  description text NOT NULL,
  latest_minecraft_version text COLLATE en_natural,
  date_created timestamptz NOT NULL,
  date_updated timestamptz NOT NULL,
  downloads integer NOT NULL,
  stars integer NOT NULL,
  watchers integer NOT NULL,
  visibility text NOT NULL,
  icon_url text NOT NULL,
  version_name text,
  source_url text,
  source_repository_host text,
  source_repository_owner text,
  source_repository_name text
);

-- Common
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

-- Ingest Logs

CREATE TYPE ingest_log_action AS ENUM('Populate', 'Update', 'Refresh');
CREATE TYPE ingest_log_repository AS ENUM('Spigot', 'Modrinth', 'Hangar', 'Common');
CREATE TYPE ingest_log_item AS ENUM('Author', 'Resource', 'Project', 'Version');

CREATE TABLE IF NOT EXISTS ingest_log (
  id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  action ingest_log_action NOT NULL,
  repository ingest_log_repository NOT NULL,
  item ingest_log_item NOT NULL,
  date_started timestamptz NOT NULL,
  date_finished timestamptz NOT NULL,
  items_processed integer NOT NULL,
  success boolean NOT NULL
);

-- Indexes

-- B-tree indexes for ordering by date_created
CREATE INDEX IF NOT EXISTS common_project_spigot_modrinth_hangar_date_created_index
ON common_project (GREATEST(spigot_date_created, modrinth_date_created, hangar_date_created) DESC NULLS LAST);

CREATE INDEX IF NOT EXISTS common_project_spigot_modrinth_date_created_index
ON common_project (GREATEST(spigot_date_created, modrinth_date_created, NULL) DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_spigot_hangar_date_created_index
ON common_project (GREATEST(spigot_date_created, NULL, hangar_date_created) DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_modrinth_hangar_date_created_index
ON common_project (GREATEST(NULL, modrinth_date_created, hangar_date_created) DESC NULLS LAST);

CREATE INDEX IF NOT EXISTS common_project_spigot_date_created_index
ON common_project (GREATEST(spigot_date_created, NULL, NULL) DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_modrinth_date_created_index
ON common_project (GREATEST(NULL, modrinth_date_created, NULL) DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_hangar_date_created_index
ON common_project (GREATEST(NULL, NULL, hangar_date_created) DESC NULLS LAST);

-- B-tree indexes for ordering by date_updated
CREATE INDEX IF NOT EXISTS common_project_spigot_modrinth_hangar_date_updated_index
ON common_project (GREATEST(spigot_date_updated, modrinth_date_updated, hangar_date_updated) DESC NULLS LAST);

CREATE INDEX IF NOT EXISTS common_project_spigot_modrinth_date_updated_index
ON common_project (GREATEST(spigot_date_updated, modrinth_date_updated, NULL) DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_spigot_hangar_date_updated_index
ON common_project (GREATEST(spigot_date_updated, NULL, hangar_date_updated) DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_modrinth_hangar_date_updated_index
ON common_project (GREATEST(NULL, modrinth_date_updated, hangar_date_updated) DESC NULLS LAST);

CREATE INDEX IF NOT EXISTS common_project_spigot_date_updated_index
ON common_project (GREATEST(spigot_date_updated, NULL, NULL) DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_modrinth_date_updated_index
ON common_project (GREATEST(NULL, modrinth_date_updated, NULL) DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_hangar_date_updated_index
ON common_project (GREATEST(NULL, NULL, hangar_date_updated) DESC NULLS LAST);

-- B-tree indexes for ordering by latest_minecraft_version
CREATE INDEX IF NOT EXISTS common_project_spigot_modrinth_hangar_latest_minecraft_version_index
ON common_project (GREATEST(spigot_latest_minecraft_version, modrinth_latest_minecraft_version, hangar_latest_minecraft_version) DESC NULLS LAST);

CREATE INDEX IF NOT EXISTS common_project_spigot_modrinth_latest_minecraft_version_index
ON common_project (GREATEST(spigot_latest_minecraft_version, modrinth_latest_minecraft_version, NULL) DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_spigot_hangar_latest_minecraft_version_index
ON common_project (GREATEST(spigot_latest_minecraft_version, NULL, hangar_latest_minecraft_version) DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_modrinth_hangar_latest_minecraft_version_index
ON common_project (GREATEST(NULL, modrinth_latest_minecraft_version, hangar_latest_minecraft_version) DESC NULLS LAST);

CREATE INDEX IF NOT EXISTS common_project_spigot_latest_minecraft_version_index
ON common_project (GREATEST(spigot_latest_minecraft_version, NULL, NULL) DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_modrinth_latest_minecraft_version_index
ON common_project (GREATEST(NULL, modrinth_latest_minecraft_version, NULL) DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_hangar_latest_minecraft_version_index
ON common_project (GREATEST(NULL, NULL, hangar_latest_minecraft_version) DESC NULLS LAST);

-- B-tree indexes for ordering by downloads
CREATE INDEX IF NOT EXISTS common_project_spigot_modrinth_hangar_downloads_index
ON common_project ((COALESCE(spigot_downloads, 0) + COALESCE(modrinth_downloads, 0) + COALESCE(hangar_downloads, 0)) DESC NULLS LAST);

CREATE INDEX IF NOT EXISTS common_project_spigot_modrinth_downloads_index
ON common_project ((COALESCE(spigot_downloads, 0) + COALESCE(modrinth_downloads, 0) + 0) DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_spigot_hangar_downloads_index
ON common_project ((COALESCE(spigot_downloads, 0) + 0 + COALESCE(hangar_downloads, 0)) DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_modrinth_hangar_downloads_index
ON common_project ((0 + COALESCE(modrinth_downloads, 0) + COALESCE(hangar_downloads, 0)) DESC NULLS LAST);

CREATE INDEX IF NOT EXISTS common_project_spigot_downloads_index
ON common_project ((COALESCE(spigot_downloads, 0) + 0 + 0) DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_modrinth_downloads_index
ON common_project ((0 + COALESCE(modrinth_downloads, 0) + 0) DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_hangar_downloads_index
ON common_project ((0 + 0 + COALESCE(hangar_downloads, 0)) DESC NULLS LAST);

-- B-tree indexes for ordering by likes and stars
CREATE INDEX IF NOT EXISTS common_project_spigot_hangar_likes_and_stars_index
ON common_project ((COALESCE(spigot_likes, 0) + COALESCE(hangar_stars, 0)) DESC NULLS LAST);

CREATE INDEX IF NOT EXISTS common_project_spigot_likes_index
ON common_project ((COALESCE(spigot_likes, 0) + 0) DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_hangar_stars_index
ON common_project ((0 + COALESCE(hangar_stars, 0)) DESC NULLS LAST);

-- B-tree indexes for ordering by follows and watchers
CREATE INDEX IF NOT EXISTS common_project_modrinth_hangar_follows_and_watchers_index
ON common_project ((COALESCE(modrinth_follows, 0) + COALESCE(hangar_watchers, 0)) DESC NULLS LAST);

CREATE INDEX IF NOT EXISTS common_project_modrinth_follows_index
ON common_project ((COALESCE(modrinth_follows, 0) + 0) DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_hangar_watchers_index
ON common_project ((0 + COALESCE(hangar_watchers, 0)) DESC NULLS LAST);

-- Trigram indexes for text search on name, description, and author
CREATE INDEX IF NOT EXISTS common_project_name_index
ON common_project
USING gin (spigot_name gin_trgm_ops, modrinth_name gin_trgm_ops, hangar_name gin_trgm_ops);

CREATE INDEX IF NOT EXISTS common_project_description_index
ON common_project
USING gin (spigot_description gin_trgm_ops, modrinth_description gin_trgm_ops, hangar_description gin_trgm_ops);

CREATE INDEX IF NOT EXISTS common_project_author_index
ON common_project
USING gin (spigot_author gin_trgm_ops, modrinth_author gin_trgm_ops, hangar_author gin_trgm_ops);
