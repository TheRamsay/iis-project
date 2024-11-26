import { cookies } from "next/headers";
import type { Session } from "./types";
import { BACKEND_URL } from "../constants";
import { backendFetch } from "../backend-fetch";
import { redirect } from "next/navigation";

export async function getSession(): Promise<Session | null> {
  const cookiez = cookies();
  const session = cookiez.get("jwt");

  if (!session) {
    return null;
  }

  const response = await backendFetch("/api/users/me");

  if (!response.ok) {
    if (response.status === 401) {
      console.warn("Unauthorized session, redirecting to /logout");
      redirect("/logout");
    }

    throw new Error("Failed to fetch session");
  }

  const data = await response.json();

  return {
    userId: data.id,
    username: data.username,
    avatar: {
      src: data.avatar_url,
    },
    role: data.user_type,
  };
}
