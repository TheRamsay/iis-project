-- DROP TYPE "group_join_status_type";

CREATE TYPE "group_join_status_type" AS ENUM (
	'pending',
	'accepted',
	'rejected');

-- DROP TYPE "post_type";

CREATE TYPE "post_type" AS ENUM (
	'image');

-- DROP TYPE "post_visibility_type";

CREATE TYPE "post_visibility_type" AS ENUM (
	'public',
	'private');

-- DROP TYPE "user_type";

CREATE TYPE "user_type" AS ENUM (
	'regular',
	'moderator',
	'administrator');

-- "location" definition

-- Drop table

-- DROP TABLE "location";

CREATE TABLE "location" (
	id uuid NOT NULL,
	"name" varchar NOT NULL,
	picture_url varchar NULL,
	latitude float8 NOT NULL,
	longitude float8 NOT NULL,
	CONSTRAINT location_pkey PRIMARY KEY (id)
);


-- seaql_migrations definition

-- Drop table

-- DROP TABLE seaql_migrations;

CREATE TABLE seaql_migrations (
	"version" varchar NOT NULL,
	applied_at int8 NOT NULL,
	CONSTRAINT seaql_migrations_pkey PRIMARY KEY (version)
);


-- wall definition

-- Drop table

-- DROP TABLE wall;

CREATE TABLE wall (
	id uuid NOT NULL,
	CONSTRAINT wall_pkey PRIMARY KEY (id)
);


-- "user" definition

-- Drop table

-- DROP TABLE "user";

CREATE TABLE "user" (
	id uuid NOT NULL,
	username varchar NOT NULL,
	email varchar NULL,
	avatar_url varchar NULL,
	"user_type" "user_type" NOT NULL,
	wall_id uuid NOT NULL,
	is_blocked bool NOT NULL,
	password_hash varchar NOT NULL,
	description varchar NULL,
	CONSTRAINT user_pkey PRIMARY KEY (id),
	CONSTRAINT fk_user_wall FOREIGN KEY (wall_id) REFERENCES wall(id)
);
CREATE UNIQUE INDEX unique_email ON "user" USING btree (email);
CREATE UNIQUE INDEX unique_username ON "user" USING btree (username);


-- "group" definition

-- Drop table

-- DROP TABLE "group";

CREATE TABLE "group" (
	id uuid NOT NULL,
	"name" varchar NOT NULL,
	admin_id uuid NOT NULL,
	wall_id uuid NOT NULL,
	CONSTRAINT group_pkey PRIMARY KEY (id),
	CONSTRAINT fk_group_admin FOREIGN KEY (admin_id) REFERENCES "user"(id) ON DELETE CASCADE ON UPDATE CASCADE,
	CONSTRAINT fk_group_wall FOREIGN KEY (wall_id) REFERENCES wall(id)
);


-- group_join_request definition

-- Drop table

-- DROP TABLE group_join_request;

CREATE TABLE group_join_request (
	id uuid NOT NULL,
	created_at timestamp NOT NULL,
	resolved_at timestamp NULL,
	group_id uuid NOT NULL,
	user_id uuid NOT NULL,
	status "group_join_status_type" NOT NULL,
	CONSTRAINT group_join_request_pkey PRIMARY KEY (id),
	CONSTRAINT group_join_request_group_id_fkey FOREIGN KEY (group_id) REFERENCES "group"(id) ON DELETE CASCADE ON UPDATE CASCADE,
	CONSTRAINT group_join_request_user_id_fkey FOREIGN KEY (user_id) REFERENCES "user"(id) ON DELETE CASCADE ON UPDATE CASCADE
);


-- group_member definition

-- Drop table

-- DROP TABLE group_member;

CREATE TABLE group_member (
	user_id uuid NOT NULL,
	group_id uuid NOT NULL,
	joined_at timestamp NOT NULL,
	CONSTRAINT group_member_pkey PRIMARY KEY (user_id, group_id),
	CONSTRAINT fk_group_member_group FOREIGN KEY (group_id) REFERENCES "group"(id) ON DELETE CASCADE ON UPDATE CASCADE,
	CONSTRAINT fk_group_member_user FOREIGN KEY (user_id) REFERENCES "user"(id) ON DELETE CASCADE ON UPDATE CASCADE
);


-- post definition

-- Drop table

