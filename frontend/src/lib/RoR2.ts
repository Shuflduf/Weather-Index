import artifacts from "$lib/artifacts.json";
import bodies from "$lib/bodies.json";
import difficulties from "$lib/difficulties.json";
import endings from "$lib/endings.json";
import items from "$lib/items.json";
import scoring_table from "$lib/scoring.json";
import tiers from "$lib/tiers.json";

export type RunReportWithUser = {
  user_image: string;
  user_username: string;
  id: number;
  user_id: string;
  upload_time: Date;

  // run info
  survivor: string;
  start_time: Date;
  ending: string;
  difficulty: string;
  time_alive_seconds: number;
  artifacts: string[];
  stages_completed: number;
  score: number;

  // items
  items: Record<string, number>;
  items_collected: number;

  // drones
  drones_purchased: number;
  turrets_purchased: number;

  // combat
  kills: number;
  elite_kills: number;
  minion_kills: number;
  deaths: number;

  // damage
  damage_dealt: number;
  minion_damage_dealt: number;
  damage_taken: number;
  highest_damage_dealt: number;

  // healing
  healing_recieved: number;

  // progression
  highest_level: number;
  gold_collected: number;
  gold_spent: number;
  lunar_coins_spent: number;
  purchases: number;
  blood_purchases: number;

  // movement
  distance_traveled_metres: number;
};

export type Item = {
  id: number;
  name: string;
  nameToken: string;
  displayName: string;
  tier: string | null;
  helper: boolean;
  icon: string;
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

export type Tier = {
  name: string;
  sort: number;
};
export type Artifact = {
  name: string;
  nameToken: string;
  displayName: string;
  icon: string;
};

export const ITEMS = Object.fromEntries(
  items.map((item: Item) => [item.id, item]),
);
export const BODIES = Object.fromEntries(
  bodies.map((body: Body) => [body.name, body]),
);
export const ENDINGS = Object.fromEntries(
  endings.map((ending: Ending) => [ending.name, ending]),
);
export const DIFFICULTIES = Object.fromEntries(
  difficulties.map((difficulty: Difficulty) => [
    difficulty.nameToken,
    difficulty,
  ]),
);
export const SCORING_TABLE = scoring_table as ScoringTable;
export const TIERS = Object.fromEntries(
  tiers.map((tier: Tier) => [tier.name, tier]),
);
export const ARTIFACTS = Object.fromEntries(
  artifacts.map((artifact: Artifact) => [artifact.name, artifact]),
);

export function countRealItems(items: Record<string, number>): number {
  return Object.entries(items)
    .filter(([id, _]) => !ITEMS[id].helper)
    .reduce((accum, [_, current]) => accum + current, 0);
}

export function sortItems(items: Record<string, number>): [number, number][] {
  return Object.entries(items)
    .map(([id, count]) => [Number(id), count] as [number, number])
    .filter(removeHelpers)
    .sort(sortByCount)
    .sort(sortByTier);
}

function removeHelpers([id, _count]: [number, number]): boolean {
  return !ITEMS[id].helper;
}

function sortByTier(
  [id_1, _count_1]: [number, number],
  [id_2, _count_2]: [number, number],
): number {
  console.log(id_2);
  return TIERS[ITEMS[id_2].tier!].sort - TIERS[ITEMS[id_1].tier!].sort;
}
function sortByCount(
  [_id_1, count_1]: [number, number],
  [_id_2, count_2]: [number, number],
): number {
  return count_2 - count_1;
}

export function formatSeconds(seconds: number): string {
  const date = new Date();
  date.setSeconds(seconds);
  const result = date.toISOString().slice(11, 19);
  return result;
}

export function formatBig(big: number): string {
  return Math.round(big)
    .toString()
    .replace(/\B(?=(\d{3})+(?!\d))/g, ",");
}
