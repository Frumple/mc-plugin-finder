--: SpigotAuthorEntity()

--! insert_spigot_author
INSERT INTO spigot_author (id, name)
  VALUES (:id, :name)
  ON CONFLICT DO NOTHING;

--! get_spigot_authors : SpigotAuthorEntity
SELECT id, name FROM spigot_author;