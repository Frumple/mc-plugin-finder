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