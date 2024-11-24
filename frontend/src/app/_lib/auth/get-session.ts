import { cookies } from "next/headers";
import type { Session } from "./types";
import { BACKEND_URL } from "../constants";

export async function getSession(): Promise<Session | null> {
  const cookiez = cookies();
  const session = cookiez.get("jwt");

  if (!session) {
    return null;
  }

  const response = await fetch(`${BACKEND_URL}/api/users/me`, {
    headers: {
      cookie: `jwt=${session.value}`,
    },
    credentials: "include",
  });

  if (!response.ok) {
    return null;
  }

  return {
    userId: "1",
    username: "fitstagram",
    avatar: {
      src: "https://avatars.githubusercontent.com/u/7655549?v=4",
      width: 128,
      height: 128,
    },
    role: "administrator",
    expires: Date.now() + 1000 * 60 * 60 * 24,
  };
}
