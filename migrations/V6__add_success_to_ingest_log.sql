ALTER TABLE ingest_log ADD COLUMN success boolean;
UPDATE ingest_log SET success = TRUE;
ALTER TABLE ingest_log ALTER COLUMN success SET NOT NULL;