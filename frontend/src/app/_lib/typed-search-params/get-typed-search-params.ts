import type { z } from "zod";

export const getTypedSearchParams = <T extends z.Schema>(
  schema: T,
  searchParams: Record<string, string>
) => {
  return schema.parse(searchParams) as z.infer<typeof schema>;
};
