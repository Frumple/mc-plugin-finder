--: IngestLogEntity()

--! insert_ingest_log ()
INSERT INTO ingest_log (action, repository, item, date_started, date_finished, items_processed)
  VALUES (:action, :repository, :item, :date_started, :date_finished, :items_processed);

--! get_last_ingest_log : IngestLogEntity
SELECT *
FROM ingest_log
ORDER BY id DESC
LIMIT 1;

--! get_ingest_logs : IngestLogEntity
SELECT *
FROM ingest_log;