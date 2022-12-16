CREATE TABLE IF NOT EXISTS public."refresh_token" (
    id serial NOT NULL,
    token character varying(128) NOT NULL,
    user_id serial references users(id),
    validity_date timestamp with time zone NOT NULL,
    CONSTRAINT refresh_token_pkey PRIMARY KEY (id)
);