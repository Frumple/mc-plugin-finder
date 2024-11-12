--: CommonProjectEntity(id?, spigot_id?, spigot_slug?, spigot_name?, spigot_description?, spigot_author?, spigot_version?, spigot_premium?, modrinth_id?, modrinth_slug?, modrinth_name?, modrinth_description?, modrinth_author?, modrinth_version?, hangar_slug?, hangar_name?, hangar_description?, hangar_author?, hangar_version?)
--: CommonProjectSearchResultEntity(id?, spigot_id?, spigot_slug?, spigot_name?, spigot_description?, spigot_author?, spigot_version?, spigot_premium?, modrinth_id?, modrinth_slug?, modrinth_name?, modrinth_description?, modrinth_author?, modrinth_version?, hangar_slug?, hangar_name?, hangar_description?, hangar_author?, hangar_version?, source_repository_host?, source_repository_owner?, source_repository_name?)

--! get_merged_common_projects : CommonProjectEntity
SELECT
  COALESCE(cs.id, cm.id, ch.id) AS id,

  s.id AS spigot_id,
  s.slug AS spigot_slug,
  s.parsed_name AS spigot_name,
  s.description AS spigot_description,
  a.name AS spigot_author,
  s.version_name AS spigot_version,
  s.premium AS spigot_premium,

  m.id AS modrinth_id,
  m.slug AS modrinth_slug,
  m.name AS modrinth_name,
  m.description AS modrinth_description,
  m.author AS modrinth_author,
  m.version_name AS modrinth_version,

  h.slug AS hangar_slug,
  h.name AS hangar_name,
  h.description AS hangar_description,
  h.author AS hangar_author,
  h.version_name AS hangar_version
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
INSERT INTO common_project (id, spigot_id, spigot_name, spigot_description, spigot_author, modrinth_id, modrinth_name, modrinth_description, modrinth_author, hangar_slug, hangar_name, hangar_description, hangar_author)
  VALUES (COALESCE(:id, nextval('common_project_id_seq')), :spigot_id, :spigot_name, :spigot_description, :spigot_author, :modrinth_id, :modrinth_name, :modrinth_description, :modrinth_author, :hangar_slug, :hangar_name, :hangar_description, :hangar_author)
  ON CONFLICT (id)
  DO UPDATE SET
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

--! get_common_projects : CommonProjectEntity
SELECT
  id,

  spigot_id,
  NULL AS spigot_slug,
  spigot_name,
  spigot_description,
  spigot_author,
  NULL AS spigot_version,
  FALSE AS spigot_premium,

  modrinth_id,
  NULL AS modrinth_slug,
  modrinth_name,
  modrinth_description,
  modrinth_author,
  NULL AS modrinth_version,

  hangar_slug,
  hangar_name,
  hangar_description,
  hangar_author,
  NULL AS hangar_version
FROM
  common_project;

