import bodies from "$lib/bodies.json";
import difficulties from "$lib/difficulties.json";
import endings from "$lib/endings.json";
import items from "$lib/items.json";
import scoring_table from "$lib/scoring.json";

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

export type ScoringTable = {
  timeAliveSeconds: number;
  kills: number;
  minionKills: number;
  damageDealt: number;
  minionDamageDealt: number;
  highestDamageDealt: number;
  highestLevel: number;
  goldCollected: number;
  itemsCollected: number;
  stagesCompleted: number;
  purchases: number;
};

export const ITEMS = Object.fromEntries(items.map((item: Item) => [item.id, item]));
export const BODIES = Object.fromEntries(bodies.map((body: Body) => [body.name, body]));
export const ENDINGS = Object.fromEntries(endings.map((ending: Ending) => [ending.name, ending]));
export const DIFFICULTIES = Object.fromEntries(
  difficulties.map((difficulty: Difficulty) => [difficulty.nameToken, difficulty]),
);
export const SCORING_TABLE = scoring_table as ScoringTable;

export function countRealItems(items: Record<string, number>): number {
  return Object.entries(items).filter(([id, _]) => !ITEMS[id].helper).reduce(
    (accum, [_, current]) => accum + current,
    0,
  );
}

export function formatSeconds(seconds: number): string {
  const date = new Date(null);
  date.setSeconds(seconds);
  const result = date.toISOString().slice(11, 19);
  return result;
}

export function formatBig(big: number): string {
  return big.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ",");
}
