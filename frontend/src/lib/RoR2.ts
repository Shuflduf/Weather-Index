import bodies from "$lib/bodies.json";
import difficulties from "$lib/difficulties.json";
import endings from "$lib/endings.json";
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

export type Ending = {
  name: string;
  nameToken: string;
  endingMessage: string;
  displayName: string;
  isWin: boolean;
  icon: string;
  colorFg: string;
  colorBg: string;
};

export type Difficulty = {
  nameToken: string;
  displayName: string;
  icon: string;
};

export const ITEMS = Object.fromEntries(items.map((item: Item) => [item.id, item]));
export const BODIES = Object.fromEntries(bodies.map((body: Body) => [body.name, body]));
export const ENDINGS = Object.fromEntries(endings.map((ending: Ending) => [ending.name, ending]));
export const DIFFICULTIES = Object.fromEntries(
  difficulties.map((difficulty: Difficulty) => [difficulty.nameToken, difficulty]),
);
