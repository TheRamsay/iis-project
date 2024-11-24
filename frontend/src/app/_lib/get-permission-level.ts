import type { schema } from "./db";

type Role = (typeof schema)["userType"]["enumValues"][number];

export function isMinRegular(role?: Role) {
  switch (role) {
    case "administrator":
      return true;
    case "moderator":
      return true;
    case "regular":
      return true;
    default:
      return false;
  }
}

export function isMinModerator(role?: Role) {
  switch (role) {
    case "administrator":
      return true;
    case "moderator":
      return true;
    default:
      return false;
  }
}

export function isMinAdministrator(role?: Role) {
  switch (role) {
    case "administrator":
      return true;
    default:
      return false;
  }
}
