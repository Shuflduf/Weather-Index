<script lang="ts">
  import {
    ArrowUpWideNarrow,
    ArrowDownWideNarrow,
    Check,
  } from "@lucide/svelte";
  import { onMount } from "svelte";
  import Table from "./Table.svelte";
  import type { Property, SortMode } from "$lib";
  import ContextMenu from "./ContextMenu.svelte";

  const TABLE_STORAGE_KEY = "table-properties";
  const SORT_STORAGE_KEY = "sort-property";

  let loaded = $state(false);
  let properties: Record<string, Property> = $state({
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
    timeAlive: {
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
  });
  let defaultProperties: Record<string, Property> = {};
  let propsByCategory = $derived(
    Object.values(properties).reduce(
      (acc, prop) => {
        (acc[prop.category] ??= []).push(prop);
        return acc;
      },
      {} as Record<string, Property[]>,
    ),
  );
  let sortProperty: { by: string; sort: SortMode } = $state({
    by: "id",
    sort: "DESC",
  });
  let contextMenu: any = $state();

  $effect(() => {
    if (!loaded) return;

    const toSave: Record<
      string,
      { enabled: boolean; order: number; filter: string[] }
    > = {};
    for (const [key, prop] of Object.entries(properties)) {
      toSave[key] = {
        order: prop.order,
        enabled: prop.enabled,
        filter: prop.filter,
      };
    }
    localStorage.setItem(TABLE_STORAGE_KEY, JSON.stringify(toSave));
  });

  let runPromise: Promise<any[]> = $state(new Promise(() => {}));
  onMount(() => {
    defaultProperties = $state.snapshot(properties);
    const savedSort = localStorage.getItem(SORT_STORAGE_KEY);
    if (savedSort) {
      const parsed = JSON.parse(savedSort);
      sortProperty = parsed;
    }

    const saved = localStorage.getItem(TABLE_STORAGE_KEY);
    if (saved) {
      const parsed = JSON.parse(saved);
      for (const [key, prop] of Object.entries(parsed) as [
        string,
        { order: number; enabled: boolean; filter: string[] },
      ][]) {
        if (properties[key]) {
          properties[key].enabled = prop.enabled;
          properties[key].order = prop.order;
          properties[key].filter = prop.filter;
        }
      }
    }

    fetchRuns();
    loaded = true;
  });

  function fetchRuns() {
    const filters = Object.fromEntries(
      Object.entries(properties)
        .filter(([_, data]) => {
          return data.filter && data.filter.length > 0;
        })
        .map(([prop, data]) => [prop, data.filter]),
    );
    runPromise = fetch(
      "/api/runs?" +
        new URLSearchParams({
          filters: JSON.stringify(filters),
          ...sortProperty,
        }),
    ).then((r) => r.json());
  }

  function setSort(sort: SortMode, by: string) {
    sortProperty.by = by;
    sortProperty.sort = sort;

    localStorage.setItem(SORT_STORAGE_KEY, JSON.stringify(sortProperty));

    fetchRuns();
  }

  function setFilter(prop: string, filter: string[]) {
    properties[prop].filter = filter;

    fetchRuns();
  }

  function resetProperties() {
    properties = structuredClone(defaultProperties);
  }
</script>

<ContextMenu
  bind:this={contextMenu}
  {properties}
  {sortProperty}
  {setSort}
  {setFilter}
/>

<div
  id="visible-properties"
  popover
  class="fixed bg-bg-secondary border text-primary p-2"
  style="position-anchor: --visible-properties; position-area: bottom span-right;"
>
  <div class="flex flex-row gap-8">
    {#each Object.entries(propsByCategory) as [category, props] (category)}
      <div class="w-40 flex flex-col gap-1">
        <h2 class="text-xl font-bold tracking-tighter">{category}</h2>
        {#each props as prop (prop.name)}
          <div class="flex flex-row justify-between relative items-center">
            <span>{prop.name}</span>
            <input type="checkbox" bind:checked={prop.enabled} class="" />
            <Check
              class="absolute right-0 w-4 h-4 pointer-events-none"
              strokeWidth="2"
            />
          </div>
        {/each}
      </div>
    {/each}
  </div>
  <button
    class="p-2 bg-default hover:bg-hover active:bg-active cursor-pointer font-mono transition-colors"
    onclick={resetProperties}
  >
    Reset
  </button>
</div>

<div class="flex flex-row gap-4">
  <button
    popovertarget="visible-properties"
    class="cursor-pointer bg-default hover:bg-hover active:bg-active p-2 font-mono mb-4 border"
    style="anchor-name: --visible-properties;"
  >
    Visible Properties
  </button>
  <span class="font-mono p-2 text-secondary flex flex-row gap-2">
    <span>
      Sorting By: {properties[sortProperty.by].name}
    </span>
    {#if sortProperty.sort == "ASC"}
      <ArrowUpWideNarrow />
    {:else}
      <ArrowDownWideNarrow />
    {/if}
  </span>
</div>
{#await runPromise}
  <span>loading</span>
{:then runs}
  <Table {properties} {runs} openContextMenu={contextMenu.open} />
{/await}