-- DROP TABLE post;

CREATE TABLE post (
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
	CONSTRAINT fk_post_author FOREIGN KEY (author_id) REFERENCES "user"(id) ON DELETE CASCADE ON UPDATE CASCADE,
	CONSTRAINT fk_post_location FOREIGN KEY (location_id) REFERENCES "location"(id)
);


-- post_comment definition

-- Drop table

-- DROP TABLE post_comment;

CREATE TABLE post_comment (
	id uuid NOT NULL,
	post_id uuid NOT NULL,
	user_id uuid NOT NULL,
	"content" varchar NOT NULL,
	parent_id uuid NULL,
	CONSTRAINT post_comment_pkey PRIMARY KEY (id),
	CONSTRAINT fk_post_comment_parent FOREIGN KEY (parent_id) REFERENCES post_comment(id) ON DELETE CASCADE ON UPDATE CASCADE,
	CONSTRAINT fk_post_comment_post FOREIGN KEY (post_id) REFERENCES post(id) ON DELETE CASCADE ON UPDATE CASCADE,
	CONSTRAINT fk_post_comment_user FOREIGN KEY (user_id) REFERENCES "user"(id) ON DELETE CASCADE ON UPDATE CASCADE
);


-- post_group_visibility definition

-- Drop table

-- DROP TABLE post_group_visibility;

CREATE TABLE post_group_visibility (
	post_id uuid NOT NULL,
	group_id uuid NOT NULL,
	CONSTRAINT post_group_visibility_pkey PRIMARY KEY (post_id, group_id),
	CONSTRAINT fk_post_group_visibility_group FOREIGN KEY (group_id) REFERENCES "group"(id) ON DELETE CASCADE ON UPDATE CASCADE,
	CONSTRAINT fk_post_group_visibility_post FOREIGN KEY (post_id) REFERENCES post(id) ON DELETE CASCADE ON UPDATE CASCADE
);


-- post_like definition

-- Drop table

-- DROP TABLE post_like;

CREATE TABLE post_like (
	post_id uuid NOT NULL,
	user_id uuid NOT NULL,
	created_at timestamp NOT NULL,
	CONSTRAINT post_like_pkey PRIMARY KEY (post_id, user_id),
	CONSTRAINT fk_post_like_post FOREIGN KEY (post_id) REFERENCES post(id) ON DELETE CASCADE ON UPDATE CASCADE,
	CONSTRAINT post_like_user_id_fkey FOREIGN KEY (user_id) REFERENCES "user"(id) ON DELETE CASCADE ON UPDATE CASCADE
);


-- post_tag definition

-- Drop table

-- DROP TABLE post_tag;

CREATE TABLE post_tag (
	tag varchar NOT NULL,
	post_id uuid NOT NULL,
	CONSTRAINT post_tag_pkey PRIMARY KEY (tag, post_id),
	CONSTRAINT fk_post_tag_post FOREIGN KEY (post_id) REFERENCES post(id) ON DELETE CASCADE ON UPDATE CASCADE
);


-- post_user_visibility definition

-- Drop table

-- DROP TABLE post_user_visibility;

CREATE TABLE post_user_visibility (
	post_id uuid NOT NULL,
	user_id uuid NOT NULL,
	CONSTRAINT post_visibility_pkey PRIMARY KEY (post_id, user_id),
	CONSTRAINT fk_post_visibility_post FOREIGN KEY (post_id) REFERENCES post(id) ON DELETE CASCADE ON UPDATE CASCADE,
	CONSTRAINT fk_post_visibility_user FOREIGN KEY (user_id) REFERENCES "user"(id) ON DELETE CASCADE ON UPDATE CASCADE
);


-- wall_post definition

-- Drop table

-- DROP TABLE wall_post;

CREATE TABLE wall_post (
	wall_id uuid NOT NULL,
	post_id uuid NOT NULL,
	CONSTRAINT wall_post_pkey PRIMARY KEY (wall_id, post_id),
	CONSTRAINT fk_wall_post_post FOREIGN KEY (post_id) REFERENCES post(id) ON DELETE CASCADE ON UPDATE CASCADE,
	CONSTRAINT fk_wall_post_wall FOREIGN KEY (wall_id) REFERENCES wall(id) ON DELETE CASCADE ON UPDATE CASCADE
);


