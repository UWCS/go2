-- create table redirects_new table
-- new because new one is associated with this sqlx migration
-- which makes it easier to do more stuff in future
CREATE TABLE redirects_new (
    source text NOT NULL,
    sink text NOT NULL,
    usages integer DEFAULT 0 NOT NULL,
    last_used timestamp with time zone,
    created timestamp with time zone DEFAULT now(),
    constraint redirects_new_pkey PRIMARY KEY (source)
);