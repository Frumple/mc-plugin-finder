CREATE TABLE IF NOT EXISTS spigot_author (
  id SERIAL NOT NULL PRIMARY KEY,
  name text NOT NULL
);

CREATE TABLE IF NOT EXISTS spigot_resource (
  id SERIAL NOT NULL PRIMARY KEY,
  name text NOT NULL,
  parsed_name text,
  tag text NOT NULL,
  slug text NOT NULL,
  release_date timestamptz NOT NULL,
  update_date timestamptz NOT NULL,
  author_id integer NOT NULL REFERENCES spigot_author,
  version_id integer NOT NULL,
  version_name text,
  premium boolean,
  source_code_link text,
  source_repository_host text,
  source_repository_owner text,
  source_repository_name text
);

CREATE TABLE IF NOT EXISTS hangar_project (
  slug text NOT NULL PRIMARY KEY,
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

CREATE TABLE IF NOT EXISTS common_project (
  id SERIAL NOT NULL PRIMARY KEY,
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