--! search_common_projects (query, spigot, modrinth, hangar, name, description, author, sort, limit) : CommonProjectSearchResultEntity
SELECT
  c.id,
  CASE WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN GREATEST(s.date_created, m.date_created, h.date_created)
       WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS FALSE THEN GREATEST(s.date_created, m.date_created)
       WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS TRUE  THEN GREATEST(s.date_created, h.date_created)
       WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN GREATEST(m.date_created, h.date_created)
       WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS FALSE THEN s.date_created
       WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS FALSE THEN m.date_created
       WHEN :spigot IS FALSE AND :modrinth IS FALSE AND :hangar IS TRUE  THEN h.date_created
       ELSE timestamptz '-infinity'
  END
  AS date_created,

  CASE WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN GREATEST(s.date_updated, m.date_updated, h.date_updated)
       WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS FALSE THEN GREATEST(s.date_updated, m.date_updated)
       WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS TRUE  THEN GREATEST(s.date_updated, h.date_updated)
       WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN GREATEST(m.date_updated, h.date_updated)
       WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS FALSE THEN s.date_updated
       WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS FALSE THEN m.date_updated
       WHEN :spigot IS FALSE AND :modrinth IS FALSE AND :hangar IS TRUE  THEN h.date_updated
       ELSE timestamptz '-infinity'
  END
  AS date_updated,

  CASE WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN COALESCE(s.downloads, 0) + COALESCE(m.downloads, 0) + COALESCE(h.downloads, 0)
       WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS FALSE THEN COALESCE(s.downloads, 0) + COALESCE(m.downloads, 0)
       WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS TRUE  THEN COALESCE(s.downloads, 0) + COALESCE(h.downloads, 0)
       WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN COALESCE(m.downloads, 0) + COALESCE(h.downloads, 0)
       WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS FALSE THEN COALESCE(s.downloads, 0)
       WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS FALSE THEN COALESCE(m.downloads, 0)
       WHEN :spigot IS FALSE AND :modrinth IS FALSE AND :hangar IS TRUE  THEN COALESCE(h.downloads, 0)
       ELSE 0
  END
  AS downloads,

  CASE WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN COALESCE(s.likes, 0) + COALESCE(h.stars, 0)
       WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS FALSE THEN COALESCE(s.likes, 0)
       WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS TRUE  THEN COALESCE(s.likes, 0) + COALESCE(h.stars, 0)
       WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN COALESCE(h.stars, 0)
       WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS FALSE THEN COALESCE(s.likes, 0)
       WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS FALSE THEN 0
       WHEN :spigot IS FALSE AND :modrinth IS FALSE AND :hangar IS TRUE  THEN COALESCE(h.stars, 0)
       ELSE 0
  END
  AS likes_and_stars,

  CASE WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN COALESCE(m.follows, 0) + COALESCE(h.watchers, 0)
       WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS FALSE THEN COALESCE(m.follows, 0)
       WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS TRUE  THEN COALESCE(h.watchers, 0)
       WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN COALESCE(m.follows, 0) + COALESCE(h.watchers, 0)
       WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS FALSE THEN 0
       WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS FALSE THEN COALESCE(m.follows, 0)
       WHEN :spigot IS FALSE AND :modrinth IS FALSE AND :hangar IS TRUE  THEN COALESCE(h.watchers, 0)
       ELSE 0
  END
  AS follows_and_watchers,

  (CASE WHEN :spigot IS TRUE THEN c.spigot_id ELSE NULL END) AS spigot_id,
  (CASE WHEN :spigot IS TRUE THEN s.slug ELSE NULL END) AS spigot_slug,
  (CASE WHEN :spigot IS TRUE THEN c.spigot_name ELSE NULL END) AS spigot_name,
  (CASE WHEN :spigot IS TRUE THEN c.spigot_description ELSE NULL END) AS spigot_description,
  (CASE WHEN :spigot IS TRUE THEN c.spigot_author ELSE NULL END) AS spigot_author,
  (CASE WHEN :spigot IS TRUE THEN s.version_name ELSE NULL END) AS spigot_version,
  (CASE WHEN :spigot IS TRUE THEN s.premium ELSE NULL END) AS spigot_premium,

  (CASE WHEN :modrinth IS TRUE THEN c.modrinth_id ELSE NULL END) AS modrinth_id,
  (CASE WHEN :modrinth IS TRUE THEN m.slug ELSE NULL END) AS modrinth_slug,
  (CASE WHEN :modrinth IS TRUE THEN c.modrinth_name ELSE NULL END) AS modrinth_name,
  (CASE WHEN :modrinth IS TRUE THEN c.modrinth_description ELSE NULL END) AS modrinth_description,
  (CASE WHEN :modrinth IS TRUE THEN c.modrinth_author ELSE NULL END) AS modrinth_author,
  (CASE WHEN :modrinth IS TRUE THEN m.version_name ELSE NULL END) AS modrinth_version,

  (CASE WHEN :hangar IS TRUE THEN c.hangar_slug ELSE NULL END) AS hangar_slug,
  (CASE WHEN :hangar IS TRUE THEN c.hangar_name ELSE NULL END) AS hangar_name,
  (CASE WHEN :hangar IS TRUE THEN c.hangar_description ELSE NULL END) AS hangar_description,
  (CASE WHEN :hangar IS TRUE THEN c.hangar_author ELSE NULL END) AS hangar_author,
  (CASE WHEN :hangar IS TRUE THEN h.version_name ELSE NULL END) AS hangar_version,

  COALESCE(s.source_repository_host, m.source_repository_host, h.source_repository_host) AS source_repository_host,
  COALESCE(s.source_repository_owner, m.source_repository_owner, h.source_repository_owner) AS source_repository_owner,
  COALESCE(s.source_repository_name, m.source_repository_name, h.source_repository_name) AS source_repository_name
