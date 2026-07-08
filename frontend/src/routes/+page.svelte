<script lang="ts">
  import { ArrowUpWideNarrow, ArrowDownWideNarrow } from "@lucide/svelte";
  import { onMount, tick } from "svelte";
  import {
    BODIES,
    DIFFICULTIES,
    ENDINGS,
    formatBig,
    formatSeconds,
    type Item,
  } from "$lib/RoR2";
  import UserDisplay from "$lib/UserDisplay.svelte";
  import TableBlock from "./TableBlock.svelte";
  import ArtifactsDisplay from "$lib/ArtifactsDisplay.svelte";

  const LEFT_MOUSE_BUTTON = 0;
  const TABLE_STORAGE_KEY = "table-properties";
  const SORT_STORAGE_KEY = "sort-property";

  type SortMode = "ASC" | "DESC";

  type Property = {
    enabled: boolean;
    order: number;
    name: string;
    category: string;
  };

  let loaded = $state(false);
  let properties: Record<string, Property> = $state({
    id: {
      enabled: true,
      order: 4,
      name: "ID",
      category: "Meta",
    },
    player: {
      enabled: true,
      order: 3,
      name: "Player",
      category: "Meta",
    },
    uploadTime: {
      enabled: false,
      order: 8,
      name: "Upload Time",
      category: "Meta",
    },

    // run info
    survivor: {
      enabled: true,
      order: 1,
      name: "Survivor",
      category: "Run",
    },
    startTime: {
      enabled: false,
      order: 7,
      name: "Start Time",
      category: "Run",
    },
    ending: {
      enabled: true,
      order: 0,
      name: "Ending",
      category: "Run",
    },
    difficulty: {
      enabled: true,
      order: 2,
      name: "Difficulty",
      category: "Run",
    },
    timeAlive: {
      enabled: false,
      order: 9,
      name: "Time Alive",
      category: "Run",
    },
    artifacts: {
      enabled: false,
      order: 10,
      name: "Artifacts",
      category: "Run",
    },
    stagesCompleted: {
      enabled: false,
      order: 11,
      name: "Stages",
      category: "Run",
    },
    score: {
      enabled: false,
      order: 6,
      name: "Score",
      category: "Run",
    },

    // items
    itemsCollected: {
      enabled: false,
      order: 5,
      name: "Items",
      category: "Pickups",
    },

    // drones
    dronesPurchased: {
      enabled: false,
      order: 12,
      name: "Drones",
      category: "Pickups",
    },
    turretsPurchased: {
      enabled: false,
      order: 13,
      name: "Turrets",
      category: "Pickups",
    },

    // combat
    kills: {
      enabled: false,
      order: 14,
      name: "Kills",
      category: "Combat",
    },
    eliteKills: {
      enabled: false,
      order: 15,
      name: "Elite Kills",
      category: "Combat",
    },
    minionKills: {
      enabled: false,
      order: 16,
      name: "Minion Kills",
      category: "Combat",
    },
    deaths: {
      enabled: false,
      order: 17,
      name: "Deaths",
      category: "Combat",
    },

    // damage
    damageDealt: {
      enabled: false,
      order: 18,
      name: "Damage Dealt",
      category: "Combat",
    },
    minionDamageDealt: {
      enabled: false,
      order: 19,
      name: "Minion Damage Dealt",
      category: "Combat",
    },
    damageTaken: {
      enabled: false,
      order: 20,
      name: "Damage Taken",
      category: "Combat",
    },
    highestDamageDealt: {
      enabled: false,
      order: 21,
      name: "Highest Damage Dealt",
      category: "Combat",
    },

    // healing
    healingRecieved: {
      enabled: false,
      order: 22,
      name: "Healing Recieved",
      category: "Combat",
    },

    // progression
    highestLevel: {
      enabled: false,
      order: 23,
      name: "Highest Level",
      category: "Progression",
    },
    goldCollected: {
      enabled: false,
      order: 24,
      name: "Gold Collected",
      category: "Progression",
    },
    purchases: {
      enabled: false,
      order: 25,
      name: "Purchases",
      category: "Progression",
    },
    goldPurchases: {
      enabled: false,
      order: 26,
      name: "Gold Purchases",
      category: "Progression",
    },
    bloodPurchases: {
      enabled: false,
      order: 27,
      name: "Blood Purchases",
      category: "Progression",
    },
    lunarPurchases: {
      enabled: false,
      order: 28,
      name: "Lunar Purchases",
      category: "Progression",
    },

    // movement
    distanceTraveled: {
      enabled: false,
      order: 29,
      name: "Distance Traveled",
      category: "Movement",
    },
  });
  let defaultProperties: Record<string, Property> = {};
  let columnCount = $derived(
    Object.values(properties).filter((prop) => prop.enabled).length,
  );
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

  $effect(() => {
    if (!loaded) return;

    const toSave: Record<string, { enabled: boolean; order: number }> = {};
    for (const [key, prop] of Object.entries(properties)) {
      toSave[key] = { order: prop.order, enabled: prop.enabled };
    }
    localStorage.setItem(TABLE_STORAGE_KEY, JSON.stringify(toSave));
  });

  let drag: {
    dragging: boolean;
    elem: HTMLElement | null;
    id: string;
    startX: number;
  } = $state({
    dragging: false,
    elem: null,
    id: "",
    startX: 0,
  });

  let runPromise: Promise<any[]> = $state(new Promise(() => {}));
  onMount(() => {
    defaultProperties = $state.snapshot(properties);
    const savedSort = localStorage.getItem(SORT_STORAGE_KEY);
    if (savedSort) {
      const parsed = JSON.parse(savedSort);
      sortProperty = parsed;
    }

    fetchRuns();

    const saved = localStorage.getItem(TABLE_STORAGE_KEY);
    if (saved) {
      const parsed = JSON.parse(saved);
      for (const [key, prop] of Object.entries(parsed) as [
        string,
        { order: number; enabled: boolean },
      ][]) {
        if (properties[key]) {
          properties[key].enabled = prop.enabled;
          properties[key].order = prop.order;
        }
      }
    }
    loaded = true;
  });

  function fetchRuns() {
    runPromise = fetch("/api/runs?" + new URLSearchParams(sortProperty)).then(
      (r) => r.json(),
    );
  }

  function startDrag(e: PointerEvent, id: string) {
    if (e.button != LEFT_MOUSE_BUTTON) return;
    drag.dragging = true;
    drag.id = id;
    drag.elem = document.getElementById(id);
    drag.startX = e.clientX - drag.elem!.getBoundingClientRect().left;

    window.addEventListener("pointermove", onDrag);
    window.addEventListener("pointerup", endDrag);
  }

  async function onDrag(e: PointerEvent) {
    if (!drag.dragging || !drag.elem) return;

    drag.elem.style.transform = "none";
    const normalWidth = drag.elem.getBoundingClientRect().width;
    const normalLeft = drag.elem.getBoundingClientRect().left;
    const normalCenter = normalLeft + normalWidth / 2.0;
    const dx = e.clientX - drag.startX - normalLeft;
    const elemCenter = normalCenter + dx;
    drag.elem.style.transform = `translateX(${dx}px)`;
    drag.elem.style.zIndex = "10";

    const headers = [
      ...document.querySelectorAll("[data-col-header]"),
    ] as HTMLElement[];
    headers.sort((a, b) => properties[a.id].order - properties[b.id].order);
    const curIdx = headers.findIndex((h) => h.id == drag.id);
    if (curIdx == -1) return;

    let swapped = false;

    if (curIdx > 0) {
      const leftRect = headers[curIdx - 1].getBoundingClientRect();
      const leftCenter = leftRect.left + leftRect.width / 2.0;
      const diff = (leftCenter + normalCenter) / 2.0;
      if (elemCenter < diff) {
        const temp = properties[drag.id].order;
        properties[drag.id].order = properties[headers[curIdx - 1].id].order;
        properties[headers[curIdx - 1].id].order = temp;
        drag.elem.style.transform = `translateX(${dx}px)`;
        swapped = true;
      }
    }
    if (curIdx < headers.length - 1) {
      const rightRect = headers[curIdx + 1].getBoundingClientRect();
      const rightCenter = rightRect.left + rightRect.width / 2.0;
      const diff = (rightCenter + normalCenter) / 2.0;
      if (elemCenter > diff) {
        const temp = properties[drag.id].order;
        properties[drag.id].order = properties[headers[curIdx + 1].id].order;
        properties[headers[curIdx + 1].id].order = temp;
        swapped = true;
      }
    }

    if (swapped) {
      const allCells = [
        ...document.querySelectorAll("[data-col-header], [data-col-cell]"),
      ] as HTMLElement[];
      const oldPositions = new Map<HTMLElement, DOMRect>();
      for (const cell of allCells) {
        if (cell != drag.elem) {
          oldPositions.set(cell, cell.getBoundingClientRect());
        }
      }

      await tick();

      for (const cell of allCells) {
        if (cell == drag.elem) continue;
        const oldRect = oldPositions.get(cell);
        if (!oldRect) continue;
        const newRect = cell.getBoundingClientRect();
        const dx = oldRect.left - newRect.left;
        if (Math.abs(dx) < 1.0) continue;

        cell.style.transform = `translateX(${dx}px)`;
        cell.style.transition = "none";

        requestAnimationFrame(() => {
          cell.style.transform = "";
          cell.style.transition = "";
        });
      }

      drag.elem.style.transform = "none";
      const newLeft = drag.elem.getBoundingClientRect().left;
      drag.elem.style.transform = `translateX(${e.clientX - drag.startX - newLeft}px)`;
    }
  }

  function endDrag(_e: PointerEvent) {
    if (!drag.dragging || !drag.elem) return;

    drag.elem.style.transform = "";
    drag.elem.style.zIndex = "";
    drag.dragging = false;

    window.removeEventListener("pointermove", onDrag);
    window.removeEventListener("pointerup", endDrag);
  }

  let contextMenu: {
    popup?: HTMLElement;
    id: string;
    pos: [number, number];
  } = $state({ id: "", pos: [0.0, 0.0] });

  function openContextMenu(e: MouseEvent, id: string) {
    if (!contextMenu.popup)
      contextMenu.popup = document.getElementById("context-menu")!;

    const RIGHT_MOUSE_BUTTON = 2;
    if (e.button != RIGHT_MOUSE_BUTTON) return;

    if (id == contextMenu.id) {
      contextMenu.popup.togglePopover();
    } else {
      contextMenu.popup.showPopover();
    }

    contextMenu.id = id;
    contextMenu.pos = [e.clientX, e.clientY];
  }

  function setSort(sort: SortMode) {
    sortProperty.by = contextMenu.id;
    sortProperty.sort = sort;

    localStorage.setItem(SORT_STORAGE_KEY, JSON.stringify(sortProperty));

    fetchRuns();
  }

  function resetProperties() {
    console.log("shit");
    properties = structuredClone(defaultProperties);
  }
