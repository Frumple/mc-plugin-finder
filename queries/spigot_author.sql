--: SpigotAuthor()

--! get_spigot_authors : SpigotAuthor
SELECT id, name FROM spigot_author;

--! insert_spigot_author
INSERT INTO spigot_author (id, name) VALUES (:id, :name);