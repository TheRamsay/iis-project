import { NextResponse } from "next/server";

export const dynamic = "force-dynamic";

export function GET() {
  const response = new NextResponse(null, { status: 303 });
  response.headers.set("Location", "/");
  response.cookies.delete("jwt");
  return response;
}