</script>

<div
  id="visible-properties"
  popover
  class="fixed bg-bg-secondary border text-primary p-2"
  style="position-anchor: --visible-properties; position-area: bottom span-right;"
>
  <div class="flex flex-row gap-8">
    {#each Object.entries(propsByCategory) as [category, props]}
      <div class="w-40">
        <h2 class="text-xl font-bold tracking-tighter">{category}</h2>
        {#each props as prop}
          <div class="flex flex-row justify-between">
            <span>{prop.name}</span>
            <input type="checkbox" bind:checked={prop.enabled} />
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

<div
  role="button"
  tabindex="-1"
  id="context-menu"
  popover
  class="fixed bg-bg-secondary border text-primary p-2 w-40"
  style="left: {contextMenu.pos[0]}px; top: {contextMenu.pos[1]}px;"
  oncontextmenu={(e) => e.preventDefault()}
>
  {#if contextMenu.id}
    <h2 class="text-xl font-bold tracking-tighter">
      {properties[contextMenu.id].name}
    </h2>
    <div class="flex flex-row justify-between">
      <span>Show</span>
      <input
        type="checkbox"
        bind:checked={properties[contextMenu.id].enabled}
      />
    </div>
    {#snippet sortButton(sort: SortMode)}
      <button
        class={`border w-full p-2 cursor-pointer transition-colors flex flex-row gap-4 ${
          sortProperty.by == contextMenu.id && sortProperty.sort == sort
            ? "bg-active"
            : "bg-default hover:bg-hover "
        }`}
        onclick={() => setSort(sort)}
      >
        {#if sort == "ASC"}
          <ArrowUpWideNarrow />
          <span>Ascending</span>
        {:else}
          <ArrowDownWideNarrow />
          <span>Descending</span>
        {/if}
      </button>
    {/snippet}
    {@render sortButton("ASC")}
    {@render sortButton("DESC")}
  {/if}
</div>

<div class="w-full p-8 text-primary">
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
    <div
      class="grid"
      style="grid-template-columns: repeat({columnCount}, auto); user-select: {drag.dragging
        ? 'none'
        : ''};"
    >
      {#snippet propHeader(id: string)}
        <div
          role="button"
          tabindex="-1"
          onpointerdown={(e) => {
            startDrag(e, id);
            openContextMenu(e, id);
          }}
          onkeydown={null}
          oncontextmenu={(e) => e.preventDefault()}
          {id}
          data-col-header
          class="items-center justify-center h-12 active:bg-active hover:bg-hover transition-colors cursor-grab flex"
          style="order: {properties[id].order}; grid-row: 1;"
        >
          <h2 class="text-xl tracking-tight text-center font-bold">
            {properties[id].name}
          </h2>
        </div>
      {/snippet}

      {#snippet basicCol(id: string, runId: string)}
        {#if properties[id].enabled}
          {@render propHeader(id)}
          {#each runs as run, idx}
            <TableBlock order={properties[id].order} {idx}>
              <span>{formatBig(run[runId])}</span>
            </TableBlock>
          {/each}
        {/if}
      {/snippet}

      {#if properties.id.enabled}
        {@render propHeader("id")}
        {#each runs as run, idx}
          <TableBlock order={properties.id.order} {idx}>
            <a href={`/run/${run.id}`}>{run.id}</a>
          </TableBlock>
        {/each}
      {/if}
      {#if properties.player.enabled}
        {@render propHeader("player")}
        {#each runs as run, idx}
          <TableBlock order={properties.player.order} {idx}>
            <UserDisplay
              class="h-12"
              user={{
                username: run.user_username,
                image: run.user_image,
                displayName: null,
              }}
            />
          </TableBlock>
        {/each}
      {/if}
      {#if properties.uploadTime.enabled}
        {@render propHeader("uploadTime")}
        {#each runs as run, idx}
          <TableBlock order={properties.uploadTime.order} {idx}>
            <span
              title={new Date(run.upload_time).toString()}
              class="text-sm text-secondary"
            >
              {new Date(run.upload_time).toLocaleString()}
            </span>
          </TableBlock>
        {/each}
      {/if}

      {#if properties.survivor.enabled}
        {@render propHeader("survivor")}
        {#each runs as run, idx}
          <TableBlock order={properties.survivor.order} {idx}>
            <img
              src={`/bodies/${BODIES[run.survivor].icon}`}
              alt={BODIES[run.survivor].displayName}
              class="h-12 inline mr-2"
            />
            <span class="text-lg">
              {BODIES[run.survivor].displayName}
            </span>
          </TableBlock>
        {/each}
      {/if}
      {#if properties.startTime.enabled}
        {@render propHeader("startTime")}
        {#each runs as run, idx}
          <TableBlock order={properties.startTime.order} {idx}>
            <span
              title={new Date(run.start_time).toString()}
              class="text-sm text-secondary"
            >
              {new Date(run.start_time).toLocaleString()}
            </span>
          </TableBlock>
        {/each}
      {/if}
      {#if properties.ending.enabled}
        {@render propHeader("ending")}
        {#each runs as run, idx}
          <TableBlock
            order={properties.ending.order}
            {idx}
            style="background-color: {ENDINGS[run.ending].colorBg};"
          >
            <img
              src={`/endings/${ENDINGS[run.ending].icon}`}
              alt={run.ending}
              class="h-12 inline mr-2"
            />
            <span class="text-shadow-lg text-lg">
              {ENDINGS[run.ending].displayName}
            </span>
          </TableBlock>
        {/each}
      {/if}
      {#if properties.difficulty.enabled}
        {@render propHeader("difficulty")}
        {#each runs as run, idx}
          <TableBlock order={properties.difficulty.order} {idx}>
            <img
              src={`/difficulties/${DIFFICULTIES[run.difficulty].icon}`}
              alt={run.difficulty}
              class="h-12 inline mr-2"
            />
            <span class="text-lg">
              {DIFFICULTIES[run.difficulty].displayName}
            </span>
          </TableBlock>
        {/each}
      {/if}
      {#if properties.timeAlive.enabled}
        {@render propHeader("timeAlive")}
        {#each runs as run, idx}
          <TableBlock order={properties.timeAlive.order} {idx}>
            <span class="">
              {formatSeconds(run.time_alive_seconds)}
            </span>
          </TableBlock>
        {/each}
      {/if}
      {#if properties.artifacts.enabled}
        {@render propHeader("artifacts")}
        {#each runs as run, idx}
          <TableBlock order={properties.artifacts.order} {idx}>
            <ArtifactsDisplay
              artifacts={run.artifacts}
              class="w-full overflow-x-auto flex-wrap"
            />
          </TableBlock>
        {/each}
      {/if}
      {@render basicCol("stagesCompleted", "stages_completed")}
      {@render basicCol("score", "score")}

      {@render basicCol("itemsCollected", "items_collected")}
      {@render basicCol("dronesPurchased", "drones_purchased")}
      {@render basicCol("turretsPurchased", "turrets_purchased")}

      {@render basicCol("kills", "kills")}
      {@render basicCol("eliteKills", "elite_kills")}
      {@render basicCol("minionKills", "minion_kills")}
      {@render basicCol("deaths", "deaths")}
      {@render basicCol("damageDealt", "damage_dealt")}
      {@render basicCol("minionDamageDealt", "minion_damage_dealt")}
      {@render basicCol("damageTaken", "damage_taken")}
      {@render basicCol("highestDamageDealt", "highest_damage_dealt")}
      {@render basicCol("healingRecieved", "healing_recieved")}

      {@render basicCol("highestLevel", "highest_level")}
      {@render basicCol("goldCollected", "gold_collected")}
      {@render basicCol("purchases", "purchases")}
      {@render basicCol("goldPurchases", "gold_purchases")}
      {@render basicCol("bloodPurchases", "blood_purchases")}
      {@render basicCol("lunarPurchases", "lunar_purchases")}

      {#if properties.distanceTraveled.enabled}
        {@render propHeader("distanceTraveled")}
        {#each runs as run, idx}
          <TableBlock order={properties.distanceTraveled.order} {idx}>
            {formatBig(run.distance_traveled_metres)}

            <span class="text-yellow-200 ml-1">metres</span>
          </TableBlock>
        {/each}
      {/if}
    </div>
  {/await}
</div>
