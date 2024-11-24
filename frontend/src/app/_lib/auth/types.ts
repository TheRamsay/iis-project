import type { schema } from "../db";

export type Session = {
  userId: string;
  username: string;
  avatar: {
    src: string | undefined;
  };
  role: (typeof schema)["userType"]["enumValues"][number];
};
