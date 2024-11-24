import { cookies } from "next/headers";
import type { Session } from "./types";
import { BACKEND_URL } from "../constants";
import { backendFetch } from "../backend-fetch";

export async function getSession(): Promise<Session | null> {
  const cookiez = cookies();
  const session = cookiez.get("jwt");

  if (!session) {
    return null;
  }

  const response = await backendFetch("/api/users/me");

  if (!response.ok) {
    return null;
  }

  const data = await response.json();

  const avatar = data.avatar_url || {
    src: "/avatar-placeholder.png",
    width: 128,
    height: 128,
  };

  return {
    userId: data.id,
    username: data.username,
    avatar: avatar,
    role: data.user_type,
  };
}