FROM
  common_project c
  LEFT JOIN spigot_resource s
  ON c.spigot_id = s.id

  LEFT JOIN modrinth_project m
  ON c.modrinth_id = m.id

  LEFT JOIN hangar_project h
  ON c.hangar_slug = h.slug
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
    CASE
      WHEN :sort = 'date_created' THEN
        CASE WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN GREATEST(s.date_created, m.date_created, h.date_created)
             WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS FALSE THEN GREATEST(s.date_created, m.date_created)
             WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS TRUE  THEN GREATEST(s.date_created, h.date_created)
             WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN GREATEST(m.date_created, h.date_created)
             WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS FALSE THEN s.date_created
             WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS FALSE THEN m.date_created
             WHEN :spigot IS FALSE AND :modrinth IS FALSE AND :hangar IS TRUE  THEN h.date_created
        END

      WHEN :sort = 'date_updated' THEN
        CASE WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN GREATEST(s.date_updated, m.date_updated, h.date_updated)
             WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS FALSE THEN GREATEST(s.date_updated, m.date_updated)
             WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS TRUE  THEN GREATEST(s.date_updated, h.date_updated)
             WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN GREATEST(m.date_updated, h.date_updated)
             WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS FALSE THEN s.date_updated
             WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS FALSE THEN m.date_updated
             WHEN :spigot IS FALSE AND :modrinth IS FALSE AND :hangar IS TRUE  THEN h.date_updated
        END
    END DESC,
    CASE
      WHEN :sort = 'downloads' THEN
        CASE WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN COALESCE(s.downloads, 0) + COALESCE(m.downloads, 0) + COALESCE(h.downloads, 0)
             WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS FALSE THEN COALESCE(s.downloads, 0) + COALESCE(m.downloads, 0)
             WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS TRUE  THEN COALESCE(s.downloads, 0) + COALESCE(h.downloads, 0)
             WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN COALESCE(m.downloads, 0) + COALESCE(h.downloads, 0)
             WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS FALSE THEN COALESCE(s.downloads, 0)
             WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS FALSE THEN COALESCE(m.downloads, 0)
             WHEN :spigot IS FALSE AND :modrinth IS FALSE AND :hangar IS TRUE  THEN COALESCE(h.downloads, 0)
        END
      WHEN :sort = 'likes_and_stars' THEN
        CASE WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN COALESCE(s.likes, 0) + COALESCE(h.stars, 0)
             WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS FALSE THEN COALESCE(s.likes, 0)
             WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS TRUE  THEN COALESCE(s.likes, 0) + COALESCE(h.stars, 0)
             WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN COALESCE(h.stars, 0)
             WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS FALSE THEN COALESCE(s.likes, 0)
             WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS FALSE THEN 0
             WHEN :spigot IS FALSE AND :modrinth IS FALSE AND :hangar IS TRUE  THEN COALESCE(h.stars, 0)
      END

      WHEN :sort = 'follows_and_watchers' THEN
        CASE WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN COALESCE(m.follows, 0) + COALESCE(h.watchers, 0)
             WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS FALSE THEN COALESCE(m.follows, 0)
             WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS TRUE  THEN COALESCE(h.watchers, 0)
             WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN COALESCE(m.follows, 0) + COALESCE(h.watchers, 0)
             WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS FALSE THEN 0
             WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS FALSE THEN COALESCE(m.follows, 0)
             WHEN :spigot IS FALSE AND :modrinth IS FALSE AND :hangar IS TRUE  THEN COALESCE(h.watchers, 0)
      END
    END DESC

LIMIT :limit;