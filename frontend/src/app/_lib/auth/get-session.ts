import { cookies } from "next/headers";
import type { Session } from "./types";

export async function getSession(): Promise<Session | null> {
  const cookiez = cookies();
  const session = cookiez.get("session");

  // TODO: endpoint

  // if (!session) {
  // 	return null
  // }

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
