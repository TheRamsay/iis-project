-- DROP TYPE public."group_join_status_type";

CREATE TYPE public."group_join_status_type" AS ENUM (
	'pending',
	'accepted',
	'rejected');

-- DROP TYPE public."post_type";

CREATE TYPE public."post_type" AS ENUM (
	'image');

-- DROP TYPE public."post_visibility_type";

CREATE TYPE public."post_visibility_type" AS ENUM (
	'public',
	'private');

-- DROP TYPE public."user_type";

CREATE TYPE public."user_type" AS ENUM (
	'regular',
	'moderator',
	'administrator');

-- public."location" definition

-- Drop table

-- DROP TABLE public."location";

CREATE TABLE public."location" (
	id uuid NOT NULL,
	"name" varchar NOT NULL,
	picture_url varchar NULL,
	latitude float8 NOT NULL,
	longitude float8 NOT NULL,
	CONSTRAINT location_pkey PRIMARY KEY (id)
);


-- public.seaql_migrations definition

-- Drop table

-- DROP TABLE public.seaql_migrations;

CREATE TABLE public.seaql_migrations (
	"version" varchar NOT NULL,
	applied_at int8 NOT NULL,
	CONSTRAINT seaql_migrations_pkey PRIMARY KEY (version)
);


-- public.wall definition

-- Drop table

-- DROP TABLE public.wall;

CREATE TABLE public.wall (
	id uuid NOT NULL,
	CONSTRAINT wall_pkey PRIMARY KEY (id)
);


-- public."user" definition

-- Drop table

-- DROP TABLE public."user";

CREATE TABLE public."user" (
	id uuid NOT NULL,
	username varchar NOT NULL,
	email varchar NULL,
	avatar_url varchar NULL,
	"user_type" public."user_type" NOT NULL,
	wall_id uuid NOT NULL,
	is_blocked bool NOT NULL,
	password_hash varchar NOT NULL,
	description varchar NULL,
	CONSTRAINT user_pkey PRIMARY KEY (id),
	CONSTRAINT fk_user_wall FOREIGN KEY (wall_id) REFERENCES public.wall(id)
);
CREATE UNIQUE INDEX unique_email ON public."user" USING btree (email);
CREATE UNIQUE INDEX unique_username ON public."user" USING btree (username);


-- public."group" definition

-- Drop table

-- DROP TABLE public."group";

CREATE TABLE public."group" (
	id uuid NOT NULL,
	"name" varchar NOT NULL,
	admin_id uuid NOT NULL,
	wall_id uuid NOT NULL,
	CONSTRAINT group_pkey PRIMARY KEY (id),
	CONSTRAINT fk_group_admin FOREIGN KEY (admin_id) REFERENCES public."user"(id) ON DELETE CASCADE ON UPDATE CASCADE,
	CONSTRAINT fk_group_wall FOREIGN KEY (wall_id) REFERENCES public.wall(id)
);


-- public.group_join_request definition

-- Drop table

-- DROP TABLE public.group_join_request;

CREATE TABLE public.group_join_request (
	id uuid NOT NULL,
	created_at timestamp NOT NULL,
	resolved_at timestamp NULL,
	group_id uuid NOT NULL,
	user_id uuid NOT NULL,
	status public."group_join_status_type" NOT NULL,
	CONSTRAINT group_join_request_pkey PRIMARY KEY (id),
	CONSTRAINT group_join_request_group_id_fkey FOREIGN KEY (group_id) REFERENCES public."group"(id) ON DELETE CASCADE ON UPDATE CASCADE,
	CONSTRAINT group_join_request_user_id_fkey FOREIGN KEY (user_id) REFERENCES public."user"(id) ON DELETE CASCADE ON UPDATE CASCADE
);


-- public.group_member definition

-- Drop table

-- DROP TABLE public.group_member;

CREATE TABLE public.group_member (
	user_id uuid NOT NULL,
	group_id uuid NOT NULL,
	joined_at timestamp NOT NULL,
	CONSTRAINT group_member_pkey PRIMARY KEY (user_id, group_id),
	CONSTRAINT fk_group_member_group FOREIGN KEY (group_id) REFERENCES public."group"(id) ON DELETE CASCADE ON UPDATE CASCADE,
	CONSTRAINT fk_group_member_user FOREIGN KEY (user_id) REFERENCES public."user"(id) ON DELETE CASCADE ON UPDATE CASCADE
);


-- public.post definition

-- Drop table

-- DROP TABLE public.post;

CREATE TABLE public.post (
	id uuid NOT NULL,
	title varchar NOT NULL,
	description varchar NOT NULL,
	location_id uuid NULL,
	author_id uuid NOT NULL,
	created_at timestamp NOT NULL,
	content_type varchar NOT NULL,
	visibility varchar NOT NULL,
	content_url varchar NOT NULL,
	CONSTRAINT post_pkey PRIMARY KEY (id),
	CONSTRAINT fk_post_author FOREIGN KEY (author_id) REFERENCES public."user"(id) ON DELETE CASCADE ON UPDATE CASCADE,
	CONSTRAINT fk_post_location FOREIGN KEY (location_id) REFERENCES public."location"(id)
);


-- public.post_comment definition

-- Drop table

-- DROP TABLE public.post_comment;

