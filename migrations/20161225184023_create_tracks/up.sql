CREATE TABLE tracks (
  id serial PRIMARY KEY,
  medium_id integer NOT NULL,
  artist_credit_id integer NOT NULL,
  song_id integer NOT NULL,
  position smallint NOT NULL,
  duration integer,
  created_at timestamp without time zone NOT NULL,
  updated_at timestamp without time zone NOT NULL
);

ALTER TABLE ONLY tracks
ADD FOREIGN KEY (medium_id)
REFERENCES media (id)
ON DELETE CASCADE;

ALTER TABLE ONLY tracks
ADD FOREIGN KEY (artist_credit_id)
REFERENCES artist_credits (id)
ON DELETE CASCADE;

ALTER TABLE ONLY tracks
ADD FOREIGN KEY (song_id)
REFERENCES songs (id)
ON DELETE CASCADE;
