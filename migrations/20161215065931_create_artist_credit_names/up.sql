CREATE TABLE artist_credit_names (
  id serial PRIMARY KEY,
  artist_id integer NOT NULL,
  artist_credit_id integer NOT NULL,
  position smallint DEFAULT 0 NOT NULL,
  name character varying NOT NULL,
  locale character varying NOT NULL,
  is_default boolean DEFAULT false NOT NULL,
  is_original boolean DEFAULT false NOT NULL,
  separator character varying NOT NULL,
  created_at timestamp without time zone NOT NULL,
  updated_at timestamp without time zone NOT NULL
);

CREATE INDEX ON artist_credit_names (artist_id);

ALTER TABLE ONLY artist_credit_names
ADD FOREIGN KEY (artist_id)
REFERENCES artists (id)
ON DELETE CASCADE;

CREATE INDEX ON artist_credit_names (artist_credit_id);

ALTER TABLE ONLY artist_credit_names
ADD FOREIGN KEY (artist_credit_id)
REFERENCES artist_credits (id)
ON DELETE CASCADE;
