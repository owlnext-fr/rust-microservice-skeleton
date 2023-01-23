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