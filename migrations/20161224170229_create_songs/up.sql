CREATE TABLE songs (
  id serial PRIMARY KEY,
  artist_credit_id integer NOT NULL,
  created_at timestamp without time zone NOT NULL,
  updated_at timestamp without time zone NOT NULL
);

CREATE INDEX ON songs (artist_credit_id);

ALTER TABLE ONLY songs
ADD FOREIGN KEY (artist_credit_id)
REFERENCES artist_credits (id)
ON DELETE CASCADE;
