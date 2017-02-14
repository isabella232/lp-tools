CREATE TABLE artist_names (
  id serial PRIMARY KEY,
  artist_id integer NOT NULL,
  name character varying NOT NULL,
  locale character varying NOT NULL,
  is_default boolean DEFAULT false NOT NULL,
  is_original boolean DEFAULT false NOT NULL,
  created_at timestamp without time zone NOT NULL,
  updated_at timestamp without time zone NOT NULL
);

CREATE INDEX ON artist_names (artist_id);

ALTER TABLE ONLY artist_names
ADD FOREIGN KEY (artist_id)
REFERENCES artists (id)
ON DELETE CASCADE;
