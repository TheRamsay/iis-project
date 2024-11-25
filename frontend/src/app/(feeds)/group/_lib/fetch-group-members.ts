import { backendFetch, checkResponse } from "@/app/_lib/backend-fetch";
import type { Group } from "./fetch-groups-by-username";

export async function fetchGroupMembers(groupId: string) {
  const response = await backendFetch(`/api/groups/${groupId}/members`);

  await checkResponse(response);

  // biome-ignore lint/suspicious/noExplicitAny: <explanation>
  const data: any[] = await response.json();

  return data.map((entry) => ({
    id: entry.id as string,
    username: entry.username as string,
  }));
}
