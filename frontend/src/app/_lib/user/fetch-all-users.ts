import type { User } from "@/app/_lib/user/fetch-user";
import { backendFetch, checkResponse } from "../backend-fetch";
import type { Role } from "@/app/_types/user";

interface Filters {
  role?: Role;
  isBlocked?: boolean;
  username?: string;
}

export async function fetchAllUsers(filters?: Filters): Promise<User[]> {
  const searchParams = new URLSearchParams();

  if (filters?.role) {
    searchParams.append("role", filters.role);
  }

  if (typeof filters?.isBlocked === "boolean") {
    searchParams.append("is_blocked", filters.isBlocked.toString());
  }

  if (filters?.username) {
    searchParams.append("username", filters.username);
  }

  const response = await backendFetch(`/api/users?${searchParams.toString()}`);

  await checkResponse(response, { customError: "Failed to fetch users" });

  // biome-ignore lint/suspicious/noExplicitAny: <explanation>
  const data: any[] = await response.json();

  return data.map((entry) => {
    return {
      id: entry.id,
      username: entry.username,
      displayName: entry.display_name,
      description: "bio", // TODO: Description
      email: entry.email,
      role: entry.user_type,
      isBlocked: entry.is_blocked,
      avatar: {
        src: entry.avatar_url,
      },
      wallId: entry.wall_id,
    };
  });
}
