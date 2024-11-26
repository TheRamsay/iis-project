import { NextResponse } from "next/server";

export const dynamic = "force-dynamic";

export function GET() {
  const response = new NextResponse(null, { status: 303 });
  response.headers.set("Location", "/");
  response.headers.set("Allow-Credentials", "true");
  response.headers.set("Access-Control-Allow-Origin", "https://iis.lufy.cz");
  response.cookies.delete({ name: "jwt", domain: "iis.lufy.cz" });
  return response;
}
