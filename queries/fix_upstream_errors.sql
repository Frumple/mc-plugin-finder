-- Removes source repository host, owner, and name from upstream Spigot resources that clearly have an incorrect source_url that belongs to another legitimate resource.
-- This prevents these resources from merging with the other legitimate resources in the common_projects view.

-- Spigot ID | Name                                         | Incorrect Source URL
-- 25773     | TigerReports                                 | https://github.com/PikaMug/Quests
-- 82123     | tuto Skript numéros 1 (Méssage de bienvenue) | https://github.com/pop4959/Chunky
-- 97659     | InvGames                                     | https://github.com/PlaceholderAPI/PlaceholderAPI
-- 119724    | FREE Grim anticheat config                   | https://github.com/GrimAnticheat/Grim

--! remove_incorrect_source_repository_host_owner_and_name_from_spigot_resources
UPDATE spigot_resource
SET source_repository_host = NULL, source_repository_owner = NULL, source_repository_name = NULL
WHERE id IN (25773, 82123, 97659, 119724);

-- Adds an internal source_repository_id to upstream resources/projects that share the same source_url.
-- This ensures that these projects aren't merged together to create duplicate projects in the common_projects view.

-- Modrinth ID | Modrinth Slug            | Name
-- Vem8mYeH	   | essentialsx-discord      | EssentialsX Discord
-- lyP3EhLg	   | essentialsx-protect      | EssentialsX Protect
-- IWjhyNzg	   | essentialsx-xmpp         | EssentialsX XMPP
-- KPfTOjGm	   | essentialsx-antibuild    | EssentialsX AntiBuild
-- 2qgyQbO1	   | essentialsx-chat-module  | EssentialsX Chat
-- sYpvDxGJ	   | essentialsx-spawn        | EssentialsX Spawn
-- cj1AijZw	   | essentialsx-discord-link | EssentialsX Discord Link
-- 3yb40IgO	   | essentialsx-geo          | EssentialsX Geo

--! add_source_repository_id_to_modrinth_projects
UPDATE modrinth_project
SET source_repository_id = id
WHERE id IN ('Vem8mYeH', 'lyP3EhLg', 'IWjhyNzg', 'KPfTOjGm', '2qgyQbO1', 'sYpvDxGJ', 'cj1AijZw', '3yb40IgO');