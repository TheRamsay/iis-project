// biome-ignore lint/suspicious/noExplicitAny: <explanation>
export function extractError(error: any): string {
  let data = error;
  try {
    data = JSON.parse(error);
  } catch {}

  // biome-ignore lint/suspicious/noExplicitAny: <explanation>
  return (Object.values(data) as any)[0][0].message as string;
}
