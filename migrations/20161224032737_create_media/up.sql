CREATE TABLE media (
  id serial PRIMARY KEY,
  release_id integer NOT NULL,
  kind integer NOT NULL,
  position smallint DEFAULT 1 NOT NULL,
  name character varying,
  created_at timestamp without time zone NOT NULL,
  updated_at timestamp without time zone NOT NULL
);

CREATE INDEX ON media (release_id);

ALTER TABLE ONLY media
ADD FOREIGN KEY (release_id)
REFERENCES releases (id)
ON DELETE CASCADE;
