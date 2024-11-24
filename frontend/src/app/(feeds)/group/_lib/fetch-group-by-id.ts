import { backendFetch, checkResponse } from "@/app/_lib/backend-fetch";
import type { Group } from "./fetch-groups-by-username";

export async function fetchGroupById(groupId: string): Promise<Group> {
  const response = await backendFetch(`/api/groups/${groupId}`);

  await checkResponse(response);

  // biome-ignore lint/suspicious/noExplicitAny: <explanation>
  const { group }: { group: any } = await response.json();

  return {
    id: group.id,
    groupname: group.name,
    admin: {
      id: group.admin.id,
      displayName: group.admin.display_name,
      username: group.admin.username,
      email: group.admin.email,
      avatar: {
        src: group.admin.avatar_url || "/avatar-placeholder.png",
        width: 32,
        height: 32,
      },
      role: group.admin.user_type,
    },
    wallId: group.wall_id,
  };
}
