-- migrate:up
CREATE TABLE visits (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  url_id UUID NOT NULL REFERENCES urls(id),
  amount INTEGER DEFAULT 0
);

-- migrate:down
DROP TABLE visits;
