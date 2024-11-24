import { backendFetch, checkResponse } from "./backend-fetch";

export async function uploadImage(data: string) {
  const blob = await fetch(data).then((res) => res.blob());
  const base64 = await blobToBase64(blob);
  console.log(base64);

  const response = await backendFetch("/api/posts/upload_image", {
    method: "POST",
    body: JSON.stringify({
      image: `data:image/jpeg;base64,${base64}`,
    }),
  });

  await checkResponse(response);

  return response.json() as Promise<{ link: string }>;
}

function blobToBase64(blob: Blob) {
  return new Promise((resolve, reject) => {
    // biome-ignore lint/suspicious/noExplicitAny: <explanation>
    const reader: any = new FileReader();
    reader.onloadend = () => resolve(reader.result.split(",")[1]); // Strip the Base64 prefix
    reader.onerror = reject;
    reader.readAsDataURL(blob); // Read the Blob as a Base64 string
  });
}
