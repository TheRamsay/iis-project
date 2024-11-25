import { backendFetch, checkResponse } from "@/app/_lib/backend-fetch";
import type { Post } from "@/app/post/_lib/fetch-post";
import { type FeedFilters, parseFilters } from "./filters";

export async function fetchWallByTag(
  tag: string,
  filters: FeedFilters = {}
): Promise<Post[]> {
  const searchParams = parseFilters(filters);

  const response = await backendFetch(
    `/api/walls/tag/${tag}?${searchParams.toString()}`
  );

  await checkResponse(response);

  // biome-ignore lint/suspicious/noExplicitAny: <explanation>
  const { posts }: { posts: any[] } = await response.json();

  return posts.map(({ post, author, comments, likes, tags }) => ({
    id: post.id,
    title: post.title,
    description: post.description,
    image: {
      src: post.content_url,
    },
    // biome-ignore lint/suspicious/noExplicitAny: <explanation>
    comments: comments.map((comment: any) => ({
      id: comment.id,
      content: comment.content,
      user: {
        id: comment.user_id,
        username: comment.username,
        avatar: {
          src: comment.avatar_url,
        },
      },
    })),
    likeCount: likes.length,
    tags,
    user: {
      id: author.id,
      username: author.username,
      avatar: {
        src: author.avatar_url,
      },
    },
    visibility: post.visibility,
    allowedGroups: [],
    allowedUsers: [],
  }));
}
