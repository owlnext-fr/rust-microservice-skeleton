-- Your SQL goes here
CREATE TABLE IF NOT EXISTS public."user" (
    id serial NOT NULL,
    email character varying(180) DEFAULT NULL,
    first_name character varying(200) DEFAULT NULL,
    last_name character varying(200) DEFAULT NULL,
    login character varying(180) NOT NULL,
    roles text [] NOT NULL,
    password character varying(32) NOT NULL,
    salt text DEFAULT null,
    created_date timestamp with time zone NOT NULL,
    created_by integer DEFAULT null,
    deleted_date timestamp with time zone DEFAULT null,
    deleted_by integer DEFAULT null,
    is_deleted boolean NOT NULL DEFAULT false,
    CONSTRAINT user_pkey PRIMARY KEY (id)
);