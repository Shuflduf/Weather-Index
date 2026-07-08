// place files you want to import through the `$lib` alias in this folder.

export type SortMode = "ASC" | "DESC";
export type Property = {
  enabled: boolean;
  order: number;
  name: string;
  category: string;
};

export type PlayerInfo = {
  id: string;
  image?: string;
  username?: string;
  display_username?: string;
  about_me?: string;
  region?: string;
};
