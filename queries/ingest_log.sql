--: IngestLogEntity()

--! insert_ingest_log ()
INSERT INTO ingest_log (action, repository, item, date_started, date_finished, items_processed, success)
  VALUES (:action, :repository, :item, :date_started, :date_finished, :items_processed, :success);

--! get_last_successful_ingest_log : IngestLogEntity
SELECT *
FROM ingest_log
WHERE success = TRUE
ORDER BY id DESC
LIMIT 1;

--! get_ingest_logs : IngestLogEntity
SELECT *
FROM ingest_log;