--: SpigotResourceEntity(parsed_name?, latest_minecraft_version?, version_name?, icon_url?, icon_data?, source_url?, source_repository_host?, source_repository_owner?, source_repository_name?, source_repository_id?)

--! upsert_spigot_resource (parsed_name?, latest_minecraft_version?, version_name?, icon_url?, icon_data?, source_url?, source_repository_host?, source_repository_owner?, source_repository_name?)
INSERT INTO spigot_resource (id, name, parsed_name, description, slug, date_created, date_updated, latest_minecraft_version, downloads, likes, author_id, version_id, version_name, premium, abandoned, icon_url, icon_data, source_url, source_repository_host, source_repository_owner, source_repository_name)
  VALUES (:id, :name, :parsed_name, :description, :slug, :date_created, :date_updated, :latest_minecraft_version, :downloads, :likes, :author_id, :version_id, :version_name, :premium, :abandoned, :icon_url, :icon_data, :source_url, :source_repository_host, :source_repository_owner, :source_repository_name)
  ON CONFLICT (id)
  DO UPDATE SET
    name = EXCLUDED.name,
    parsed_name = EXCLUDED.parsed_name,
    description = EXCLUDED.description,
    slug = EXCLUDED.slug,
    date_created = EXCLUDED.date_created,
    date_updated = EXCLUDED.date_updated,
    latest_minecraft_version = EXCLUDED.latest_minecraft_version,
    downloads = EXCLUDED.downloads,
    likes = EXCLUDED.likes,
    author_id = EXCLUDED.author_id,
    version_id = EXCLUDED.version_id,
    version_name = EXCLUDED.version_name,
    premium = EXCLUDED.premium,
    abandoned = EXCLUDED.abandoned,
    icon_url = EXCLUDED.icon_url,
    icon_data = EXCLUDED.icon_data,
    source_url = EXCLUDED.source_url,
    source_repository_host = EXCLUDED.source_repository_host,
    source_repository_owner = EXCLUDED.source_repository_owner,
    source_repository_name = EXCLUDED.source_repository_name;

--! get_spigot_resources : SpigotResourceEntity
SELECT * FROM spigot_resource;

--! get_latest_spigot_resource_update_date
SELECT max(date_updated) FROM spigot_resource;