CREATE TABLE public.post_comment (
	id uuid NOT NULL,
	post_id uuid NOT NULL,
	user_id uuid NOT NULL,
	"content" varchar NOT NULL,
	parent_id uuid NULL,
	CONSTRAINT post_comment_pkey PRIMARY KEY (id),
	CONSTRAINT fk_post_comment_parent FOREIGN KEY (parent_id) REFERENCES public.post_comment(id) ON DELETE CASCADE ON UPDATE CASCADE,
	CONSTRAINT fk_post_comment_post FOREIGN KEY (post_id) REFERENCES public.post(id) ON DELETE CASCADE ON UPDATE CASCADE,
	CONSTRAINT fk_post_comment_user FOREIGN KEY (user_id) REFERENCES public."user"(id) ON DELETE CASCADE ON UPDATE CASCADE
);


-- public.post_group_visibility definition

-- Drop table

-- DROP TABLE public.post_group_visibility;

CREATE TABLE public.post_group_visibility (
	post_id uuid NOT NULL,
	group_id uuid NOT NULL,
	CONSTRAINT post_group_visibility_pkey PRIMARY KEY (post_id, group_id),
	CONSTRAINT fk_post_group_visibility_group FOREIGN KEY (group_id) REFERENCES public."group"(id) ON DELETE CASCADE ON UPDATE CASCADE,
	CONSTRAINT fk_post_group_visibility_post FOREIGN KEY (post_id) REFERENCES public.post(id) ON DELETE CASCADE ON UPDATE CASCADE
);


-- public.post_like definition

-- Drop table

-- DROP TABLE public.post_like;

CREATE TABLE public.post_like (
	post_id uuid NOT NULL,
	user_id uuid NOT NULL,
	created_at timestamp NOT NULL,
	CONSTRAINT post_like_pkey PRIMARY KEY (post_id, user_id),
	CONSTRAINT fk_post_like_post FOREIGN KEY (post_id) REFERENCES public.post(id) ON DELETE CASCADE ON UPDATE CASCADE,
	CONSTRAINT post_like_user_id_fkey FOREIGN KEY (user_id) REFERENCES public."user"(id) ON DELETE CASCADE ON UPDATE CASCADE
);


-- public.post_tag definition

-- Drop table

-- DROP TABLE public.post_tag;

CREATE TABLE public.post_tag (
	tag varchar NOT NULL,
	post_id uuid NOT NULL,
	CONSTRAINT post_tag_pkey PRIMARY KEY (tag, post_id),
	CONSTRAINT fk_post_tag_post FOREIGN KEY (post_id) REFERENCES public.post(id) ON DELETE CASCADE ON UPDATE CASCADE
);


-- public.post_user_visibility definition

-- Drop table

-- DROP TABLE public.post_user_visibility;

CREATE TABLE public.post_user_visibility (
	post_id uuid NOT NULL,
	user_id uuid NOT NULL,
	CONSTRAINT post_visibility_pkey PRIMARY KEY (post_id, user_id),
	CONSTRAINT fk_post_visibility_post FOREIGN KEY (post_id) REFERENCES public.post(id) ON DELETE CASCADE ON UPDATE CASCADE,
	CONSTRAINT fk_post_visibility_user FOREIGN KEY (user_id) REFERENCES public."user"(id) ON DELETE CASCADE ON UPDATE CASCADE
);


-- public.wall_post definition

-- Drop table

-- DROP TABLE public.wall_post;

CREATE TABLE public.wall_post (
	wall_id uuid NOT NULL,
	post_id uuid NOT NULL,
	CONSTRAINT wall_post_pkey PRIMARY KEY (wall_id, post_id),
	CONSTRAINT fk_wall_post_post FOREIGN KEY (post_id) REFERENCES public.post(id) ON DELETE CASCADE ON UPDATE CASCADE,
	CONSTRAINT fk_wall_post_wall FOREIGN KEY (wall_id) REFERENCES public.wall(id) ON DELETE CASCADE ON UPDATE CASCADE
);

-- Inserts

INSERT INTO public."wall" (id) VALUES ('9eabfafe-d210-4628-82c8-9e9aa5cf2952');
INSERT INTO public."wall" (id) VALUES ('88ff5762-5372-4c2e-a761-99f8ae80f1ea');
INSERT INTO public."wall" (id) VALUES ('68ce78e1-1f3a-402d-8729-427fe4661456');

INSERT INTO public."user" (id, username, email, "user_type", wall_id, is_blocked, password_hash) VALUES ('6214fd84-1c6d-4f06-a233-efb1935fc7ad', 'dominik', 'dominik@test.cz', 'regular', '9eabfafe-d210-4628-82c8-9e9aa5cf2952', false, '$argon2id$v=19$m=19456,t=2,p=1$mWJ85rgmfeWcePKPJbvHCA$TE2U0+pqL+SomJIyZQSZqA2UA0yJE/kLcJe4WXFguI0');
INSERT INTO public."user" (id, username, email, "user_type", wall_id, is_blocked, password_hash) VALUES ('59f437da-cef7-43b1-9d25-09590d80bd20', 'lukas', 'lukas@test.cz', 'moderator', '88ff5762-5372-4c2e-a761-99f8ae80f1ea', false, '$argon2id$v=19$m=19456,t=2,p=1$gVpxmHTz7xaYtYy/mMHprQ$H1zEnA+rCwuH4AZZCrmrDQhO0iHnKoNozSnjfvbyeNo');
INSERT INTO public."user" (id, username, email, "user_type", wall_id, is_blocked, password_hash) VALUES ('db003ce1-6ea3-4eda-ac06-193cb092b7ac', 'matyas', 'matyas@test.cz', 'admin', '68ce78e1-1f3a-402d-8729-427fe4661456', false, '$argon2id$v=19$m=19456,t=2,p=1$RsvzIsDJczdZcbPAfw5lrw$m/5Pa12tqf8j1Un1iC9UVwHglO6irRmnOyOgGbDDCj8');