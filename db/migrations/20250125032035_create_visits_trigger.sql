-- migrate:up
CREATE OR REPLACE FUNCTION sync_visit() RETURNS TRIGGER AS $sync_visit$
  BEGIN
    IF (TG_OP = 'DELETE') THEN
      DELETE FROM visits WHERE url_id = NEW.id;
    ELSIF (TG_OP = 'UPDATE') THEN
      UPDATE visits SET amount = 0 WHERE url_id = NEW.id;
    ELSIF (TG_OP = 'INSERT') THEN
      INSERT INTO visits (url_id, amount) VALUES (NEW.id, 0);
    END IF;
    RETURN NULL; -- this is an after trigger, so we may ignore the result
  END;
$sync_visit$ LANGUAGE plpgsql;

CREATE TRIGGER sync_visits_on_urls_change
AFTER INSERT OR UPDATE OR DELETE ON urls
FOR EACH ROW EXECUTE FUNCTION sync_visit();

-- migrate:down
DROP TRIGGER IF EXISTS sync_visits_on_urls_change ON urls;
DROP FUNCTION IF EXISTS sync_visit();
