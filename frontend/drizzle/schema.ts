import { pgTable, varchar, bigint, uuid, foreignKey, timestamp, primaryKey, pgEnum } from "drizzle-orm/pg-core"
  import { sql } from "drizzle-orm"

export const postType = pgEnum("post_type", ['image'])
export const postVisibilityType = pgEnum("post_visibility_type", ['public', 'private'])
export const userType = pgEnum("user_type", ['regular', 'moderator', 'administrator'])



export const seaqlMigrations = pgTable("seaql_migrations", {
	version: varchar().primaryKey().notNull(),
	// You can use { mode: "bigint" } if numbers are exceeding js number limitations
	appliedAt: bigint("applied_at", { mode: "number" }).notNull(),
});

export const location = pgTable("location", {
	id: uuid().primaryKey().notNull(),
	name: varchar().notNull(),
	pictureUrl: varchar("picture_url"),
});

export const post = pgTable("post", {
	id: uuid().primaryKey().notNull(),
	title: varchar().notNull(),
	description: varchar().notNull(),
	locationId: uuid("location_id"),
	authorId: uuid("author_id").notNull(),
	createdAt: timestamp("created_at", { mode: 'string' }).notNull(),
	contentType: varchar("content_type").notNull(),
	visibility: varchar().notNull(),
	contentUrl: varchar("content_url").notNull(),
},
(table) => {
	return {
		fkPostLocation: foreignKey({
			columns: [table.locationId],
			foreignColumns: [location.id],
			name: "fk_post_location"
		}),
		fkPostAuthor: foreignKey({
			columns: [table.authorId],
			foreignColumns: [user.id],
			name: "fk_post_author"
		}),
	}
});

export const postComment = pgTable("post_comment", {
	id: uuid().primaryKey().notNull(),
	postId: uuid("post_id").notNull(),
	userId: uuid("user_id").notNull(),
	content: varchar().notNull(),
	parentId: uuid("parent_id"),
},
(table) => {
	return {
		fkPostCommentPost: foreignKey({
			columns: [table.postId],
			foreignColumns: [post.id],
			name: "fk_post_comment_post"
		}),
		fkPostCommentUser: foreignKey({
			columns: [table.userId],
			foreignColumns: [user.id],
			name: "fk_post_comment_user"
		}),
		fkPostCommentParent: foreignKey({
			columns: [table.parentId],
			foreignColumns: [table.id],
			name: "fk_post_comment_parent"
		}),
	}
});

export const postTag = pgTable("post_tag", {
	tag: varchar().primaryKey().notNull(),
	postId: uuid("post_id").notNull(),
},
(table) => {
	return {
		fkPostTagPost: foreignKey({
			columns: [table.postId],
			foreignColumns: [post.id],
			name: "fk_post_tag_post"
		}),
	}
});

export const wall = pgTable("wall", {
	id: uuid().primaryKey().notNull(),
});

export const group = pgTable("group", {
	id: uuid().primaryKey().notNull(),
	name: varchar().notNull(),
	adminId: uuid("admin_id").notNull(),
	wallId: uuid("wall_id").notNull(),
},
(table) => {
	return {
		fkGroupAdmin: foreignKey({
			columns: [table.adminId],
			foreignColumns: [user.id],
			name: "fk_group_admin"
		}),
		fkGroupWall: foreignKey({
			columns: [table.wallId],
			foreignColumns: [post.id],
			name: "fk_group_wall"
		}),
	}
});

export const user = pgTable("user", {
	id: uuid().primaryKey().notNull(),
	displayName: varchar("display_name").notNull(),
	username: varchar().notNull(),
	email: varchar().notNull(),
	avatarUrl: varchar("avatar_url"),
	userType: userType("user_type").notNull(),
	wallId: uuid("wall_id"),
},
(table) => {
	return {
		fkUserWall: foreignKey({
			columns: [table.wallId],
			foreignColumns: [wall.id],
			name: "fk_user_wall"
		}),
	}
});

export const postVisibility = pgTable("post_visibility", {
	postId: uuid("post_id").notNull(),
	userId: uuid("user_id").notNull(),
},
(table) => {
	return {
		fkPostVisibilityPost: foreignKey({
			columns: [table.postId],
			foreignColumns: [post.id],
			name: "fk_post_visibility_post"
		}),
		fkPostVisibilityUser: foreignKey({
			columns: [table.userId],
			foreignColumns: [user.id],
			name: "fk_post_visibility_user"
		}),
		postVisibilityPkey: primaryKey({ columns: [table.postId, table.userId], name: "post_visibility_pkey"}),
	}
});

export const wallPost = pgTable("wall_post", {
	wallId: uuid("wall_id").notNull(),
	postId: uuid("post_id").notNull(),
},
(table) => {
	return {
		fkWallPostWall: foreignKey({
			columns: [table.wallId],
			foreignColumns: [wall.id],
			name: "fk_wall_post_wall"
		}),
		fkWallPostPost: foreignKey({
			columns: [table.postId],
			foreignColumns: [post.id],
			name: "fk_wall_post_post"
		}),
		wallPostPkey: primaryKey({ columns: [table.wallId, table.postId], name: "wall_post_pkey"}),
	}
});

export const postLike = pgTable("post_like", {
	postId: uuid("post_id").notNull(),
	userId: uuid("user_id").notNull(),
	createdAt: timestamp("created_at", { mode: 'string' }).notNull(),
},
(table) => {
	return {
		fkPostLikePost: foreignKey({
			columns: [table.postId],
			foreignColumns: [post.id],
			name: "fk_post_like_post"
		}),
		fkPostLikeUser: foreignKey({
			columns: [table.userId],
			foreignColumns: [user.id],
			name: "fk_post_like_user"
		}),
		postLikePkey: primaryKey({ columns: [table.postId, table.userId], name: "post_like_pkey"}),
	}
});

export const groupMember = pgTable("group_member", {
	userId: uuid("user_id").notNull(),
	groupId: uuid("group_id").notNull(),
	joinedAt: timestamp("joined_at", { mode: 'string' }).notNull(),
},
(table) => {
	return {
		fkGroupMemberUser: foreignKey({
			columns: [table.userId],
			foreignColumns: [user.id],
			name: "fk_group_member_user"
		}),
		fkGroupMemberGroup: foreignKey({
			columns: [table.groupId],
			foreignColumns: [group.id],
			name: "fk_group_member_group"
		}),
		groupMemberPkey: primaryKey({ columns: [table.userId, table.groupId], name: "group_member_pkey"}),
	}
});