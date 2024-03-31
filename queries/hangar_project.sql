--: HangarProjectEntity(version_name?, source_url?, source_repository_host?, source_repository_owner?, source_repository_name?)

--! upsert_hangar_project (version_name?, source_url?, source_repository_host?, source_repository_owner?, source_repository_name?)
INSERT INTO hangar_project (slug, author, name, description, date_created, date_updated, downloads, visibility, avatar_url, version_name, source_url, source_repository_host, source_repository_owner, source_repository_name)
  VALUES (:slug, :author, :name, :description, :date_created, :date_updated, :downloads, :visibility, :avatar_url, :version_name, :source_url, :source_repository_host, :source_repository_owner, :source_repository_name)
  ON CONFLICT (slug)
  DO UPDATE SET
    author = EXCLUDED.author,
    name = EXCLUDED.name,
    description = EXCLUDED.description,
    date_created = EXCLUDED.date_created,
    date_updated = EXCLUDED.date_updated,
    downloads = EXCLUDED.downloads,
    visibility = EXCLUDED.visibility,
    avatar_url = EXCLUDED.avatar_url,
    version_name = EXCLUDED.version_name,
    source_url = EXCLUDED.source_url,
    source_repository_host = EXCLUDED.source_repository_host,
    source_repository_owner = EXCLUDED.source_repository_owner,
    source_repository_name = EXCLUDED.source_repository_name;

--! get_hangar_projects : HangarProjectEntity
SELECT * FROM hangar_project;

--! get_latest_hangar_project_update_date
SELECT max(date_updated) FROM hangar_project;