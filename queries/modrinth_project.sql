--: ModrinthProjectEntity(version_id?, version_name?, icon_url?, monetization_status?, source_url?, source_repository_host?, source_repository_owner?, source_repository_name?)

--! upsert_modrinth_project (version_id?, version_name?, icon_url?, monetization_status?, source_url?, source_repository_host?, source_repository_owner?, source_repository_name?)
INSERT INTO modrinth_project (id, slug, name, description, author, date_created, date_updated, downloads, follows, version_id, version_name, icon_url, monetization_status, source_url, source_repository_host, source_repository_owner, source_repository_name)
  VALUES (:id, :slug, :name, :description, :author, :date_created, :date_updated, :downloads, :follows, :version_id, :version_name, :icon_url, :monetization_status, :source_url, :source_repository_host, :source_repository_owner, :source_repository_name)
  ON CONFLICT(id)
  DO UPDATE SET
    id = EXCLUDED.id,
    slug = EXCLUDED.slug,
    name = EXCLUDED.name,
    description = EXCLUDED.description,
    author = EXCLUDED.author,
    date_created = EXCLUDED.date_created,
    date_updated = EXCLUDED.date_updated,
    downloads = EXCLUDED.downloads,
    follows = EXCLUDED.follows,
    version_id = EXCLUDED.version_id,
    version_name = EXCLUDED.version_name,
    icon_url = EXCLUDED.icon_url,
    monetization_status = EXCLUDED.monetization_status,
    source_url = EXCLUDED.source_url,
    source_repository_host = EXCLUDED.source_repository_host,
    source_repository_owner = EXCLUDED.source_repository_owner,
    source_repository_name = EXCLUDED.source_repository_name;

--! get_modrinth_projects : ModrinthProjectEntity
SELECT * FROM modrinth_project;

--! get_latest_modrinth_project_update_date
SELECT max(date_updated) FROM modrinth_project;