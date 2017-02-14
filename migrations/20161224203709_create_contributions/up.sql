CREATE TABLE contributions (
  id serial PRIMARY KEY,
  artist_credit_id integer NOT NULL,
  song_id integer NOT NULL,
  kind integer NOT NULL,
  created_at timestamp without time zone NOT NULL,
  updated_at timestamp without time zone NOT NULL
);

CREATE INDEX ON contributions (artist_credit_id);

ALTER TABLE ONLY contributions
ADD FOREIGN KEY (artist_credit_id)
REFERENCES artist_credits (id)
ON DELETE CASCADE;

CREATE INDEX ON contributions (song_id);

ALTER TABLE ONLY contributions
ADD FOREIGN KEY (song_id)
REFERENCES songs (id)
ON DELETE CASCADE;
