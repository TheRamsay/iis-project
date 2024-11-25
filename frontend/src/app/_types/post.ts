import type { Comment } from "./comments";
import type { User } from "./user";

export type Post = {
  id: number;
  image: {
    src: string;
  };
  title: string;
  description?: string;
  tags: string[];
  location?: {
    lat: number;
    lng: number;
  };
  user: User;
  likeCount: number;
  comments: Comment[];
};

export const dummyPosts: Post[] = Array(10)
  .fill(0)
  .map((_, i) => ({
    id: i,
    image: {
      src: "https://picsum.photos/256",
    },
    title: "This is a post",
    description: "This is a post description",
    tags: ["tag1", "tag2"],
    user: {
      id: "1",
      username: "fitstagram",
      avatar: {
        src: "https://avatars.githubusercontent.com/u/7655549?v=4",
      },
    },
    likeCount: 10,
    comments: [
      {
        id: 1,
        user: {
          id: "1",
          username: "fitstagram",
          avatar: {
            src: "https://avatars.githubusercontent.com/u/7655549?v=4",
            width: 32,
            height: 32,
          },
        },
        content: "This is a comment",
      },
      {
        id: 2,
        user: {
          id: "1",
          username: "fitstagram",
          avatar: {
            src: "https://avatars.githubusercontent.com/u/7655549?v=4",
            width: 32,
            height: 32,
          },
        },
        content: "This is a comment",
      },
    ],
  }));
