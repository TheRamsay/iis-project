import { cookies } from "next/headers";

export function backendFetch(path: string, options: RequestInit = {}) {
  const cookiez = cookies();
  const session = cookiez.get("jwt");

  if (!session) {
    return fetch(`${process.env.NEXT_PUBLIC_BACKEND_URL}${path}`, options);
  }

  return fetch(`${process.env.NEXT_PUBLIC_BACKEND_URL}${path}`, {
    ...options,
    headers: {
      ...options.headers,
      cookie: `jwt=${session.value}`,
    },
    credentials: "include",
  });
}
