import type { schema } from "../db";

export type Session = {
  userId: string;
  username: string;
  avatar: {
    src: string;
    width: number;
    height: number;
  };
  role: (typeof schema)["userType"]["enumValues"][number];
};
