import { backendFetch, checkResponse } from "@/app/_lib/backend-fetch";
import { fetchUserById } from "@/app/_lib/user/fetch-user";
import type { Comment } from "@/app/_types/comments";
import type { User } from "@/app/_types/user";

export type Post = {
  id: string;
  title: string;
  description: string;
  tags: string[];
  image: {
    src: string;
  };
  user: User;
  visibility: "public" | "private";
  likeCount: number;
  comments: Comment[];
  allowedUsers: User[];
  allowedGroups: User[];
};

export async function fetchPost(
  postId: string,
  opts?: RequestInit
): Promise<Post> {
  const response = await backendFetch(`/api/posts/${postId}`, opts);

  await checkResponse(response);

  const data = await response.json();

  const user = await fetchUserById(data.author_id);

  return {
    id: data.id,
    title: data.title,
    description: data.description,
    image: { src: data.content_url },
    tags: data.tags,
    user: {
      id: user.id,
      username: user.username,
      avatar: user.avatar,
    },
    visibility: data.visibility,
    likeCount: data.like_count,
    // biome-ignore lint/suspicious/noExplicitAny: <explanation>
    comments: data.comments.map((comment: any) => ({
      id: comment.id,
      content: comment.content,
      user: {
        id: comment.user_id,
        username: comment.username,
        avatar: {
          src: comment.avatar_url,
          width: 32,
          height: 32,
        },
      },
    })),
    // biome-ignore lint/suspicious/noExplicitAny: <explanation>
    allowedGroups: data.allowed_groups.map((group: any) => ({
      ...group,
      avatar: { src: undefined },
    })),
    allowedUsers: data.allowed_users,
  };
}
