<script lang="ts">
  import { onMount, tick } from "svelte";
  import { BODIES, DIFFICULTIES, ENDINGS, type Item } from "$lib/RoR2";
  import UserDisplay from "$lib/UserDisplay.svelte";
  import TableBlock from "./TableBlock.svelte";
  import ArtifactsDisplay from "$lib/ArtifactsDisplay.svelte";

  let properties: Record<
    string,
    { enabled: boolean; order: number; name: string }
  > = $state({
    id: { enabled: true, order: 0, name: "ID" },
    player: { enabled: true, order: 1, name: "Player" },
    uploadTime: { enabled: false, order: 8, name: "Upload Time" },

    // run info
    survivor: { enabled: true, order: 2, name: "Survivor" },
    startTime: { enabled: false, order: 7, name: "Start Time" },
    ending: { enabled: true, order: 3, name: "Ending" },
    difficulty: { enabled: true, order: 4, name: "Difficulty" },
    timeAlive: { enabled: false, order: 9, name: "Time Alive" },
    artifacts: { enabled: false, order: 10, name: "Artifacts" },
    stagesCompleted: { enabled: false, order: 0, name: "Stages Completed" },
    score: { enabled: true, order: 6, name: "Score" },

    // items
    itemsCollected: { enabled: true, order: 5, name: "Items" },

    // drones
    dronesPurchased: { enabled: false, order: 0, name: "Drones" },
    turretsPurchased: { enabled: false, order: 0, name: "Turrets" },

    // combat
    kills: { enabled: false, order: 0, name: "Kills" },
    eliteKills: { enabled: false, order: 0, name: "Elite Kills" },
    minionKills: { enabled: false, order: 0, name: "Minion Kills" },
    deaths: { enabled: false, order: 0, name: "Deaths" },

    // damage
    damageDealt: { enabled: false, order: 0, name: "Damage Dealt" },
    minionDamageDealt: {
      enabled: false,
      order: 0,
      name: "Minion Damage Dealt",
    },
    damageTaken: { enabled: false, order: 0, name: "Damage Taken" },
    highestDamageDealt: {
      enabled: false,
      order: 0,
      name: "Highest Damage Dealt",
    },

    // healing
    healingRecieved: { enabled: false, order: 0, name: "Healing Recieved" },

    // progression
    highestLevel: { enabled: false, order: 0, name: "Highest Level" },
    goldCollected: { enabled: false, order: 0, name: "Gold Collected" },
    goldSpent: { enabled: false, order: 0, name: "Gold Spent" },
    lunarCoinsSpent: { enabled: false, order: 0, name: "Lunar Coins Spent" },
    purchases: { enabled: false, order: 0, name: "Purchases" },
    bloodPurchases: { enabled: false, order: 0, name: "Blood Purchases" },

    // movement
    distanceTraveledMetres: {
      enabled: false,
      order: 0,
      name: "Distance Traveled",
    },
  });
  let columnCount = $derived(
    Object.values(properties).filter((prop) => prop.enabled).length,
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
  class="fixed bg-bg-secondary border text-primary p-2 w-64"
  style="position-anchor: --visible-properties; position-area: bottom span-right;"
>
  <div>
    {#each Object.values(properties) as prop}
      <div class="flex flex-row justify-between">
        <span>{prop.name}</span>
        <input type="checkbox" bind:checked={prop.enabled} />
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
      class=" cursor-pointer bg-default hover:bg-hover active:bg-active p-2 font-mono mb-4"
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
