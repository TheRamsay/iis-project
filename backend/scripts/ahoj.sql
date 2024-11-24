--
-- PostgreSQL database dump
--

-- Dumped from database version 14.13
-- Dumped by pg_dump version 14.13 (Ubuntu 14.13-0ubuntu0.22.04.1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: group_join_status_type; Type: TYPE; Schema: public; Owner: koteseni
--

CREATE TYPE public.group_join_status_type AS ENUM (
    'pending',
    'accepted',
    'rejected'
);


ALTER TYPE public.group_join_status_type OWNER TO koteseni;

--
-- Name: post_type; Type: TYPE; Schema: public; Owner: koteseni
--

CREATE TYPE public.post_type AS ENUM (
    'image'
);


ALTER TYPE public.post_type OWNER TO koteseni;

--
-- Name: post_visibility_type; Type: TYPE; Schema: public; Owner: koteseni
--

CREATE TYPE public.post_visibility_type AS ENUM (
    'public',
    'private'
);


ALTER TYPE public.post_visibility_type OWNER TO koteseni;

--
-- Name: user_type; Type: TYPE; Schema: public; Owner: koteseni
--

CREATE TYPE public.user_type AS ENUM (
    'regular',
    'moderator',
    'administrator'
);


ALTER TYPE public.user_type OWNER TO koteseni;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: group; Type: TABLE; Schema: public; Owner: koteseni
--

CREATE TABLE public."group" (
    id uuid NOT NULL,
    name character varying NOT NULL,
    admin_id uuid NOT NULL,
    wall_id uuid NOT NULL
);


ALTER TABLE public."group" OWNER TO koteseni;

--
-- Name: group_join_request; Type: TABLE; Schema: public; Owner: koteseni
--

CREATE TABLE public.group_join_request (
    id uuid NOT NULL,
    created_at timestamp without time zone NOT NULL,
    resolved_at timestamp without time zone,
    group_id uuid NOT NULL,
    user_id uuid NOT NULL,
    status public.group_join_status_type NOT NULL
);


ALTER TABLE public.group_join_request OWNER TO koteseni;

--
-- Name: group_member; Type: TABLE; Schema: public; Owner: koteseni
--

CREATE TABLE public.group_member (
    user_id uuid NOT NULL,
    group_id uuid NOT NULL,
    joined_at timestamp without time zone NOT NULL
);


ALTER TABLE public.group_member OWNER TO koteseni;

--
-- Name: location; Type: TABLE; Schema: public; Owner: koteseni
--

CREATE TABLE public.location (
    id uuid NOT NULL,
    name character varying NOT NULL,
    picture_url character varying,
    latitude double precision NOT NULL,
    longitude double precision NOT NULL
);


ALTER TABLE public.location OWNER TO koteseni;

--
-- Name: post; Type: TABLE; Schema: public; Owner: koteseni
--

CREATE TABLE public.post (
    id uuid NOT NULL,
    title character varying NOT NULL,
    description character varying NOT NULL,
    location_id uuid,
    author_id uuid NOT NULL,
    created_at timestamp without time zone NOT NULL,
    content_type character varying NOT NULL,
    visibility character varying NOT NULL,
    content_url character varying NOT NULL
);


ALTER TABLE public.post OWNER TO koteseni;

--
-- Name: post_comment; Type: TABLE; Schema: public; Owner: koteseni
--

CREATE TABLE public.post_comment (
    id uuid NOT NULL,
    post_id uuid NOT NULL,
    user_id uuid NOT NULL,
    content character varying NOT NULL,
    parent_id uuid
);


ALTER TABLE public.post_comment OWNER TO koteseni;

--
-- Name: post_group_visibility; Type: TABLE; Schema: public; Owner: koteseni
--

CREATE TABLE public.post_group_visibility (
    post_id uuid NOT NULL,
    group_id uuid NOT NULL
);


ALTER TABLE public.post_group_visibility OWNER TO koteseni;

--
-- Name: post_like; Type: TABLE; Schema: public; Owner: koteseni
--

CREATE TABLE public.post_like (
    post_id uuid NOT NULL,
    user_id uuid NOT NULL,
    created_at timestamp without time zone NOT NULL
);


ALTER TABLE public.post_like OWNER TO koteseni;

--
-- Name: post_tag; Type: TABLE; Schema: public; Owner: koteseni
--

CREATE TABLE public.post_tag (
    tag character varying NOT NULL,
    post_id uuid NOT NULL
);


ALTER TABLE public.post_tag OWNER TO koteseni;

--
-- Name: post_user_visibility; Type: TABLE; Schema: public; Owner: koteseni
--

CREATE TABLE public.post_user_visibility (
    post_id uuid NOT NULL,
    user_id uuid NOT NULL
);


ALTER TABLE public.post_user_visibility OWNER TO koteseni;

--
-- Name: seaql_migrations; Type: TABLE; Schema: public; Owner: koteseni
--

CREATE TABLE public.seaql_migrations (
    version character varying NOT NULL,
    applied_at bigint NOT NULL
);


ALTER TABLE public.seaql_migrations OWNER TO koteseni;

--
-- Name: user; Type: TABLE; Schema: public; Owner: koteseni
--

CREATE TABLE public."user" (
    id uuid NOT NULL,
    display_name character varying,
    email character varying,
    username character varying NOT NULL,
    avatar_url character varying,
    user_type public.user_type NOT NULL,
    wall_id uuid NOT NULL,
    is_blocked boolean NOT NULL,
    password_hash character varying NOT NULL
);


ALTER TABLE public."user" OWNER TO koteseni;

--
-- Name: wall; Type: TABLE; Schema: public; Owner: koteseni
--

CREATE TABLE public.wall (
    id uuid NOT NULL
);


ALTER TABLE public.wall OWNER TO koteseni;

--
-- Name: wall_post; Type: TABLE; Schema: public; Owner: koteseni
--

CREATE TABLE public.wall_post (
    wall_id uuid NOT NULL,
    post_id uuid NOT NULL
);


ALTER TABLE public.wall_post OWNER TO koteseni;

--
-- Data for Name: group; Type: TABLE DATA; Schema: public; Owner: koteseni
--

COPY public."group" (id, name, admin_id, wall_id) FROM stdin;
addd17e9-d1ef-44b3-ac0b-5f75da98dba1	Matyho kamaradi	4e3c05c3-6b68-4796-b6a3-66838e0c8d67	e6178e55-1b68-4ef8-b31c-8638ca572c90
\.


--
-- Data for Name: group_join_request; Type: TABLE DATA; Schema: public; Owner: koteseni
--

COPY public.group_join_request (id, created_at, resolved_at, group_id, user_id, status) FROM stdin;
0b69c91e-1b66-4886-ab72-99eb8992d74f	2024-11-24 22:29:19.528315	\N	addd17e9-d1ef-44b3-ac0b-5f75da98dba1	ea92344e-c91d-4df0-9813-a6fb5abfbd53	accepted
\.


--
-- Data for Name: group_member; Type: TABLE DATA; Schema: public; Owner: koteseni
--

COPY public.group_member (user_id, group_id, joined_at) FROM stdin;
4e3c05c3-6b68-4796-b6a3-66838e0c8d67	addd17e9-d1ef-44b3-ac0b-5f75da98dba1	2024-11-24 22:28:32.295371
ea92344e-c91d-4df0-9813-a6fb5abfbd53	addd17e9-d1ef-44b3-ac0b-5f75da98dba1	2024-11-24 22:29:44.843587
\.


--
-- Data for Name: location; Type: TABLE DATA; Schema: public; Owner: koteseni
--

COPY public.location (id, name, picture_url, latitude, longitude) FROM stdin;
\.


--
-- Data for Name: post; Type: TABLE DATA; Schema: public; Owner: koteseni
--

COPY public.post (id, title, description, location_id, author_id, created_at, content_type, visibility, content_url) FROM stdin;
fe3e8848-2c89-43a0-bfdd-c7150cc17901	fotka	moje fotka	\N	4e3c05c3-6b68-4796-b6a3-66838e0c8d67	2024-11-24 22:31:41.108261	photo	public	https://images.pexels.com/photos/2071882/pexels-photo-2071882.jpeg?auto=compress&cs=tinysrgb&dpr=1&w=500
519c5762-ebc1-44b8-8b73-0190c5ffc4c3	fotka2	moje fotka2	\N	4e3c05c3-6b68-4796-b6a3-66838e0c8d67	2024-11-24 22:32:57.496821	photo	private	https://cdn.pixabay.com/photo/2024/02/28/07/42/european-shorthair-8601492_640.jpg
\.


--
-- Data for Name: post_comment; Type: TABLE DATA; Schema: public; Owner: koteseni
--

COPY public.post_comment (id, post_id, user_id, content, parent_id) FROM stdin;
\.


--
-- Data for Name: post_group_visibility; Type: TABLE DATA; Schema: public; Owner: koteseni
--

COPY public.post_group_visibility (post_id, group_id) FROM stdin;
\.


--
-- Data for Name: post_like; Type: TABLE DATA; Schema: public; Owner: koteseni
--

COPY public.post_like (post_id, user_id, created_at) FROM stdin;
\.


--
-- Data for Name: post_tag; Type: TABLE DATA; Schema: public; Owner: koteseni
--

COPY public.post_tag (tag, post_id) FROM stdin;
\.


--
-- Data for Name: post_user_visibility; Type: TABLE DATA; Schema: public; Owner: koteseni
--

COPY public.post_user_visibility (post_id, user_id) FROM stdin;
\.


--
-- Data for Name: seaql_migrations; Type: TABLE DATA; Schema: public; Owner: koteseni
--

COPY public.seaql_migrations (version, applied_at) FROM stdin;
m20220101_000001_create_user_table	1732487125
m20241009_204556_create_location_table	1732487125
m20241009_204559_create_post_table	1732487125
m20241010_141247_create_post_related_tables	1732487125
m20241010_142036_create_wall_tables	1732487125
m20241010_142037_create_group_tables	1732487125
m20241010_142838_add_wall_to_user	1732487125
m20241028_210624_user_is_bloced	1732487125
m20241028_230949_location_lat_long	1732487125
m20241102_185447_pwd_hash_for_user	1732487125
m20241110_225527_group_join_request	1732487125
m20241121_111845_user_make_fields_unique	1732487125
m20241124_004127_visibility_tables	1732487125
\.


--
-- Data for Name: user; Type: TABLE DATA; Schema: public; Owner: koteseni
--

COPY public."user" (id, display_name, email, username, avatar_url, user_type, wall_id, is_blocked, password_hash) FROM stdin;
4e3c05c3-6b68-4796-b6a3-66838e0c8d67	\N	matyas@test.com	matyas	\N	regular	ede3e256-de24-41fa-9f4f-e50222ca2c52	f	$argon2id$v=19$m=19456,t=2,p=1$fWLJjq1tUh4DV2GsJ63sMA$iNKQX2que/PS0lf7HTlvI5/gQBYq2e2T7ei3t7pNLxI
e1249f52-b83c-4ae7-abe4-93b50652ddc2	\N	dominik@test.com	dominik	\N	administrator	b1fc54d8-85ca-481c-a74e-440740f88daa	f	$argon2id$v=19$m=19456,t=2,p=1$tEemlN1toG7WYWYtLyDlcA$O+rI8k2SEAZkr0TQJReaWriuSiQhhv0wM+cZvs/2jYw
ea92344e-c91d-4df0-9813-a6fb5abfbd53	\N	lukas@test.com	lukas	\N	moderator	d1c1eee4-8f1b-4c39-a641-c49c7b6e8117	f	$argon2id$v=19$m=19456,t=2,p=1$17FYrh6R/M5CPOv5BJBeEQ$ZXqKY/P0i9KZxNtzvUd+CA+Qn2Q5ok3i5Kb1CoOKIaM
\.


--
-- Data for Name: wall; Type: TABLE DATA; Schema: public; Owner: koteseni
--

COPY public.wall (id) FROM stdin;
b1fc54d8-85ca-481c-a74e-440740f88daa
74ad4d63-191f-47f2-8bd1-9e614c35e73f
d1c1eee4-8f1b-4c39-a641-c49c7b6e8117
ede3e256-de24-41fa-9f4f-e50222ca2c52
e6178e55-1b68-4ef8-b31c-8638ca572c90
\.


--
-- Data for Name: wall_post; Type: TABLE DATA; Schema: public; Owner: koteseni
--

COPY public.wall_post (wall_id, post_id) FROM stdin;
ede3e256-de24-41fa-9f4f-e50222ca2c52	fe3e8848-2c89-43a0-bfdd-c7150cc17901
ede3e256-de24-41fa-9f4f-e50222ca2c52	519c5762-ebc1-44b8-8b73-0190c5ffc4c3
\.


--
-- Name: group_join_request group_join_request_pkey; Type: CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public.group_join_request
    ADD CONSTRAINT group_join_request_pkey PRIMARY KEY (id);


--
-- Name: group_member group_member_pkey; Type: CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public.group_member
    ADD CONSTRAINT group_member_pkey PRIMARY KEY (user_id, group_id);


--
-- Name: group group_pkey; Type: CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public."group"
    ADD CONSTRAINT group_pkey PRIMARY KEY (id);


--
-- Name: location location_pkey; Type: CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public.location
    ADD CONSTRAINT location_pkey PRIMARY KEY (id);


--
-- Name: post_comment post_comment_pkey; Type: CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public.post_comment
    ADD CONSTRAINT post_comment_pkey PRIMARY KEY (id);


--
-- Name: post_group_visibility post_group_visibility_pkey; Type: CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public.post_group_visibility
    ADD CONSTRAINT post_group_visibility_pkey PRIMARY KEY (post_id, group_id);


--
-- Name: post_like post_like_pkey; Type: CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public.post_like
    ADD CONSTRAINT post_like_pkey PRIMARY KEY (post_id, user_id);


--
-- Name: post post_pkey; Type: CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public.post
    ADD CONSTRAINT post_pkey PRIMARY KEY (id);


--
-- Name: post_tag post_tag_pkey; Type: CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public.post_tag
    ADD CONSTRAINT post_tag_pkey PRIMARY KEY (tag);


--
-- Name: post_user_visibility post_visibility_pkey; Type: CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public.post_user_visibility
    ADD CONSTRAINT post_visibility_pkey PRIMARY KEY (post_id, user_id);


--
-- Name: seaql_migrations seaql_migrations_pkey; Type: CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public.seaql_migrations
    ADD CONSTRAINT seaql_migrations_pkey PRIMARY KEY (version);


--
-- Name: user user_pkey; Type: CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public."user"
    ADD CONSTRAINT user_pkey PRIMARY KEY (id);


--
-- Name: wall wall_pkey; Type: CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public.wall
    ADD CONSTRAINT wall_pkey PRIMARY KEY (id);


--
-- Name: wall_post wall_post_pkey; Type: CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public.wall_post
    ADD CONSTRAINT wall_post_pkey PRIMARY KEY (wall_id, post_id);


--
-- Name: unique_email; Type: INDEX; Schema: public; Owner: koteseni
--

CREATE UNIQUE INDEX unique_email ON public."user" USING btree (email);


--
-- Name: unique_username; Type: INDEX; Schema: public; Owner: koteseni
--

CREATE UNIQUE INDEX unique_username ON public."user" USING btree (username);


--
-- Name: group fk_group_admin; Type: FK CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public."group"
    ADD CONSTRAINT fk_group_admin FOREIGN KEY (admin_id) REFERENCES public."user"(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: group_member fk_group_member_group; Type: FK CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public.group_member
    ADD CONSTRAINT fk_group_member_group FOREIGN KEY (group_id) REFERENCES public."group"(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: group_member fk_group_member_user; Type: FK CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public.group_member
    ADD CONSTRAINT fk_group_member_user FOREIGN KEY (user_id) REFERENCES public."user"(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: group fk_group_wall; Type: FK CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public."group"
    ADD CONSTRAINT fk_group_wall FOREIGN KEY (wall_id) REFERENCES public.wall(id);


--
-- Name: post fk_post_author; Type: FK CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public.post
    ADD CONSTRAINT fk_post_author FOREIGN KEY (author_id) REFERENCES public."user"(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: post_comment fk_post_comment_parent; Type: FK CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public.post_comment
    ADD CONSTRAINT fk_post_comment_parent FOREIGN KEY (parent_id) REFERENCES public.post_comment(id);


--
-- Name: post_comment fk_post_comment_post; Type: FK CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public.post_comment
    ADD CONSTRAINT fk_post_comment_post FOREIGN KEY (post_id) REFERENCES public.post(id);


--
-- Name: post_comment fk_post_comment_user; Type: FK CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public.post_comment
    ADD CONSTRAINT fk_post_comment_user FOREIGN KEY (user_id) REFERENCES public."user"(id);


--
-- Name: post_group_visibility fk_post_group_visibility_group; Type: FK CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public.post_group_visibility
    ADD CONSTRAINT fk_post_group_visibility_group FOREIGN KEY (group_id) REFERENCES public."group"(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: post_group_visibility fk_post_group_visibility_post; Type: FK CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public.post_group_visibility
    ADD CONSTRAINT fk_post_group_visibility_post FOREIGN KEY (post_id) REFERENCES public.post(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: post_like fk_post_like_post; Type: FK CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public.post_like
    ADD CONSTRAINT fk_post_like_post FOREIGN KEY (post_id) REFERENCES public.post(id);


--
-- Name: post_like fk_post_like_user; Type: FK CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public.post_like
    ADD CONSTRAINT fk_post_like_user FOREIGN KEY (user_id) REFERENCES public."user"(id);


--
-- Name: post fk_post_location; Type: FK CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public.post
    ADD CONSTRAINT fk_post_location FOREIGN KEY (location_id) REFERENCES public.location(id);


--
-- Name: post_tag fk_post_tag_post; Type: FK CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public.post_tag
    ADD CONSTRAINT fk_post_tag_post FOREIGN KEY (post_id) REFERENCES public.post(id);


--
-- Name: post_user_visibility fk_post_visibility_post; Type: FK CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public.post_user_visibility
    ADD CONSTRAINT fk_post_visibility_post FOREIGN KEY (post_id) REFERENCES public.post(id);


--
-- Name: post_user_visibility fk_post_visibility_user; Type: FK CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public.post_user_visibility
    ADD CONSTRAINT fk_post_visibility_user FOREIGN KEY (user_id) REFERENCES public."user"(id);


--
-- Name: user fk_user_wall; Type: FK CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public."user"
    ADD CONSTRAINT fk_user_wall FOREIGN KEY (wall_id) REFERENCES public.wall(id);


--
-- Name: wall_post fk_wall_post_post; Type: FK CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public.wall_post
    ADD CONSTRAINT fk_wall_post_post FOREIGN KEY (post_id) REFERENCES public.post(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: wall_post fk_wall_post_wall; Type: FK CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public.wall_post
    ADD CONSTRAINT fk_wall_post_wall FOREIGN KEY (wall_id) REFERENCES public.wall(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: group_join_request group_join_request_group_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public.group_join_request
    ADD CONSTRAINT group_join_request_group_id_fkey FOREIGN KEY (group_id) REFERENCES public."group"(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- Name: group_join_request group_join_request_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: koteseni
--

ALTER TABLE ONLY public.group_join_request
    ADD CONSTRAINT group_join_request_user_id_fkey FOREIGN KEY (user_id) REFERENCES public."user"(id) ON UPDATE CASCADE ON DELETE CASCADE;


--
-- PostgreSQL database dump complete
--

