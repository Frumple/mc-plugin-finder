--: SpigotResourceEntity(version_name?, premium?, source_code_link?)

--! upsert_spigot_resource (version_name?, premium?, source_code_link?)
INSERT INTO spigot_resource (id, name, tag, slug, release_date, update_date, author_id, version_id, version_name, premium, source_code_link)
  VALUES (:id, :name, :tag, :slug, :release_date, :update_date, :author_id, :version_id, :version_name, :premium, :source_code_link)
  ON CONFLICT (id)
  DO UPDATE SET
    name = EXCLUDED.name,
    tag = EXCLUDED.tag,
    slug = EXCLUDED.slug,
    release_date = EXCLUDED.release_date,
    update_date = EXCLUDED.update_date,
    author_id = EXCLUDED.author_id,
    version_id = EXCLUDED.version_id,
    version_name = EXCLUDED.version_name,
    premium = EXCLUDED.premium,
    source_code_link = EXCLUDED.source_code_link;

--! get_spigot_resources : SpigotResourceEntity
SELECT * FROM spigot_resource;

--! get_latest_spigot_resource_update_date
SELECT max(update_date) from spigot_resource;
