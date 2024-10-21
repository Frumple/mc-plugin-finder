--: CommonProjectEntity(id?, spigot_id?, spigot_name?, spigot_description?, spigot_author?, modrinth_id?, modrinth_name?, modrinth_description?, modrinth_author?, hangar_slug?, hangar_name?, hangar_description?, hangar_author?)

--! get_merged_common_projects : CommonProjectEntity
SELECT
  COALESCE(cs.id, cm.id, ch.id) AS id,
  GREATEST(s.date_created, m.date_created, h.date_created) AS date_created,
  GREATEST(s.date_updated, m.date_updated, h.date_updated) AS date_updated,
  s.id AS spigot_id,
  s.parsed_name AS spigot_name,
  s.description AS spigot_description,
  a.name AS spigot_author,
  m.id AS modrinth_id,
  m.name AS modrinth_name,
  m.description AS modrinth_description,
  m.author AS modrinth_author,
  h.slug AS hangar_slug,
  h.name AS hangar_name,
  h.description AS hangar_description,
  h.author AS hangar_author
FROM
  spigot_resource s
  INNER JOIN spigot_author a
  ON  s.author_id = a.id

  FULL JOIN modrinth_project m
  ON  s.source_repository_host = m.source_repository_host
  AND s.source_repository_owner = m.source_repository_owner
  AND s.source_repository_name = m.source_repository_name

  FULL JOIN hangar_project h
  ON  COALESCE(s.source_repository_host, m.source_repository_host) = h.source_repository_host
  AND COALESCE(s.source_repository_owner, m.source_repository_owner) = h.source_repository_owner
  AND COALESCE(s.source_repository_name, m.source_repository_name) = h.source_repository_name

  LEFT JOIN common_project cs
  ON  s.id = cs.spigot_id

  LEFT JOIN common_project cm
  ON  m.id = cm.modrinth_id

  LEFT JOIN common_project ch
  ON  h.slug = ch.hangar_slug

WHERE
  GREATEST(s.date_updated, m.date_updated, h.date_updated) > :date_updated;

--! upsert_common_project (id?, spigot_id?, spigot_name?, spigot_description?, spigot_author?, modrinth_id?, modrinth_name?, modrinth_description?, modrinth_author?, hangar_slug?, hangar_name?, hangar_description?, hangar_author?)
INSERT INTO common_project (id, date_created, date_updated, spigot_id, spigot_name, spigot_description, spigot_author, modrinth_id, modrinth_name, modrinth_description, modrinth_author, hangar_slug, hangar_name, hangar_description, hangar_author)
  VALUES (COALESCE(:id, nextval('common_project_id_seq')), :date_created, :date_updated, :spigot_id, :spigot_name, :spigot_description, :spigot_author, :modrinth_id, :modrinth_name, :modrinth_description, :modrinth_author, :hangar_slug, :hangar_name, :hangar_description, :hangar_author)
  ON CONFLICT (id)
  DO UPDATE SET
    date_created = EXCLUDED.date_created,
    date_updated = EXCLUDED.date_updated,
    spigot_id = EXCLUDED.spigot_id,
    spigot_name = EXCLUDED.spigot_name,
    spigot_description = EXCLUDED.spigot_description,
    spigot_author = EXCLUDED.spigot_author,
    modrinth_id = EXCLUDED.modrinth_id,
    modrinth_name = EXCLUDED.modrinth_name,
    modrinth_description = EXCLUDED.modrinth_description,
    modrinth_author = EXCLUDED.modrinth_author,
    hangar_slug = EXCLUDED.hangar_slug,
    hangar_name = EXCLUDED.hangar_name,
    hangar_description = EXCLUDED.hangar_description,
    hangar_author = EXCLUDED.hangar_author;

--! search_common_projects (query, spigot, modrinth, hangar, name, description, author, sort_field, sort_ascending) : CommonProjectEntity
SELECT
  *
FROM
  common_project
WHERE
  CASE :spigot IS TRUE AND :name IS TRUE
    WHEN TRUE THEN spigot_name ILIKE :query
    ELSE FALSE
  END

  OR

  CASE :spigot IS TRUE AND :description IS TRUE
    WHEN TRUE THEN spigot_description ILIKE :query
    ELSE FALSE
  END

  OR

  CASE :spigot IS TRUE AND :author IS TRUE
    WHEN TRUE THEN spigot_author ILIKE :query
    ELSE FALSE
  END

  OR

  CASE :modrinth IS TRUE AND :name IS TRUE
    WHEN TRUE THEN modrinth_name ILIKE :query
    ELSE FALSE
  END

  OR

  CASE :modrinth IS TRUE AND :description IS TRUE
    WHEN TRUE THEN modrinth_description ILIKE :query
    ELSE FALSE
  END

  OR

  CASE :modrinth IS TRUE AND :author IS TRUE
    WHEN TRUE THEN modrinth_author ILIKE :query
    ELSE FALSE
  END

  OR

  CASE :hangar IS TRUE AND :name IS TRUE
    WHEN TRUE THEN hangar_name ILIKE :query
    ELSE FALSE
  END

  OR

  CASE :hangar IS TRUE AND :description IS TRUE
    WHEN TRUE THEN hangar_description ILIKE :query
    ELSE FALSE
  END

  OR

  CASE :hangar IS TRUE AND :author IS TRUE
    WHEN TRUE THEN hangar_author ILIKE :query
    ELSE FALSE
  END

ORDER BY
  (CASE WHEN :sort_field = 'date_created' AND :sort_ascending IS TRUE THEN date_created END) ASC,
  (CASE WHEN :sort_field = 'date_created' AND :sort_ascending IS FALSE THEN date_created END) DESC,
  (CASE WHEN :sort_field = 'date_updated' AND :sort_ascending IS TRUE THEN date_updated END) ASC,
  (CASE WHEN :sort_field = 'date_updated' AND :sort_ascending IS FALSE THEN date_updated END) DESC;

--! get_common_projects : CommonProjectEntity
SELECT
  *
FROM
  common_project;