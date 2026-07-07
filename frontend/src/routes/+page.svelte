<script lang="ts">
  import { onMount, tick } from "svelte";
  import { BODIES, DIFFICULTIES, ENDINGS, type Item } from "$lib/RoR2";
  import UserDisplay from "$lib/UserDisplay.svelte";
  import TableBlock from "./TableBlock.svelte";
  import ArtifactsDisplay from "$lib/ArtifactsDisplay.svelte";

  type Property = {
    enabled: boolean;
    order: number;
    name: string;
    category: string;
  };

  let properties: Record<string, Property> = $state({
    id: { enabled: true, order: 0, name: "ID", category: "Meta" },
    player: { enabled: true, order: 1, name: "Player", category: "Meta" },
    uploadTime: {
      enabled: false,
      order: 8,
      name: "Upload Time",
      category: "Meta",
    },

    // run info
    survivor: { enabled: true, order: 2, name: "Survivor", category: "Run" },
    startTime: {
      enabled: false,
      order: 7,
      name: "Start Time",
      category: "Run",
    },
    ending: { enabled: true, order: 3, name: "Ending", category: "Run" },
    difficulty: {
      enabled: true,
      order: 4,
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
      name: "Stages Completed",
      category: "Run",
    },
    score: { enabled: true, order: 6, name: "Score", category: "Run" },

    // items
    itemsCollected: {
      enabled: true,
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
    kills: { enabled: false, order: 14, name: "Kills", category: "Combat" },
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
    deaths: { enabled: false, order: 17, name: "Deaths", category: "Combat" },

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
    goldSpent: {
      enabled: false,
      order: 25,
      name: "Gold Spent",
      category: "Progression",
    },
    lunarCoinsSpent: {
      enabled: false,
      order: 26,
      name: "Lunar Coins Spent",
      category: "Progression",
    },
    purchases: {
      enabled: false,
      order: 27,
      name: "Purchases",
      category: "Progression",
    },
    bloodPurchases: {
      enabled: false,
      order: 28,
      name: "Blood Purchases",
      category: "Progression",
    },

    // movement
    distanceTraveledMetres: {
      enabled: false,
      order: 29,
      name: "Distance Traveled",
      category: "Movement",
    },
  });
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
  onMount(async () => {
    runPromise = fetch("/api/runs").then((r) => r.json());
    console.log(propsByCategory);
  });

  function startDrag(e: PointerEvent, id: string) {
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
        <h1 class="text-xl font-bold tracking-tighter">{category}</h1>
        {#each props as prop}
          <div class="flex flex-row justify-between">
            <span class="">{prop.name}</span>
            <input type="checkbox" bind:checked={prop.enabled} />
          </div>
        {/each}
      </div>
    {/each}
  </div>
</div>

<div class="w-full p-8 text-primary">
  {#await runPromise}
    <span>loading</span>
  {:then runs}
    <button
      popovertarget="visible-properties"
      class=" cursor-pointer bg-default hover:bg-hover active:bg-active p-2 font-mono mb-4 border"
      style="anchor-name: --visible-properties;"
    >
      Visible Properties
    </button>

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
          onpointerdown={(e) => startDrag(e, id)}
          onkeydown={null}
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

      {#if properties.itemsCollected.enabled}
        {@render propHeader("itemsCollected")}
        {#each runs as run, idx}
          <TableBlock order={properties.itemsCollected.order} {idx}>
            {run.items_collected}
          </TableBlock>
        {/each}
      {/if}
      {#if properties.score.enabled}
        {@render propHeader("score")}
        {#each runs as run, idx}
          <TableBlock order={properties.score.order} {idx}>
            {run.score}
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
    </div>
  {/await}
</div>
