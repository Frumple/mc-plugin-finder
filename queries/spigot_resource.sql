--: SpigotResource()

--! get_spigot_resources : SpigotResource
SELECT * FROM spigot_resource;

--! insert_spigot_resource (version_name?, premium?, source_code_link?)
INSERT INTO spigot_resource (id, name, slug, release_date, update_date, author_id, version_id, version_name, premium, source_code_link)
  VALUES (:id, :name, :slug, :release_date, :update_date, :author_id, :version_id, :version_name, :premium, :source_code_link);