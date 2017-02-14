CREATE TABLE track_names (
  id serial PRIMARY KEY,
  track_id integer NOT NULL,
  name character varying NOT NULL,
  locale character varying NOT NULL,
  is_default boolean DEFAULT false NOT NULL,
  is_original boolean DEFAULT false NOT NULL,
  created_at timestamp without time zone NOT NULL,
  updated_at timestamp without time zone NOT NULL
);

CREATE INDEX ON track_names (track_id);

ALTER TABLE ONLY track_names
ADD FOREIGN KEY (track_id)
REFERENCES tracks (id)
ON DELETE CASCADE;
