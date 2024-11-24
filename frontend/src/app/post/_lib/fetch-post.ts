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
    width: number;
    height: number;
  };
  user: User;
  visibility: "public" | "private";
  likeCount: number;
  comments: Comment[];
};

export async function fetchPost(postId: string): Promise<Post> {
  const response = await backendFetch(`/api/posts/${postId}`);

  await checkResponse(response);

  const data = await response.json();

  const user = await fetchUserById(data.author_id);

  return {
    id: data.id,
    title: data.title,
    description: data.description,
    image: { src: data.content_url, width: 32, height: 32 },
    tags: [],
    user: {
      id: user.id,
      username: user.username,
      avatar: user.avatar,
    },
    visibility: data.visibility,
    likeCount: data.like_count,
    comments: data.comments,
  };
}
