--: SearchResultEntity(latest_minecraft_version?, spigot_id?, spigot_slug?, spigot_name?, spigot_description?, spigot_author?, spigot_version?, spigot_premium?, spigot_abandoned?, spigot_icon_data?, modrinth_id?, modrinth_slug?, modrinth_name?, modrinth_description?, modrinth_author?, modrinth_version?, modrinth_status?, modrinth_icon_url?, hangar_slug?, hangar_name?, hangar_description?, hangar_author?, hangar_version?, hangar_icon_url?, source_repository_host?, source_repository_owner?, source_repository_name?)

--! search_projects (query, spigot, modrinth, hangar, name, description, author, sort, limit, offset) : SearchResultEntity
SELECT
  COUNT(*) OVER() AS full_count,
  CASE WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN GREATEST(spigot_date_created, modrinth_date_created, hangar_date_created)
       WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS FALSE THEN GREATEST(spigot_date_created, modrinth_date_created)
       WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS TRUE  THEN GREATEST(spigot_date_created, hangar_date_created)
       WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN GREATEST(modrinth_date_created, hangar_date_created)
       WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS FALSE THEN spigot_date_created
       WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS FALSE THEN modrinth_date_created
       WHEN :spigot IS FALSE AND :modrinth IS FALSE AND :hangar IS TRUE  THEN hangar_date_created
       ELSE timestamptz '-infinity'
  END
  AS date_created,

  CASE WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN GREATEST(spigot_date_updated, modrinth_date_updated, hangar_date_updated)
       WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS FALSE THEN GREATEST(spigot_date_updated, modrinth_date_updated)
       WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS TRUE  THEN GREATEST(spigot_date_updated, hangar_date_updated)
       WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN GREATEST(modrinth_date_updated, hangar_date_updated)
       WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS FALSE THEN spigot_date_updated
       WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS FALSE THEN modrinth_date_updated
       WHEN :spigot IS FALSE AND :modrinth IS FALSE AND :hangar IS TRUE  THEN hangar_date_updated
       ELSE timestamptz '-infinity'
  END
  AS date_updated,

  CASE WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN GREATEST(spigot_latest_minecraft_version, modrinth_latest_minecraft_version, hangar_latest_minecraft_version)
       WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS FALSE THEN GREATEST(spigot_latest_minecraft_version, modrinth_latest_minecraft_version)
       WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS TRUE  THEN GREATEST(spigot_latest_minecraft_version, hangar_latest_minecraft_version)
       WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN GREATEST(modrinth_latest_minecraft_version, hangar_latest_minecraft_version)
       WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS FALSE THEN spigot_latest_minecraft_version
       WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS FALSE THEN modrinth_latest_minecraft_version
       WHEN :spigot IS FALSE AND :modrinth IS FALSE AND :hangar IS TRUE  THEN hangar_latest_minecraft_version
       ELSE NULL
  END
  AS latest_minecraft_version,

  CASE WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN COALESCE(spigot_downloads, 0) + COALESCE(modrinth_downloads, 0) + COALESCE(hangar_downloads, 0)
       WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS FALSE THEN COALESCE(spigot_downloads, 0) + COALESCE(modrinth_downloads, 0)
       WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS TRUE  THEN COALESCE(spigot_downloads, 0) + COALESCE(hangar_downloads, 0)
       WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN COALESCE(modrinth_downloads, 0) + COALESCE(hangar_downloads, 0)
       WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS FALSE THEN COALESCE(spigot_downloads, 0)
       WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS FALSE THEN COALESCE(modrinth_downloads, 0)
       WHEN :spigot IS FALSE AND :modrinth IS FALSE AND :hangar IS TRUE  THEN COALESCE(hangar_downloads, 0)
       ELSE 0
  END
  AS downloads,

  CASE WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN COALESCE(spigot_likes, 0) + COALESCE(hangar_stars, 0)
       WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS FALSE THEN COALESCE(spigot_likes, 0)
       WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS TRUE  THEN COALESCE(spigot_likes, 0) + COALESCE(hangar_stars, 0)
       WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN COALESCE(hangar_stars, 0)
       WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS FALSE THEN COALESCE(spigot_likes, 0)
       WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS FALSE THEN 0
       WHEN :spigot IS FALSE AND :modrinth IS FALSE AND :hangar IS TRUE  THEN COALESCE(hangar_stars, 0)
       ELSE 0
  END
  AS likes_and_stars,

  CASE WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN COALESCE(modrinth_follows, 0) + COALESCE(hangar_watchers, 0)
       WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS FALSE THEN COALESCE(modrinth_follows, 0)
       WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS TRUE  THEN COALESCE(hangar_watchers, 0)
       WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN COALESCE(modrinth_follows, 0) + COALESCE(hangar_watchers, 0)
       WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS FALSE THEN 0
       WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS FALSE THEN COALESCE(modrinth_follows, 0)
       WHEN :spigot IS FALSE AND :modrinth IS FALSE AND :hangar IS TRUE  THEN COALESCE(hangar_watchers, 0)
       ELSE 0
  END
  AS follows_and_watchers,

  (CASE WHEN :spigot IS TRUE THEN spigot_id ELSE NULL END) AS spigot_id,
  (CASE WHEN :spigot IS TRUE THEN spigot_slug ELSE NULL END) AS spigot_slug,
  (CASE WHEN :spigot IS TRUE THEN spigot_name ELSE NULL END) AS spigot_name,
  (CASE WHEN :spigot IS TRUE THEN spigot_description ELSE NULL END) AS spigot_description,
  (CASE WHEN :spigot IS TRUE THEN spigot_author ELSE NULL END) AS spigot_author,
  (CASE WHEN :spigot IS TRUE THEN spigot_version ELSE NULL END) AS spigot_version,
  (CASE WHEN :spigot IS TRUE THEN spigot_premium ELSE NULL END) AS spigot_premium,
  (CASE WHEN :spigot IS TRUE THEN spigot_abandoned ELSE NULL END) AS spigot_abandoned,
  (CASE WHEN :spigot IS TRUE THEN spigot_icon_data ELSE NULL END) AS spigot_icon_data,

  (CASE WHEN :modrinth IS TRUE THEN modrinth_id ELSE NULL END) AS modrinth_id,
  (CASE WHEN :modrinth IS TRUE THEN modrinth_slug ELSE NULL END) AS modrinth_slug,
  (CASE WHEN :modrinth IS TRUE THEN modrinth_name ELSE NULL END) AS modrinth_name,
  (CASE WHEN :modrinth IS TRUE THEN modrinth_description ELSE NULL END) AS modrinth_description,
  (CASE WHEN :modrinth IS TRUE THEN modrinth_author ELSE NULL END) AS modrinth_author,
  (CASE WHEN :modrinth IS TRUE THEN modrinth_version ELSE NULL END) AS modrinth_version,
  (CASE WHEN :modrinth IS TRUE THEN modrinth_status ELSE NULL END) AS modrinth_status,
  (CASE WHEN :modrinth IS TRUE THEN modrinth_icon_url ELSE NULL END) AS modrinth_icon_url,

  (CASE WHEN :hangar IS TRUE THEN hangar_slug ELSE NULL END) AS hangar_slug,
  (CASE WHEN :hangar IS TRUE THEN hangar_name ELSE NULL END) AS hangar_name,
  (CASE WHEN :hangar IS TRUE THEN hangar_description ELSE NULL END) AS hangar_description,
  (CASE WHEN :hangar IS TRUE THEN hangar_author ELSE NULL END) AS hangar_author,
  (CASE WHEN :hangar IS TRUE THEN hangar_version ELSE NULL END) AS hangar_version,
  (CASE WHEN :hangar IS TRUE THEN hangar_icon_url ELSE NULL END) AS hangar_icon_url,

  source_repository_host,
  source_repository_owner,
  source_repository_name
