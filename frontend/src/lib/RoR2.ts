import artifacts from "../../../data/artifacts.json";
import bodies from "../../../data/bodies.json";
import difficulties from "../../../data/difficulties.json";
import endings from "../../../data/endings.json";
import items from "../../../data/items.json";
import scoring_table from "../../../data/scoring.json";
import tiers from "../../../data/tiers.json";
import environments from "../../../data/environments.json";
import equipment from "../../../data/equipment.json";
import skills from "../../../data/skills.json";

type ItemEvent = {
  id: number;
  count: number;
  time: number;
};
type EquipmentEvent = {
  id: number;
  time: number;
};

export type StageInteractable = {
  name: string;
  time?: number;
  item: number;
};

export type RunReportWithUser = {
  userImage: string;
  userUsername: string;
  userDisplayUsername: string;
  id: number;
  userId: string;
  uploadTime: Date;

  // run info
  survivor: string;
  skills: number[];
  startTime: Date;
  ending: string;
  difficulty: string;
  timeAliveSeconds: number;
  artifacts: string[];
  stagesCompleted: number;
  stageHistory: { name: string; interactables: StageInteractable[] }[];
  score: number;

  // items
  items: Record<string, number>;
  equipment?: number;
  itemsCollected: number;
  itemHistory: ItemEvent[];
  equipmentHistory: EquipmentEvent[];

  // drones
  dronesPurchased: number;
  turretsPurchased: number;

  // combat
  kills: number;
  eliteKills: number;
  minionKills: number;
  deaths: number;

  // damage
  damageDealt: number;
  minionDamageDealt: number;
  damageTaken: number;
  highestDamageDealt: number;

  // healing
  healingRecieved: number;

  // progression
  highestLevel: number;
  goldCollected: number;
  purchases: number;
  goldPurchases: number;
  bloodPurchases: number;
  lunarPurchases: number;

  // movement
  distanceTraveled: number;
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
  survivorColor?: string;
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
  color: string;
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

export type Environment = {
  name: string;
  nameToken: string;
  displayName: string;
  icon: string;
};

export type Equipment = {
  id: number;
  name: string;
  nameToken: string;
  displayName: string;
  icon: string;
};

export type Skill = {
  id: number;
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
export const ENVIRONMENTS = Object.fromEntries(
  environments.map((environment: Environment) => [
    environment.name,
    environment,
  ]),
);
export const EQUIPMENTS = Object.fromEntries(
  equipment.map((equipment: Equipment) => [equipment.id, equipment]),
);
export const SKILLS = Object.fromEntries(
  skills.map((skill: Skill) => [skill.id, skill]),
);

export const ORDERED_SURVIVORS: string[] = [
  "CommandoBody",
  "HuntressBody",
  "BanditBody",
  "ToolbotBody",
  "EngiBody",
  "MageBody",
  "MercBody",
  "TreebotBody",
  "LoaderBody",
  "CrocoBody",
  "CaptainBody",
  "RailgunnerBody",
  "VoidSurvivorBody",
  "SeekerBody",
  "FalseSonBody",
  "ChefBody",
  "DroneTechBody",
  "DrifterBody",
];

export const ORDERED_DIFFICULTIES: string[] = [
  "DIFFICULTY_EASY_NAME",
  "DIFFICULTY_NORMAL_NAME",
  "DIFFICULTY_HARD_NAME",
  "ECLIPSE_1_NAME",
  "ECLIPSE_2_NAME",
  "ECLIPSE_3_NAME",
  "ECLIPSE_4_NAME",
  "ECLIPSE_5_NAME",
  "ECLIPSE_6_NAME",
  "ECLIPSE_7_NAME",
  "ECLIPSE_8_NAME",
];

export const ORDERED_ENDINGS: string[] = [
  "StandardLoss",
  "EscapeSequenceFailed",
  "PrismaticTrialEnding",
  "VoidEnding",
  "DecompileEnding",
  "RebirthEndingDef",
  "ObliterationEnding",
  "LimboEnding",
  "MainEnding",
];

export function countRealItems(items: Record<string, number>): number {
  return Object.entries(items)
    .filter(([id, _]) => !(ITEMS[Number(id)]?.helper ?? false))
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
  return !(ITEMS[id]?.helper ?? false);
}

function tierSort(id: number): number {
  const item = ITEMS[id];
  return item?.tier && TIERS[item.tier] ? TIERS[item.tier].sort : -Infinity;
}

function sortByTier(
  [id_1, _count_1]: [number, number],
  [id_2, _count_2]: [number, number],
): number {
  return tierSort(id_2) - tierSort(id_1);
}
function sortByCount(
  [_id_1, count_1]: [number, number],
  [_id_2, count_2]: [number, number],
): number {
  return count_2 - count_1;
}

export function formatSeconds(seconds: number): string {
  const date = new Date(0);
  date.setSeconds(seconds);
  const result = date.toISOString().slice(11, 19);
  return result;
}

export function formatBig(big: number): string {
  return Math.round(big)
    .toString()
    .replace(/\B(?=(\d{3})+(?!\d))/g, ",");
}

export function sortStageInteractables(
  a: StageInteractable,
  b: StageInteractable,
): number {
  return (a.time ?? Infinity) - (b.time ?? Infinity) || 0.0;
}
