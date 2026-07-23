import type { Property } from "$lib";

export const defaultProperties: Record<string, Property> = {
  id: {
    enabled: true,
    order: 4,
    name: "ID",
    category: "Meta",
    filter: [],
  },
  player: {
    enabled: true,
    order: 3,
    name: "Player",
    category: "Meta",
    filter: [],
  },
  uploadTime: {
    enabled: false,
    order: 8,
    name: "Upload Time",
    category: "Meta",
    filter: [],
  },

  // run info
  survivor: {
    enabled: true,
    order: 1,
    name: "Survivor",
    category: "Run",
    filter: [],
  },
  startTime: {
    enabled: false,
    order: 7,
    name: "Start Time",
    category: "Run",
    filter: [],
  },
  ending: {
    enabled: true,
    order: 0,
    name: "Ending",
    category: "Run",
    filter: [],
  },
  difficulty: {
    enabled: true,
    order: 2,
    name: "Difficulty",
    category: "Run",
    filter: [],
  },
  timeAliveSeconds: {
    enabled: false,
    order: 9,
    name: "Time Alive",
    category: "Run",
    filter: [],
  },
  artifacts: {
    enabled: false,
    order: 10,
    name: "Artifacts",
    category: "Run",
    filter: [],
  },
  stagesCompleted: {
    enabled: false,
    order: 11,
    name: "Stages",
    category: "Run",
    filter: [],
  },
  score: {
    enabled: false,
    order: 6,
    name: "Score",
    category: "Run",
    filter: [],
  },

  // items
  itemsCollected: {
    enabled: false,
    order: 5,
    name: "Items",
    category: "Pickups",
    filter: [],
  },

  // drones
  dronesPurchased: {
    enabled: false,
    order: 12,
    name: "Drones",
    category: "Pickups",
    filter: [],
  },
  turretsPurchased: {
    enabled: false,
    order: 13,
    name: "Turrets",
    category: "Pickups",
    filter: [],
  },

  // combat
  kills: {
    enabled: false,
    order: 14,
    name: "Kills",
    category: "Combat",
    filter: [],
  },
  eliteKills: {
    enabled: false,
    order: 15,
    name: "Elite Kills",
    category: "Combat",
    filter: [],
  },
  minionKills: {
    enabled: false,
    order: 16,
    name: "Minion Kills",
    category: "Combat",
    filter: [],
  },
  deaths: {
    enabled: false,
    order: 17,
    name: "Deaths",
    category: "Combat",
    filter: [],
  },

  // damage
  damageDealt: {
    enabled: false,
    order: 18,
    name: "Damage Dealt",
    category: "Combat",
    filter: [],
  },
  minionDamageDealt: {
    enabled: false,
    order: 19,
    name: "Minion Damage Dealt",
    category: "Combat",
    filter: [],
  },
  damageTaken: {
    enabled: false,
    order: 20,
    name: "Damage Taken",
    category: "Combat",
    filter: [],
  },
  highestDamageDealt: {
    enabled: false,
    order: 21,
    name: "Highest Damage Dealt",
    category: "Combat",
    filter: [],
  },

  // healing
  healingRecieved: {
    enabled: false,
    order: 22,
    name: "Healing Recieved",
    category: "Combat",
    filter: [],
  },

  // progression
  highestLevel: {
    enabled: false,
    order: 23,
    name: "Highest Level",
    category: "Progression",
    filter: [],
  },
  goldCollected: {
    enabled: false,
    order: 24,
    name: "Gold Collected",
    category: "Progression",
    filter: [],
  },
  purchases: {
    enabled: false,
    order: 25,
    name: "Purchases",
    category: "Progression",
    filter: [],
  },
  goldPurchases: {
    enabled: false,
    order: 26,
    name: "Gold Purchases",
    category: "Progression",
    filter: [],
  },
  bloodPurchases: {
    enabled: false,
    order: 27,
    name: "Blood Purchases",
    category: "Progression",
    filter: [],
  },
  lunarPurchases: {
    enabled: false,
    order: 28,
    name: "Lunar Purchases",
    category: "Progression",
    filter: [],
  },

  // movement
  distanceTraveled: {
    enabled: false,
    order: 29,
    name: "Distance Traveled",
    category: "Movement",
    filter: [],
  },
};
