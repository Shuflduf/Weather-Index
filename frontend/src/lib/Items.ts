import items from "$lib/items.json";

export type Item = {
  id: number;
  name: string;
  nameToken: string;
  displayName: string;
  icon: string;
  helper: boolean;
};

export const ITEMS = Object.fromEntries(items.map((item: Item) => [item.id, item]));
