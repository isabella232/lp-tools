CREATE TABLE releases (
  id serial PRIMARY KEY,
  album_id integer NOT NULL,
  released_on date NOT NULL,
  country character varying,
  catalog_number character varying,
  disambiguation character varying,
  artwork_data text,
  created_at timestamp without time zone NOT NULL,
  updated_at timestamp without time zone NOT NULL
);

CREATE INDEX ON album_names (album_id);

ALTER TABLE ONLY album_names
ADD FOREIGN KEY (album_id)
REFERENCES albums (id)
ON DELETE CASCADE;
