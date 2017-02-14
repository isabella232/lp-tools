CREATE TABLE artist_urls (
  id serial PRIMARY KEY,
  artist_id integer NOT NULL,
  url character varying NOT NULL,
  name character varying NOT NULL,
  created_at timestamp without time zone NOT NULL,
  updated_at timestamp without time zone NOT NULL
);

CREATE INDEX ON artist_urls (artist_id);

ALTER TABLE ONLY artist_urls
ADD FOREIGN KEY (artist_id)
REFERENCES artists (id)
ON DELETE CASCADE;
