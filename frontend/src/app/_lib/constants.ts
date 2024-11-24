if (!process.env.NEXT_PUBLIC_BACKEND_URL) {
  throw new Error("NEXT_PUBLIC_BACKEND_URL is not set");
}

export const BACKEND_URL = process.env.NEXT_PUBLIC_BACKEND_URL;
