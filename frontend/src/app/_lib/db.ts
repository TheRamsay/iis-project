import type { Config } from "drizzle-kit";
import { drizzle } from "drizzle-orm/connect";

import * as schema from "../../../drizzle/schema";

const dbCredentials = {
	host: "localhost",
	port: 5432,
	database: "fitstagram",
	user: "koteseni",
	password: "koteseni",
	ssl: false,
};

const db = drizzle("node-postgres", {
	connection: dbCredentials,
});

export { db, schema, dbCredentials };
