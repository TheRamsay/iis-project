-- Current sql file was generated after introspecting the database
-- If you want to run this migration please uncomment this code before executing migrations
/*
DO $$ BEGIN
 CREATE TYPE "public"."post_type" AS ENUM('image');
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;
--> statement-breakpoint
DO $$ BEGIN
 CREATE TYPE "public"."post_visibility_type" AS ENUM('public', 'private');
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;
--> statement-breakpoint
DO $$ BEGIN
 CREATE TYPE "public"."user_type" AS ENUM('regular', 'moderator', 'administrator');
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "seaql_migrations" (
	"version" varchar PRIMARY KEY NOT NULL,
	"applied_at" bigint NOT NULL
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "location" (
	"id" uuid PRIMARY KEY NOT NULL,
	"name" varchar NOT NULL,
	"picture_url" varchar
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "post" (
	"id" uuid PRIMARY KEY NOT NULL,
	"title" varchar NOT NULL,
	"description" varchar NOT NULL,
	"location_id" uuid,
	"author_id" uuid NOT NULL,
	"created_at" timestamp NOT NULL,
	"content_type" varchar NOT NULL,
	"visibility" varchar NOT NULL,
	"content_url" varchar NOT NULL
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "post_comment" (
	"id" uuid PRIMARY KEY NOT NULL,
	"post_id" uuid NOT NULL,
	"user_id" uuid NOT NULL,
	"content" varchar NOT NULL,
	"parent_id" uuid
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "post_tag" (
	"tag" varchar PRIMARY KEY NOT NULL,
	"post_id" uuid NOT NULL
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "wall" (
	"id" uuid PRIMARY KEY NOT NULL
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "group" (
	"id" uuid PRIMARY KEY NOT NULL,
	"name" varchar NOT NULL,
	"admin_id" uuid NOT NULL,
	"wall_id" uuid NOT NULL
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "user" (
	"id" uuid PRIMARY KEY NOT NULL,
	"display_name" varchar NOT NULL,
	"username" varchar NOT NULL,
	"email" varchar NOT NULL,
	"avatar_url" varchar,
	"user_type" "user_type" NOT NULL,
	"wall_id" uuid
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "post_visibility" (
	"post_id" uuid NOT NULL,
	"user_id" uuid NOT NULL,
	CONSTRAINT "post_visibility_pkey" PRIMARY KEY("post_id","user_id")
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "wall_post" (
	"wall_id" uuid NOT NULL,
	"post_id" uuid NOT NULL,
	CONSTRAINT "wall_post_pkey" PRIMARY KEY("wall_id","post_id")
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "post_like" (
	"post_id" uuid NOT NULL,
	"user_id" uuid NOT NULL,
	"created_at" timestamp NOT NULL,
	CONSTRAINT "post_like_pkey" PRIMARY KEY("post_id","user_id")
);
--> statement-breakpoint
CREATE TABLE IF NOT EXISTS "group_member" (
	"user_id" uuid NOT NULL,
	"group_id" uuid NOT NULL,
	"joined_at" timestamp NOT NULL,
	CONSTRAINT "group_member_pkey" PRIMARY KEY("user_id","group_id")
);
--> statement-breakpoint
DO $$ BEGIN
 ALTER TABLE "post" ADD CONSTRAINT "fk_post_location" FOREIGN KEY ("location_id") REFERENCES "public"."location"("id") ON DELETE no action ON UPDATE no action;
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;
--> statement-breakpoint
DO $$ BEGIN
 ALTER TABLE "post" ADD CONSTRAINT "fk_post_author" FOREIGN KEY ("author_id") REFERENCES "public"."user"("id") ON DELETE no action ON UPDATE no action;
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;
--> statement-breakpoint
DO $$ BEGIN
 ALTER TABLE "post_comment" ADD CONSTRAINT "fk_post_comment_post" FOREIGN KEY ("post_id") REFERENCES "public"."post"("id") ON DELETE no action ON UPDATE no action;
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;
--> statement-breakpoint
DO $$ BEGIN
 ALTER TABLE "post_comment" ADD CONSTRAINT "fk_post_comment_user" FOREIGN KEY ("user_id") REFERENCES "public"."user"("id") ON DELETE no action ON UPDATE no action;
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;
--> statement-breakpoint
DO $$ BEGIN
 ALTER TABLE "post_comment" ADD CONSTRAINT "fk_post_comment_parent" FOREIGN KEY ("parent_id") REFERENCES "public"."post_comment"("id") ON DELETE no action ON UPDATE no action;
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;
--> statement-breakpoint
DO $$ BEGIN
 ALTER TABLE "post_tag" ADD CONSTRAINT "fk_post_tag_post" FOREIGN KEY ("post_id") REFERENCES "public"."post"("id") ON DELETE no action ON UPDATE no action;
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;
--> statement-breakpoint
DO $$ BEGIN
 ALTER TABLE "group" ADD CONSTRAINT "fk_group_admin" FOREIGN KEY ("admin_id") REFERENCES "public"."user"("id") ON DELETE no action ON UPDATE no action;
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;
--> statement-breakpoint
DO $$ BEGIN
 ALTER TABLE "group" ADD CONSTRAINT "fk_group_wall" FOREIGN KEY ("wall_id") REFERENCES "public"."post"("id") ON DELETE no action ON UPDATE no action;
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;
--> statement-breakpoint
DO $$ BEGIN
 ALTER TABLE "user" ADD CONSTRAINT "fk_user_wall" FOREIGN KEY ("wall_id") REFERENCES "public"."wall"("id") ON DELETE no action ON UPDATE no action;
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;
--> statement-breakpoint
DO $$ BEGIN
 ALTER TABLE "post_visibility" ADD CONSTRAINT "fk_post_visibility_post" FOREIGN KEY ("post_id") REFERENCES "public"."post"("id") ON DELETE no action ON UPDATE no action;
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;
--> statement-breakpoint
DO $$ BEGIN
 ALTER TABLE "post_visibility" ADD CONSTRAINT "fk_post_visibility_user" FOREIGN KEY ("user_id") REFERENCES "public"."user"("id") ON DELETE no action ON UPDATE no action;
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;
--> statement-breakpoint
DO $$ BEGIN
 ALTER TABLE "wall_post" ADD CONSTRAINT "fk_wall_post_wall" FOREIGN KEY ("wall_id") REFERENCES "public"."wall"("id") ON DELETE no action ON UPDATE no action;
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;
--> statement-breakpoint
DO $$ BEGIN
 ALTER TABLE "wall_post" ADD CONSTRAINT "fk_wall_post_post" FOREIGN KEY ("post_id") REFERENCES "public"."post"("id") ON DELETE no action ON UPDATE no action;
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;
--> statement-breakpoint
DO $$ BEGIN
 ALTER TABLE "post_like" ADD CONSTRAINT "fk_post_like_post" FOREIGN KEY ("post_id") REFERENCES "public"."post"("id") ON DELETE no action ON UPDATE no action;
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;
--> statement-breakpoint
DO $$ BEGIN
 ALTER TABLE "post_like" ADD CONSTRAINT "fk_post_like_user" FOREIGN KEY ("user_id") REFERENCES "public"."user"("id") ON DELETE no action ON UPDATE no action;
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;
--> statement-breakpoint
DO $$ BEGIN
 ALTER TABLE "group_member" ADD CONSTRAINT "fk_group_member_user" FOREIGN KEY ("user_id") REFERENCES "public"."user"("id") ON DELETE no action ON UPDATE no action;
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;
--> statement-breakpoint
DO $$ BEGIN
 ALTER TABLE "group_member" ADD CONSTRAINT "fk_group_member_group" FOREIGN KEY ("group_id") REFERENCES "public"."group"("id") ON DELETE no action ON UPDATE no action;
EXCEPTION
 WHEN duplicate_object THEN null;
END $$;

*/