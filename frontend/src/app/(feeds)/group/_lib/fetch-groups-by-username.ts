import { backendFetch, checkResponse } from "@/app/_lib/backend-fetch";
import type { Role } from "@/app/_types/user";

export type Group = {
  id: string;
  groupname: string;
  admin: {
    id: string;
    displayName: string;
    username: string;
    email: string;
    avatar: {
      src: string | undefined;
    };
    role: Role;
  };
  wallId: string;
};

export async function fetchGroupsByUsername(
  groupname: string
): Promise<Group[]> {
  const response = await backendFetch(`/api/groups?query=${groupname}`);

  await checkResponse(response);

  // biome-ignore lint/suspicious/noExplicitAny: <explanation>
  const { groups }: { groups: any[] } = await response.json();

  return groups.map((entry) => ({
    id: entry.id,
    groupname: entry.name,
    admin: {
      id: entry.admin.id,
      displayName: entry.admin.display_name,
      username: entry.admin.username,
      email: entry.admin.email,
      avatar: {
        src: entry.admin.avatar_url,
      },
      role: entry.admin.user_type,
    },
    wallId: entry.wall_id,
  }));
}

export async function fetchGroupByUsername(groupname: string): Promise<Group> {
  const groups = await fetchGroupsByUsername(groupname);

  const group = groups.find((group) => group.groupname === groupname);

  if (!group) {
    throw new Error("Group not found");
  }

  return group;
}
