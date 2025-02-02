CREATE TYPE ingest_log_action AS ENUM('Populate', 'Update');
CREATE TYPE ingest_log_repository AS ENUM('Spigot', 'Modrinth', 'Hangar');
CREATE TYPE ingest_log_item AS ENUM('Author', 'Resource', 'Project', 'Version');

CREATE TABLE IF NOT EXISTS ingest_log (
  id integer PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  action ingest_log_action NOT NULL,
  repository ingest_log_repository NOT NULL,
  item ingest_log_item NOT NULL,
  date_started timestamptz NOT NULL,
  date_finished timestamptz NOT NULL,
  items_processed integer NOT NULL
);