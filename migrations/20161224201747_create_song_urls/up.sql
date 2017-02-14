CREATE TABLE song_urls (
  id serial PRIMARY KEY,
  song_id integer NOT NULL,
  url character varying NOT NULL,
  name character varying NOT NULL,
  created_at timestamp without time zone NOT NULL,
  updated_at timestamp without time zone NOT NULL
);

CREATE INDEX ON song_urls (song_id);

ALTER TABLE ONLY song_urls
ADD FOREIGN KEY (song_id)
REFERENCES songs (id)
ON DELETE CASCADE;
