import bodies from "$lib/bodies.json";
import items from "$lib/items.json";

export type Item = {
  id: number;
  name: string;
  nameToken: string;
  displayName: string;
  icon: string;
  helper: boolean;
};

export type Body = {
  name: string;
  nameToken: string;
  displayName: string;
  icon: string;
};

export const ITEMS = Object.fromEntries(items.map((item: Item) => [item.id, item]));
export const BODIES = Object.fromEntries(bodies.map((body: Body) => [body.name, body]));
