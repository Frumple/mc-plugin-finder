--: ModrinthProjectEntity(version_name?, icon_url?, monetization_status?, source_code_link?, source_repository_host?, source_repository_owner?, source_repository_name?)

--! upsert_modrinth_project (version_name?, icon_url?, monetization_status?, source_code_link?, source_repository_host?, source_repository_owner?, source_repository_name?)
INSERT INTO modrinth_project (id, slug, title, description, author, date_created, date_modified, downloads, version_id, version_name, icon_url, monetization_status, source_code_link, source_repository_host, source_repository_owner, source_repository_name)
  VALUES (:id, :slug, :title, :description, :author, :date_created, :date_modified, :downloads, :version_id, :version_name, :icon_url, :monetization_status, :source_code_link, :source_repository_host, :source_repository_owner, :source_repository_name)
  ON CONFLICT(id)
  DO UPDATE SET
    id = EXCLUDED.id,
    slug = EXCLUDED.slug,
    title = EXCLUDED.title,
    description = EXCLUDED.description,
    author = EXCLUDED.author,
    date_created = EXCLUDED.date_created,
    date_modified = EXCLUDED.date_modified,
    downloads = EXCLUDED.downloads,
    version_id = EXCLUDED.version_id,
    version_name = EXCLUDED.version_name,
    icon_url = EXCLUDED.icon_url,
    monetization_status = EXCLUDED.monetization_status,
    source_code_link = EXCLUDED.source_code_link,
    source_repository_host = EXCLUDED.source_repository_host,
    source_repository_owner = EXCLUDED.source_repository_owner,
    source_repository_name = EXCLUDED.source_repository_name;

--! get_modrinth_projects : ModrinthProjectEntity
SELECT * FROM modrinth_project;

--! get_latest_modrinth_project_update_date
SELECT max(date_modified) FROM modrinth_project;