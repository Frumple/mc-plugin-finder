CREATE TABLE spigot_author (
  id serial NOT NULL,
  name text NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE spigot_resource (
  id serial NOT NULL,
  name text NOT NULL,
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
  source_repository_name text,
  PRIMARY KEY (id)
);

CREATE TABLE hangar_project (
  slug text NOT NULL,
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
  source_repository_name text,
  PRIMARY KEY (slug)
);