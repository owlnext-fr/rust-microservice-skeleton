CREATE TABLE public.cron_logs (
    id serial NOT NULL,
    command character varying(512) NOT NULL,
    command_args character varying(512) NOT NULL,
    exit_status integer DEFAULT null,
    exit_message text DEFAULT null,
    started_at timestamp with time zone NOT NULL,
    ended_at timestamp with time zone DEFAULT null,
    PRIMARY KEY (id)
);
CREATE TABLE public.account (
    id serial NOT NULL,
    name character varying(255) NOT NULL,
    created_date timestamp with time zone NOT NULL,
    deleted_date timestamp with time zone DEFAULT null,
    is_deleted boolean NOT NULL DEFAULT false,
    CONSTRAINT account_pkey PRIMARY KEY (id)
);
CREATE TABLE public.application (
    id serial NOT NULL,
    ulid character varying(40) NOT NULL,
    name character varying(255) NOT NULL,
    contact_email character varying(255) NOT NULL,
    account_id serial references account(id) NOT NULL,
    created_date timestamp with time zone NOT NULL,
    deleted_date timestamp with time zone DEFAULT null,
    is_deleted boolean NOT NULL DEFAULT false,
    CONSTRAINT application_pkey PRIMARY KEY (id)
);
CREATE TABLE IF NOT EXISTS public.users (
    id serial NOT NULL,
    email character varying(180) DEFAULT NULL,
    first_name character varying(200) DEFAULT NULL,
    last_name character varying(200) DEFAULT NULL,
    login character varying(180) NOT NULL,
    roles text [] NOT NULL,
    password text NOT NULL,
    salt text DEFAULT null,
    application_id serial references application(id) NOT NULL,
    created_date timestamp with time zone NOT NULL,
    created_by serial references users(id),
    deleted_date timestamp with time zone DEFAULT NULL,
    deleted_by serial references users(id),
    is_deleted boolean NOT NULL DEFAULT false,
    CONSTRAINT user_pkey PRIMARY KEY (id)
);
CREATE TABLE IF NOT EXISTS public."refresh_token" (
    id serial NOT NULL,
    token character varying(128) NOT NULL,
    user_id serial references users(id),
    validity_date timestamp with time zone NOT NULL,
    CONSTRAINT refresh_token_pkey PRIMARY KEY (id)
);