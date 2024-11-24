export type User = {
  id: string;
  username: string;
  avatar: {
    src: string;
    width: number;
    height: number;
  };
};

export type Role = "regular" | "administrator" | "moderator";
