import { backendFetch } from "@/app/_lib/backend-fetch";

export type User = {
  id: string;
  username: string;
  displayName: string;
  description: string;
  email: string;
  role: "regular" | "administrator" | "moderator";
  isBlocked: boolean;
  avatar: {
    src: string | undefined;
  };
  wallId: string;
};

// biome-ignore lint/suspicious/noExplicitAny: <explanation>
function transform(data: any) {
  return {
    id: data.id,
    username: data.username,
    displayName: data.display_name,
    description: "bio", // TODO: Description
    email: data.email,
    role: data.user_type,
    isBlocked: data.is_blocked,
    avatar: {
      src: data.avatar_url,
    },
    wallId: data.wall_id,
  };
}

export async function fetchUserById(id: string): Promise<User> {
  const response = await backendFetch(`/api/users/id/${id}`);

  if (!response.ok) {
    if (response.status === 404) {
      throw new Error("User not found");
    }

    throw new Error("Failed to fetch user");
  }

  const data = await response.json();

  return transform(data);
}

export async function fetchUserByUsername(username: string): Promise<User> {
  const response = await backendFetch(`/api/users/${username}`);

  if (!response.ok) {
    if (response.status === 404) {
      throw new Error("User not found");
    }

    throw new Error("Failed to fetch user");
  }

  const data = await response.json();

  return transform(data);
}