-- Inserts

INSERT INTO seaql_migrations
("version", applied_at)
VALUES('m20220101_000001_create_user_table', 1732567840);
INSERT INTO seaql_migrations
("version", applied_at)
VALUES('m20241009_204556_create_location_table', 1732567840);
INSERT INTO seaql_migrations
("version", applied_at)
VALUES('m20241009_204559_create_post_table', 1732567841);
INSERT INTO seaql_migrations
("version", applied_at)
VALUES('m20241010_141247_create_post_related_tables', 1732567841);
INSERT INTO seaql_migrations
("version", applied_at)
VALUES('m20241010_142036_create_wall_tables', 1732567841);
INSERT INTO seaql_migrations
("version", applied_at)
VALUES('m20241010_142037_create_group_tables', 1732567841);
INSERT INTO seaql_migrations
("version", applied_at)
VALUES('m20241010_142838_add_wall_to_user', 1732567841);
INSERT INTO seaql_migrations
("version", applied_at)
VALUES('m20241028_210624_user_is_bloced', 1732567841);
INSERT INTO seaql_migrations
("version", applied_at)
VALUES('m20241028_230949_location_lat_long', 1732567841);
INSERT INTO seaql_migrations
("version", applied_at)
VALUES('m20241102_185447_pwd_hash_for_user', 1732567841);
INSERT INTO seaql_migrations
("version", applied_at)
VALUES('m20241110_225527_group_join_request', 1732567841);
INSERT INTO seaql_migrations
("version", applied_at)
VALUES('m20241121_111845_user_make_fields_unique', 1732567841);
INSERT INTO seaql_migrations
("version", applied_at)
VALUES('m20241124_004127_visibility_tables', 1732567841);
INSERT INTO seaql_migrations
("version", applied_at)
VALUES('m20241125_001304_add_description_user', 1732567841);

INSERT INTO "wall" (id) VALUES ('9eabfafe-d210-4628-82c8-9e9aa5cf2952');
INSERT INTO "wall" (id) VALUES ('88ff5762-5372-4c2e-a761-99f8ae80f1ea');
INSERT INTO "wall" (id) VALUES ('68ce78e1-1f3a-402d-8729-427fe4661456');

INSERT INTO "user" (id, username, email, "user_type", wall_id, is_blocked, password_hash, avatar_url) VALUES ('6214fd84-1c6d-4f06-a233-efb1935fc7ad', 'dominik', 'dominik@test.cz', 'regular', '9eabfafe-d210-4628-82c8-9e9aa5cf2952', false, '$argon2id$v=19$m=19456,t=2,p=1$mWJ85rgmfeWcePKPJbvHCA$TE2U0+pqL+SomJIyZQSZqA2UA0yJE/kLcJe4WXFguI0', 'https://res.cloudinary.com/pelisek/image/upload/v1732559523/iis_project/9cf470878b05e62110c75c03fa829077.jpg');
INSERT INTO "user" (id, username, email, "user_type", wall_id, is_blocked, password_hash, avatar_url) VALUES ('59f437da-cef7-43b1-9d25-09590d80bd20', 'lukas', 'lukas@test.cz', 'moderator', '88ff5762-5372-4c2e-a761-99f8ae80f1ea', false, '$argon2id$v=19$m=19456,t=2,p=1$gVpxmHTz7xaYtYy/mMHprQ$H1zEnA+rCwuH4AZZCrmrDQhO0iHnKoNozSnjfvbyeNo', 'https://res.cloudinary.com/pelisek/image/upload/v1732565381/iis_project/e274242a723dce6b5474b4ee7833550f.jpg');
INSERT INTO "user" (id, username, email, "user_type", wall_id, is_blocked, password_hash, avatar_url) VALUES ('db003ce1-6ea3-4eda-ac06-193cb092b7ac', 'matyas', 'matyas@test.cz', 'administrator', '68ce78e1-1f3a-402d-8729-427fe4661456', false, '$argon2id$v=19$m=19456,t=2,p=1$RsvzIsDJczdZcbPAfw5lrw$m/5Pa12tqf8j1Un1iC9UVwHglO6irRmnOyOgGbDDCj8', 'https://res.cloudinary.com/pelisek/image/upload/v1732565381/iis_project/e274242a723dce6b5474b4ee7833550f.jpg');