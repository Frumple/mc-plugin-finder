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
  icon_url text,
  monetization_status text,
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
  ON  s.source_repository_host = m.source_repository_host
  AND s.source_repository_owner = m.source_repository_owner
  AND s.source_repository_name = m.source_repository_name

  FULL JOIN hangar_project h
  ON  COALESCE(s.source_repository_host, m.source_repository_host) = h.source_repository_host
  AND COALESCE(s.source_repository_owner, m.source_repository_owner) = h.source_repository_owner
  AND COALESCE(s.source_repository_name, m.source_repository_name) = h.source_repository_name;

-- Indexes

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
