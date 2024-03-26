-- Enable trigram module for text search
CREATE EXTENSION IF NOT EXISTS pg_trgm;

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
  tag text NOT NULL,
  slug text NOT NULL,
  release_date timestamptz NOT NULL,
  update_date timestamptz NOT NULL,
  downloads integer NOT NULL,
  author_id integer NOT NULL REFERENCES spigot_author,
  version_id integer NOT NULL,
  version_name text,
  premium boolean,
  source_url text,
  source_repository_host text,
  source_repository_owner text,
  source_repository_name text
);

-- Modrinth
CREATE TABLE IF NOT EXISTS modrinth_project (
  id text PRIMARY KEY,
  slug text NOT NULL,
  title text NOT NULL,
  description text NOT NULL,
  author text NOT NULL,
  date_created timestamptz NOT NULL,
  date_modified timestamptz NOT NULL,
  downloads integer NOT NULL,
  version_id text NOT NULL,
  version_name text,
  icon_url text,
  monetization_status text,
  source_code_link text,
  source_repository_host text,
  source_repository_owner text,
  source_repository_name text
);

-- Hangar
CREATE TABLE IF NOT EXISTS hangar_project (
  slug text PRIMARY KEY,
  owner text NOT NULL,
  name text NOT NULL,
  description text NOT NULL,
  created_at timestamptz NOT NULL,
  last_updated timestamptz NOT NULL,
  visibility text NOT NULL,
  avatar_url text NOT NULL,
  version text NOT NULL,
  source_code_link text,
  source_repository_host text,
  source_repository_owner text,
  source_repository_name text
);

-- Common
CREATE TABLE IF NOT EXISTS common_project (
  id SERIAL PRIMARY KEY,
  date_created timestamptz NOT NULL,
  date_updated timestamptz NOT NULL,
  spigot_id integer REFERENCES spigot_resource,
  spigot_name text,
  spigot_author text,
  spigot_tag text,
  hangar_slug text REFERENCES hangar_project,
  hangar_name text,
  hangar_owner text,
  hangar_description text
);

-- Indexes

-- Trigram indexes for text search on name, description, and author
CREATE INDEX IF NOT EXISTS common_project_name_index
ON common_project
USING gin (spigot_name gin_trgm_ops, hangar_name gin_trgm_ops);

CREATE INDEX IF NOT EXISTS common_project_description_index
ON common_project
USING gin (spigot_tag gin_trgm_ops, hangar_description gin_trgm_ops);

CREATE INDEX IF NOT EXISTS common_project_author_index
ON common_project
USING gin (spigot_author gin_trgm_ops, hangar_owner gin_trgm_ops);

-- B-tree indexes for ordering by date_created and date_updated
CREATE INDEX IF NOT EXISTS common_project_date_created_index
ON common_project (date_created);

CREATE INDEX IF NOT EXISTS common_project_date_updated_index
ON common_project (date_updated);