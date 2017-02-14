CREATE TABLE artists (
  id serial PRIMARY KEY,
  kind integer NOT NULL,
  country character varying NOT NULL,
  disambiguation character varying,
  started_on_year smallint,
  started_on_month smallint,
  started_on_day smallint,
  ended_on_year smallint,
  ended_on_month smallint,
  ended_on_day smallint,
  created_at timestamp without time zone NOT NULL,
  updated_at timestamp without time zone NOT NULL
);

CREATE INDEX ON artists (started_on_month);
