import { dbCredentials } from "@/app/_lib/db";
import { defineConfig } from "drizzle-kit";

export default defineConfig({
	dialect: "postgresql",

	dbCredentials,
});
