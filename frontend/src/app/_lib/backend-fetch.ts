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
      headers: {
        "content-type": "application/json",
        ...options.headers,
      },
      credentials: "include",
    });
  }

  return fetch(`${process.env.NEXT_PUBLIC_BACKEND_URL}${path}`, {
    ...options,
    headers: {
      "content-type": "application/json",
      ...options.headers,
      cookie: `jwt=${session.value}`,
    },
    credentials: "include",
  });
}

export async function checkResponse(
  response: Response,
  opts: { passError: true } | { customError?: string } = {}
) {
  if (!response.ok) {
    try {
      const data = await response.json();

      const error =
        typeof data.error === "string"
          ? data.error
          : JSON.stringify(data.error, null, 2);
      throw new Error(error);
    } catch (error) {
      if ("passError" in opts) {
        throw error;
      }

      throw new Error(opts.customError || "An unknown error has occurred.");
    }
  }

  return response;
}
