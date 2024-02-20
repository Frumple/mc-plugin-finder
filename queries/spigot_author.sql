--: SpigotAuthorEntity()

--! insert_spigot_author
INSERT INTO spigot_author (id, name) VALUES (:id, :name);

--! get_spigot_authors : SpigotAuthorEntity
SELECT id, name FROM spigot_author;

--! get_highest_spigot_author_id
SELECT max(id) from spigot_author;