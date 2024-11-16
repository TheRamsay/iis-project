import { z } from "zod";
import { SortOption } from "./types";

export const feedSearchSchema = z.object({
  sorting: z.nativeEnum(SortOption).default(SortOption.New),
});
