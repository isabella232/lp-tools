CREATE TABLE song_names (
  id serial PRIMARY KEY,
  song_id integer NOT NULL,
  name character varying NOT NULL,
  locale character varying NOT NULL,
  is_default boolean DEFAULT false NOT NULL,
  is_original boolean DEFAULT false NOT NULL,
  created_at timestamp without time zone NOT NULL,
  updated_at timestamp without time zone NOT NULL
);

CREATE INDEX ON song_names (song_id);

ALTER TABLE ONLY song_names
ADD FOREIGN KEY (song_id)
REFERENCES songs (id)
ON DELETE CASCADE;
