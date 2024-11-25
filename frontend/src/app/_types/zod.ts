import { z } from "zod";

export const myz = {
  username: z.string().min(3).max(15),
  displayName: z.string().min(3).max(15),
  password: z.string().min(4).max(20),
  title: z.string().min(3).max(15),
  description: z.string().max(255),
};
