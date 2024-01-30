CREATE TABLE spigot_author (
  id serial NOT NULL,
  name text NOT NULL,
  PRIMARY KEY (id)
);

CREATE TABLE spigot_resource (
  id serial NOT NULL,
  name text NOT NULL,
  slug text NOT NULL,
  release_date timestamptz NOT NULL,
  update_date timestamptz NOT NULL,
  author_id integer NOT NULL REFERENCES spigot_author,
  version_id integer NOT NULL,
  version_name text,
  premium boolean,
  source_code_link text,
  PRIMARY KEY (id)
);