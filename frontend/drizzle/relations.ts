import { relations } from "drizzle-orm/relations";
import { location, post, user, postComment, postTag, group, wall, postVisibility, wallPost, postLike, groupMember } from "./schema";

export const postRelations = relations(post, ({one, many}) => ({
	location: one(location, {
		fields: [post.locationId],
		references: [location.id]
	}),
	user: one(user, {
		fields: [post.authorId],
		references: [user.id]
	}),
	postComments: many(postComment),
	postTags: many(postTag),
	groups: many(group),
	postVisibilities: many(postVisibility),
	wallPosts: many(wallPost),
	postLikes: many(postLike),
}));

export const locationRelations = relations(location, ({many}) => ({
	posts: many(post),
}));

export const userRelations = relations(user, ({one, many}) => ({
	posts: many(post),
	postComments: many(postComment),
	groups: many(group),
	wall: one(wall, {
		fields: [user.wallId],
		references: [wall.id]
	}),
	postVisibilities: many(postVisibility),
	postLikes: many(postLike),
	groupMembers: many(groupMember),
}));

export const postCommentRelations = relations(postComment, ({one, many}) => ({
	post: one(post, {
		fields: [postComment.postId],
		references: [post.id]
	}),
	user: one(user, {
		fields: [postComment.userId],
		references: [user.id]
	}),
	postComment: one(postComment, {
		fields: [postComment.parentId],
		references: [postComment.id],
		relationName: "postComment_parentId_postComment_id"
	}),
	postComments: many(postComment, {
		relationName: "postComment_parentId_postComment_id"
	}),
}));

export const postTagRelations = relations(postTag, ({one}) => ({
	post: one(post, {
		fields: [postTag.postId],
		references: [post.id]
	}),
}));

export const groupRelations = relations(group, ({one, many}) => ({
	user: one(user, {
		fields: [group.adminId],
		references: [user.id]
	}),
	post: one(post, {
		fields: [group.wallId],
		references: [post.id]
	}),
	groupMembers: many(groupMember),
}));

export const wallRelations = relations(wall, ({many}) => ({
	users: many(user),
	wallPosts: many(wallPost),
}));

export const postVisibilityRelations = relations(postVisibility, ({one}) => ({
	post: one(post, {
		fields: [postVisibility.postId],
		references: [post.id]
	}),
	user: one(user, {
		fields: [postVisibility.userId],
		references: [user.id]
	}),
}));

export const wallPostRelations = relations(wallPost, ({one}) => ({
	wall: one(wall, {
		fields: [wallPost.wallId],
		references: [wall.id]
	}),
	post: one(post, {
		fields: [wallPost.postId],
		references: [post.id]
	}),
}));

export const postLikeRelations = relations(postLike, ({one}) => ({
	post: one(post, {
		fields: [postLike.postId],
		references: [post.id]
	}),
	user: one(user, {
		fields: [postLike.userId],
		references: [user.id]
	}),
}));

export const groupMemberRelations = relations(groupMember, ({one}) => ({
	user: one(user, {
		fields: [groupMember.userId],
		references: [user.id]
	}),
	group: one(group, {
		fields: [groupMember.groupId],
		references: [group.id]
	}),
}));