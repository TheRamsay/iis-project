async function getSession() {
  if (typeof window !== "undefined") {
    return null;
  }

  const cookies = (await import("next/headers")).cookies;
  const cookiez = cookies();
  const session = cookiez.get("jwt");

  return session;
}

export async function backendFetch(path: string, options: RequestInit = {}) {
  const session = await getSession();

  if (!session) {
    return fetch(`${process.env.NEXT_PUBLIC_BACKEND_URL}${path}`, {
      ...options,
      credentials: "include",
    });
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

export async function checkResponse(response: Response, customError?: string) {
  if (!response.ok) {
    try {
      const data = await response.json();
      throw new Error(data.error);
    } catch (error) {
      throw new Error(customError || "An unknown error has occurred.");
    }
  }

  return response;
}
