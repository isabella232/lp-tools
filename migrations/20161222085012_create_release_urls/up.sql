CREATE TABLE release_urls (
  id serial PRIMARY KEY,
  release_id integer NOT NULL,
  url character varying NOT NULL,
  name character varying NOT NULL,
  created_at timestamp without time zone NOT NULL,
  updated_at timestamp without time zone NOT NULL
);

CREATE INDEX ON release_urls (release_id);

ALTER TABLE ONLY release_urls
ADD FOREIGN KEY (release_id)
REFERENCES releases (id)
ON DELETE CASCADE;
