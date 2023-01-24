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