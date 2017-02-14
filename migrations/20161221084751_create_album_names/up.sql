CREATE TABLE album_names (
  id serial PRIMARY KEY,
  album_id integer NOT NULL,
  name character varying NOT NULL,
  locale character varying NOT NULL,
  is_default boolean DEFAULT false NOT NULL,
  is_original boolean DEFAULT false NOT NULL,
  created_at timestamp without time zone NOT NULL,
  updated_at timestamp without time zone NOT NULL
);

CREATE INDEX ON album_names (album_id);

ALTER TABLE ONLY album_names
ADD FOREIGN KEY (album_id)
REFERENCES albums (id)
ON DELETE CASCADE;
