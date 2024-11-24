import type { User } from "./user";

export type Comments = {
  id: number;
  user: User;
  content: string;
};
