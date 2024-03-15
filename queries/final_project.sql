--: FinalProjectEntity(id?, spigot_id?, spigot_name?, spigot_author?, spigot_tag?, hangar_slug?, hangar_name?, hangar_owner?, hangar_description?)

--! get_merged_projects : FinalProjectEntity
SELECT f.id AS id, GREATEST(s.release_date, h.created_at) AS release_date, GREATEST(s.update_date, h.last_updated) AS update_date, s.id AS spigot_id, s.parsed_name AS spigot_name, a.name AS spigot_author, s.tag AS spigot_tag, h.slug AS hangar_slug, h.name AS hangar_name, h.owner AS hangar_owner, h.description AS hangar_description
  FROM spigot_resource s
  INNER JOIN spigot_author a
  ON  s.author_id = a.id

  FULL OUTER JOIN hangar_project h
  ON  s.source_repository_host = h.source_repository_host
  AND s.source_repository_owner = h.source_repository_owner
  AND s.source_repository_name = h.source_repository_name

  LEFT JOIN final_project f
  ON  f.spigot_id = spigot_id
  OR  f.hangar_slug = hangar_slug;

--! upsert_final_project (id?, spigot_id?, spigot_name?, spigot_author?, spigot_tag?, hangar_slug?, hangar_name?, hangar_owner?, hangar_description?)
INSERT INTO final_project (id, release_date, update_date, spigot_id, spigot_name, spigot_author, spigot_tag, hangar_slug, hangar_name, hangar_owner, hangar_description)
  VALUES (COALESCE(:id, nextval('final_project_id_seq')), :release_date, :update_date, :spigot_id, :spigot_name, :spigot_author, :spigot_tag, :hangar_slug, :hangar_name, :hangar_owner, :hangar_description)
  ON CONFLICT (id)
  DO UPDATE SET
    release_date = EXCLUDED.release_date,
    update_date = EXCLUDED.update_date,
    spigot_id = EXCLUDED.spigot_id,
    spigot_name = EXCLUDED.spigot_name,
    spigot_author = EXCLUDED.spigot_author,
    spigot_tag = EXCLUDED.spigot_tag,
    hangar_slug = EXCLUDED.hangar_slug,
    hangar_name = EXCLUDED.hangar_name,
    hangar_owner = EXCLUDED.hangar_owner,
    hangar_description = EXCLUDED.hangar_description;

--! get_final_projects : FinalProjectEntity
SELECT * FROM final_project;