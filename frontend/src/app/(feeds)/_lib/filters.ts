export interface FeedFilters {
  sorting?: "top" | "new";
  pagination?: {
    pageIndex: number;
    pageSize: number;
  };
}

export function parseFilters(filters: FeedFilters): URLSearchParams {
  const searchParams = new URLSearchParams();

  if (filters.sorting) {
    const map = {
      top: "mostLiked",
      new: "newest",
    };

    searchParams.set("sort_by", map[filters.sorting]);
  }

  if (filters.pagination) {
    const offset =
      (filters.pagination.pageIndex - 1) * filters.pagination.pageSize;
    const limit = filters.pagination.pageSize;

    searchParams.set("offset", offset.toString());
    searchParams.set("limit", limit.toString());
  }

  return searchParams;
}
