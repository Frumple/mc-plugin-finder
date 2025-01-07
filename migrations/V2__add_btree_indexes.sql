-- B-tree indexes for ordering by date_created
CREATE INDEX IF NOT EXISTS common_project_spigot_modrinth_hangar_date_created_index
ON common_project (GREATEST(spigot_date_created, modrinth_date_created, hangar_date_created) DESC NULLS LAST);

CREATE INDEX IF NOT EXISTS common_project_spigot_modrinth_date_created_index
ON common_project (GREATEST(spigot_date_created, modrinth_date_created) DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_spigot_hangar_date_created_index
ON common_project (GREATEST(spigot_date_created, hangar_date_created) DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_modrinth_hangar_date_created_index
ON common_project (GREATEST(modrinth_date_created, hangar_date_created) DESC NULLS LAST);

CREATE INDEX IF NOT EXISTS common_project_spigot_date_created_index
ON common_project (spigot_date_created DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_modrinth_date_created_index
ON common_project (modrinth_date_created DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_hangar_date_created_index
ON common_project (hangar_date_created DESC NULLS LAST);

-- B-tree indexes for ordering by date_updated
CREATE INDEX IF NOT EXISTS common_project_spigot_modrinth_hangar_date_updated_index
ON common_project (GREATEST(spigot_date_updated, modrinth_date_updated, hangar_date_updated) DESC NULLS LAST);

CREATE INDEX IF NOT EXISTS common_project_spigot_modrinth_date_updated_index
ON common_project (GREATEST(spigot_date_updated, modrinth_date_updated) DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_spigot_hangar_date_updated_index
ON common_project (GREATEST(spigot_date_updated, hangar_date_updated) DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_modrinth_hangar_date_updated_index
ON common_project (GREATEST(modrinth_date_updated, hangar_date_updated) DESC NULLS LAST);

CREATE INDEX IF NOT EXISTS common_project_spigot_date_updated_index
ON common_project (spigot_date_updated DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_modrinth_date_updated_index
ON common_project (modrinth_date_updated DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_hangar_date_updated_index
ON common_project (hangar_date_updated DESC NULLS LAST);

-- B-tree indexes for ordering by latest_minecraft_version
CREATE INDEX IF NOT EXISTS common_project_spigot_modrinth_hangar_latest_minecraft_version_index
ON common_project (GREATEST(spigot_latest_minecraft_version, modrinth_latest_minecraft_version, hangar_latest_minecraft_version) DESC NULLS LAST);

CREATE INDEX IF NOT EXISTS common_project_spigot_modrinth_latest_minecraft_version_index
ON common_project (GREATEST(spigot_latest_minecraft_version, modrinth_latest_minecraft_version) DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_spigot_hangar_latest_minecraft_version_index
ON common_project (GREATEST(spigot_latest_minecraft_version, hangar_latest_minecraft_version) DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_modrinth_hangar_latest_minecraft_version_index
ON common_project (GREATEST(modrinth_latest_minecraft_version, hangar_latest_minecraft_version) DESC NULLS LAST);

CREATE INDEX IF NOT EXISTS common_project_spigot_latest_minecraft_version_index
ON common_project (spigot_latest_minecraft_version DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_modrinth_latest_minecraft_version_index
ON common_project (modrinth_latest_minecraft_version DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_hangar_latest_minecraft_version_index
ON common_project (hangar_latest_minecraft_version DESC NULLS LAST);

-- B-tree indexes for ordering by downloads
CREATE INDEX IF NOT EXISTS common_project_spigot_modrinth_hangar_downloads_index
ON common_project ((COALESCE(spigot_downloads, 0) + COALESCE(modrinth_downloads, 0) + COALESCE(hangar_downloads, 0)) DESC NULLS LAST);

CREATE INDEX IF NOT EXISTS common_project_spigot_modrinth_downloads_index
ON common_project ((COALESCE(spigot_downloads, 0) + COALESCE(modrinth_downloads, 0)) DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_spigot_hangar_downloads_index
ON common_project ((COALESCE(spigot_downloads, 0) + COALESCE(hangar_downloads, 0)) DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_modrinth_hangar_downloads_index
ON common_project ((COALESCE(modrinth_downloads, 0) + COALESCE(hangar_downloads, 0)) DESC NULLS LAST);

CREATE INDEX IF NOT EXISTS common_project_spigot_downloads_index
ON common_project (COALESCE(spigot_downloads, 0) DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_modrinth_downloads_index
ON common_project (COALESCE(modrinth_downloads, 0) DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_hangar_downloads_index
ON common_project (COALESCE(modrinth_downloads, 0) DESC NULLS LAST);

-- B-tree indexes for ordering by likes and stars
CREATE INDEX IF NOT EXISTS common_project_spigot_hangar_likes_and_stars_index
ON common_project ((COALESCE(spigot_likes, 0) + COALESCE(hangar_stars, 0)) DESC NULLS LAST);

CREATE INDEX IF NOT EXISTS common_project_spigot_likes_index
ON common_project (COALESCE(spigot_likes, 0) DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_hangar_stars_index
ON common_project (COALESCE(hangar_stars, 0) DESC NULLS LAST);

-- B-tree indexes for ordering by follows and watchers
CREATE INDEX IF NOT EXISTS common_project_modrinth_hangar_follows_and_watchers_index
ON common_project ((COALESCE(modrinth_follows, 0) + COALESCE(hangar_watchers, 0)) DESC NULLS LAST);

CREATE INDEX IF NOT EXISTS common_project_modrinth_follows_index
ON common_project (COALESCE(modrinth_follows, 0) DESC NULLS LAST);
CREATE INDEX IF NOT EXISTS common_project_hangar_watchers_index
ON common_project (COALESCE(hangar_watchers, 0) DESC NULLS LAST);