FROM
  common_project
WHERE
  :query = ''

  OR

  CASE :spigot IS TRUE AND :name IS TRUE
    WHEN TRUE THEN :query <% spigot_name
    ELSE FALSE
  END

  OR

  CASE :spigot IS TRUE AND :description IS TRUE
    WHEN TRUE THEN :query <% spigot_description
    ELSE FALSE
  END

  OR

  CASE :spigot IS TRUE AND :author IS TRUE
    WHEN TRUE THEN :query <% spigot_author
    ELSE FALSE
  END

  OR

  CASE :modrinth IS TRUE AND :name IS TRUE
    WHEN TRUE THEN :query <% modrinth_name
    ELSE FALSE
  END

  OR

  CASE :modrinth IS TRUE AND :description IS TRUE
    WHEN TRUE THEN :query <% modrinth_description
    ELSE FALSE
  END

  OR

  CASE :modrinth IS TRUE AND :author IS TRUE
    WHEN TRUE THEN :query <% modrinth_author
    ELSE FALSE
  END

  OR

  CASE :hangar IS TRUE AND :name IS TRUE
    WHEN TRUE THEN :query <% hangar_name
    ELSE FALSE
  END

  OR

  CASE :hangar IS TRUE AND :description IS TRUE
    WHEN TRUE THEN :query <% hangar_description
    ELSE FALSE
  END

  OR

  CASE :hangar IS TRUE AND :author IS TRUE
    WHEN TRUE THEN :query <% hangar_author
    ELSE FALSE
  END

  ORDER BY
    CASE
      WHEN :sort = 'date_created' THEN
        CASE WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN GREATEST(spigot_date_created, modrinth_date_created, hangar_date_created)
             WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS FALSE THEN GREATEST(spigot_date_created, modrinth_date_created)
             WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS TRUE  THEN GREATEST(spigot_date_created, hangar_date_created)
             WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN GREATEST(modrinth_date_created, hangar_date_created)
             WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS FALSE THEN spigot_date_created
             WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS FALSE THEN modrinth_date_created
             WHEN :spigot IS FALSE AND :modrinth IS FALSE AND :hangar IS TRUE  THEN hangar_date_created
        END

      WHEN :sort = 'date_updated' THEN
        CASE WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN GREATEST(spigot_date_updated, modrinth_date_updated, hangar_date_updated)
             WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS FALSE THEN GREATEST(spigot_date_updated, modrinth_date_updated)
             WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS TRUE  THEN GREATEST(spigot_date_updated, hangar_date_updated)
             WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN GREATEST(modrinth_date_updated, hangar_date_updated)
             WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS FALSE THEN spigot_date_updated
             WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS FALSE THEN modrinth_date_updated
             WHEN :spigot IS FALSE AND :modrinth IS FALSE AND :hangar IS TRUE  THEN hangar_date_updated
        END
    END DESC,

    CASE
      WHEN :sort = 'latest_minecraft_version' THEN
        CASE WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN GREATEST(spigot_latest_minecraft_version, modrinth_latest_minecraft_version, hangar_latest_minecraft_version)
             WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS FALSE THEN GREATEST(spigot_latest_minecraft_version, modrinth_latest_minecraft_version)
             WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS TRUE  THEN GREATEST(spigot_latest_minecraft_version, hangar_latest_minecraft_version)
             WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN GREATEST(modrinth_latest_minecraft_version, hangar_latest_minecraft_version)
             WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS FALSE THEN spigot_latest_minecraft_version
             WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS FALSE THEN modrinth_latest_minecraft_version
             WHEN :spigot IS FALSE AND :modrinth IS FALSE AND :hangar IS TRUE  THEN hangar_latest_minecraft_version
        END
    END DESC NULLS LAST,

    CASE
      WHEN :sort = 'downloads' THEN
        CASE WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN COALESCE(spigot_downloads, 0) + COALESCE(modrinth_downloads, 0) + COALESCE(hangar_downloads, 0)
             WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS FALSE THEN COALESCE(spigot_downloads, 0) + COALESCE(modrinth_downloads, 0)
             WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS TRUE  THEN COALESCE(spigot_downloads, 0) + COALESCE(hangar_downloads, 0)
             WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN COALESCE(modrinth_downloads, 0) + COALESCE(hangar_downloads, 0)
             WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS FALSE THEN COALESCE(spigot_downloads, 0)
             WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS FALSE THEN COALESCE(modrinth_downloads, 0)
             WHEN :spigot IS FALSE AND :modrinth IS FALSE AND :hangar IS TRUE  THEN COALESCE(hangar_downloads, 0)
        END

      WHEN :sort = 'likes_and_stars' THEN
        CASE WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN COALESCE(spigot_likes, 0) + COALESCE(hangar_stars, 0)
             WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS FALSE THEN COALESCE(spigot_likes, 0)
             WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS TRUE  THEN COALESCE(spigot_likes, 0) + COALESCE(hangar_stars, 0)
             WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN COALESCE(hangar_stars, 0)
             WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS FALSE THEN COALESCE(spigot_likes, 0)
             WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS FALSE THEN 0
             WHEN :spigot IS FALSE AND :modrinth IS FALSE AND :hangar IS TRUE  THEN COALESCE(hangar_stars, 0)
        END

      WHEN :sort = 'follows_and_watchers' THEN
        CASE WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN COALESCE(modrinth_follows, 0) + COALESCE(hangar_watchers, 0)
             WHEN :spigot IS TRUE  AND :modrinth IS TRUE  AND :hangar IS FALSE THEN COALESCE(modrinth_follows, 0)
             WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS TRUE  THEN COALESCE(hangar_watchers, 0)
             WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS TRUE  THEN COALESCE(modrinth_follows, 0) + COALESCE(hangar_watchers, 0)
             WHEN :spigot IS TRUE  AND :modrinth IS FALSE AND :hangar IS FALSE THEN 0
             WHEN :spigot IS FALSE AND :modrinth IS TRUE  AND :hangar IS FALSE THEN COALESCE(modrinth_follows, 0)
             WHEN :spigot IS FALSE AND :modrinth IS FALSE AND :hangar IS TRUE  THEN COALESCE(hangar_watchers, 0)
        END
    END DESC

LIMIT :limit
OFFSET :offset;