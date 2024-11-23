import { NextResponse } from "next/server";

export const dynamic = "force-dynamic";

export function GET(request: Request) {
  // TODO: invalidate jwt token

  // TODO: endpoint

  const response = new NextResponse(null, { status: 303 });
  response.headers.set("Location", "/");
  response.cookies.set("session", "", { expires: new Date(0) });
  return response;
}
