import { backendFetch, checkResponse } from "@/app/_lib/backend-fetch";
import type { Post } from "@/app/post/_lib/fetch-post";
import { parseFilters, type FeedFilters } from "./filters";

export async function fetchMyFeed(filters: FeedFilters = {}): Promise<Post[]> {
  const searchParams = parseFilters(filters);

  const response = await backendFetch(
    `/api/walls/feed?${searchParams.toString()}`
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
  }));
}
