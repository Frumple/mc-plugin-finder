--: HangarProjectEntity(source_code_link?, source_repository_host?, source_repository_owner?, source_repository_name?)

--! upsert_hangar_project (source_code_link?, source_repository_host?, source_repository_owner?, source_repository_name?)
INSERT INTO hangar_project (slug, owner, name, description, created_at, last_updated, visibility, avatar_url, version, source_code_link, source_repository_host, source_repository_owner, source_repository_name)
  VALUES (:slug, :owner, :name, :description, :created_at, :last_updated, :visibility, :avatar_url, :version, :source_code_link, :source_repository_host, :source_repository_owner, :source_repository_name)
  ON CONFLICT (slug)
  DO UPDATE SET
    owner = EXCLUDED.owner,
    name = EXCLUDED.name,
    description = EXCLUDED.description,
    created_at = EXCLUDED.created_at,
    last_updated = EXCLUDED.last_updated,
    visibility = EXCLUDED.visibility,
    avatar_url = EXCLUDED.avatar_url,
    version = EXCLUDED.version,
    source_code_link = EXCLUDED.source_code_link,
    source_repository_host = EXCLUDED.source_repository_host,
    source_repository_owner = EXCLUDED.source_repository_owner,
    source_repository_name = EXCLUDED.source_repository_name;

--! get_hangar_projects : HangarProjectEntity
SELECT * FROM hangar_project;

--! get_latest_hangar_project_update_date
SELECT max(last_updated) FROM hangar_project;