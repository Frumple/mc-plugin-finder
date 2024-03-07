--: SpigotResourceEntity(parsed_name?, version_name?, premium?, source_code_link?, source_repository_host?, source_repository_owner?, source_repository_name?)

--! upsert_spigot_resource (parsed_name?, version_name?, premium?, source_code_link?, source_repository_host?, source_repository_owner?, source_repository_name?)
INSERT INTO spigot_resource (id, name, parsed_name, tag, slug, release_date, update_date, author_id, version_id, version_name, premium, source_code_link, source_repository_host, source_repository_owner, source_repository_name)
  VALUES (:id, :name, :parsed_name, :tag, :slug, :release_date, :update_date, :author_id, :version_id, :version_name, :premium, :source_code_link, :source_repository_host, :source_repository_owner, :source_repository_name)
  ON CONFLICT (id)
  DO UPDATE SET
    name = EXCLUDED.name,
    parsed_name = EXCLUDED.parsed_name,
    tag = EXCLUDED.tag,
    slug = EXCLUDED.slug,
    release_date = EXCLUDED.release_date,
    update_date = EXCLUDED.update_date,
    author_id = EXCLUDED.author_id,
    version_id = EXCLUDED.version_id,
    version_name = EXCLUDED.version_name,
    premium = EXCLUDED.premium,
    source_code_link = EXCLUDED.source_code_link,
    source_repository_host = EXCLUDED.source_repository_host,
    source_repository_owner = EXCLUDED.source_repository_owner,
    source_repository_name = EXCLUDED.source_repository_name;

--! get_spigot_resources : SpigotResourceEntity
SELECT * FROM spigot_resource;

--! get_latest_spigot_resource_update_date
SELECT max(update_date) FROM spigot_resource;
