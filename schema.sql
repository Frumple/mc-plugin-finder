-- Enable trigram module for text search
CREATE EXTENSION IF NOT EXISTS pg_trgm;

-- Tables

-- Spigot
CREATE UNLOGGED TABLE IF NOT EXISTS spigot_author (
  id SERIAL PRIMARY KEY,
  name text NOT NULL
);

CREATE UNLOGGED TABLE IF NOT EXISTS spigot_resource (
  id SERIAL PRIMARY KEY,
  name text NOT NULL,
  parsed_name text,
  description text NOT NULL,
  slug text NOT NULL,
  date_created timestamptz NOT NULL,
  date_updated timestamptz NOT NULL,
  downloads integer NOT NULL,
  likes integer NOT NULL,
  author_id integer NOT NULL REFERENCES spigot_author,
  version_id integer NOT NULL,
  version_name text,
  premium boolean NOT NULL,
  source_url text,
  source_repository_host text,
  source_repository_owner text,
  source_repository_name text
);

-- Modrinth
CREATE UNLOGGED TABLE IF NOT EXISTS modrinth_project (
  id text PRIMARY KEY,
  slug text NOT NULL,
  name text NOT NULL,
  description text NOT NULL,
  author text NOT NULL,
  date_created timestamptz NOT NULL,
  date_updated timestamptz NOT NULL,
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
CREATE UNLOGGED TABLE IF NOT EXISTS hangar_project (
  slug text PRIMARY KEY,
  author text NOT NULL,
  name text NOT NULL,
  description text NOT NULL,
  date_created timestamptz NOT NULL,
  date_updated timestamptz NOT NULL,
  downloads integer NOT NULL,
  stars integer NOT NULL,
  watchers integer NOT NULL,
  visibility text NOT NULL,
  avatar_url text NOT NULL,
  version_name text,
  source_url text,
  source_repository_host text,
  source_repository_owner text,
  source_repository_name text
);

-- Common
CREATE UNLOGGED TABLE IF NOT EXISTS common_project (
  id SERIAL PRIMARY KEY,
  spigot_id integer REFERENCES spigot_resource,
  spigot_name text,
  spigot_description text,
  spigot_author text,
  modrinth_id text REFERENCES modrinth_project,
  modrinth_name text,
  modrinth_description text,
  modrinth_author text,
  hangar_slug text REFERENCES hangar_project,
  hangar_name text,
  hangar_description text,
  hangar_author text
);

-- Indexes

-- B-tree indexes for joining on spigot_id, modrinth_id, and hangar_slug
CREATE INDEX IF NOT EXISTS common_project_spigot_id_index
ON common_project (spigot_id);

CREATE INDEX IF NOT EXISTS common_project_modrinth_id_index
ON common_project (modrinth_id);

CREATE INDEX IF NOT EXISTS common_project_hangar_slug_index
ON common_project (hangar_slug);

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
