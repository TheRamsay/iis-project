import { backendFetch, checkResponse } from "@/app/_lib/backend-fetch";
import type { GroupFollowState } from "../[groupname]/(group)/_ui/group-header/group-header-follow";

export type GroupStatus = {
  status: GroupFollowState;
};

export async function fetchGroupStatus({
  groupId,
}: {
  groupId: string;
}): Promise<GroupStatus> {
  const response = await backendFetch(`/api/groups/${groupId}/status`);

  await checkResponse(response);

  const data = await response.json();

  return {
    status: data.status,
  };
}
