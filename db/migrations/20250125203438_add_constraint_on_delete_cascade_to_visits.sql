-- migrate:up
ALTER TABLE visits
DROP CONSTRAINT IF EXISTS visits_url_id_fkey;

ALTER TABLE visits
ADD CONSTRAINT visits_url_id_fkey
FOREIGN KEY (url_id)
REFERENCES urls(id)
ON DELETE CASCADE;

-- migrate:down
ALTER TABLE visits
DROP CONSTRAINT IF EXISTS visits_url_id_fkey;

ALTER TABLE visits
ADD CONSTRAINT visits_url_id_fkey
FOREIGN KEY (url_id)
REFERENCES urls(id)
ON DELETE RESTRICT;
