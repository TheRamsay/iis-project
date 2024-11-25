// biome-ignore lint/suspicious/noExplicitAny: <explanation>
export function extractError(error: any): string {
  console.log(error);

  let data = error;
  try {
    data = JSON.parse(error);
  } catch {}

  console.log(data);

  // biome-ignore lint/suspicious/noExplicitAny: <explanation>
  return (Object.values(data) as any)[0][0].message as string;
}
