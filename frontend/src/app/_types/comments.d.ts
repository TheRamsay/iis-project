import type { User } from "./user";

export type Comment = {
  id: number;
  user: User;
  content: string;
};
