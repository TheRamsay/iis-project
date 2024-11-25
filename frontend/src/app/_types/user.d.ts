export type User = {
  id: string;
  username: string;
  avatar: {
    src: string | undefined;
  };
};

export type Role = "regular" | "administrator" | "moderator";
