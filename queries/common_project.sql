--: CommonProjectEntity(id?, spigot_id?, spigot_name?, spigot_tag?, spigot_author?, modrinth_id?, modrinth_title?, modrinth_description?, modrinth_author?, hangar_slug?, hangar_name?, hangar_description?, hangar_owner?)

--! get_merged_common_projects : CommonProjectEntity
SELECT
  COALESCE(cs.id, cm.id, ch.id) AS id,
  GREATEST(sm.date_created, h.created_at) AS date_created,
  GREATEST(sm.date_updated, h.last_updated) AS date_updated,
  sm.spigot_id,
  sm.spigot_name,
  sm.spigot_tag,
  sm.spigot_author,
  sm.modrinth_id,
  sm.modrinth_title,
  sm.modrinth_description,
  sm.modrinth_author,
  h.slug AS hangar_slug,
  h.name AS hangar_name,
  h.description AS hangar_description,
  h.owner AS hangar_owner

FROM
  (
    SELECT
      GREATEST(s.release_date, m.date_created) AS date_created,
      GREATEST(s.update_date, m.date_modified) AS date_updated,
      s.id AS spigot_id,
      s.parsed_name AS spigot_name,
      s.tag AS spigot_tag,
      a.name AS spigot_author,
      m.id AS modrinth_id,
      m.title AS modrinth_title,
      m.description AS modrinth_description,
      m.author AS modrinth_author,
      COALESCE(s.source_repository_host, m.source_repository_host) AS source_repository_host,
    	COALESCE(s.source_repository_owner, m.source_repository_owner) AS source_repository_owner,
      COALESCE(s.source_repository_name, m.source_repository_name) AS source_repository_name
    FROM spigot_resource s
      INNER JOIN spigot_author a
      ON  s.author_id = a.id

      FULL OUTER JOIN modrinth_project m
      ON  s.source_repository_host = m.source_repository_host
      AND s.source_repository_owner = m.source_repository_owner
      AND s.source_repository_name = m.source_repository_name
  ) sm

  FULL OUTER JOIN hangar_project h
  ON  sm.source_repository_host = h.source_repository_host
  AND sm.source_repository_owner = h.source_repository_owner
  AND sm.source_repository_name = h.source_repository_name

  LEFT JOIN common_project cs
  ON  sm.spigot_id = cs.spigot_id

  LEFT JOIN common_project cm
  ON  sm.modrinth_id = cm.modrinth_id

  LEFT JOIN common_project ch
  ON  h.slug = ch.hangar_slug

WHERE
  GREATEST(sm.date_updated, h.last_updated) > :date_updated;

--! upsert_common_project (id?, spigot_id?, spigot_name?, spigot_tag?, spigot_author?, modrinth_id?, modrinth_title?, modrinth_description?, modrinth_author?, hangar_slug?, hangar_name?, hangar_description?, hangar_owner?)
INSERT INTO common_project (id, date_created, date_updated, spigot_id, spigot_name, spigot_tag, spigot_author, modrinth_id, modrinth_title, modrinth_description, modrinth_author, hangar_slug, hangar_name, hangar_description, hangar_owner)
  VALUES (COALESCE(:id, nextval('common_project_id_seq')), :date_created, :date_updated, :spigot_id, :spigot_name, :spigot_tag, :spigot_author, :modrinth_id, :modrinth_title, :modrinth_description, :modrinth_author, :hangar_slug, :hangar_name, :hangar_description, :hangar_owner)
  ON CONFLICT (id)
  DO UPDATE SET
    date_created = EXCLUDED.date_created,
    date_updated = EXCLUDED.date_updated,
    spigot_id = EXCLUDED.spigot_id,
    spigot_name = EXCLUDED.spigot_name,
    spigot_tag = EXCLUDED.spigot_tag,
    spigot_author = EXCLUDED.spigot_author,
    modrinth_id = EXCLUDED.modrinth_id,
    modrinth_title = EXCLUDED.modrinth_title,
    modrinth_description = EXCLUDED.modrinth_description,
    modrinth_author = EXCLUDED.modrinth_author,
    hangar_slug = EXCLUDED.hangar_slug,
    hangar_name = EXCLUDED.hangar_name,
    hangar_description = EXCLUDED.hangar_description,
    hangar_owner = EXCLUDED.hangar_owner;

--! get_common_projects : CommonProjectEntity
SELECT * FROM common_project;