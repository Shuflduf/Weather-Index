<script lang="ts">
  import { onMount } from "svelte";
  import { BODIES, DIFFICULTIES, ENDINGS, type Item } from "$lib/RoR2";
  import UserDisplay from "$lib/UserDisplay.svelte";
  import TableBlock from "./TableBlock.svelte";

  let properties: Record<string, { enabled: boolean; order: number }> = $state({
    id: { enabled: true, order: 0 },
    player: { enabled: true, order: 1 },
    uploadTime: { enabled: false, order: 0 },

    // run info
    survivor: { enabled: true, order: 2 },
    startTime: { enabled: false, order: 0 },
    ending: { enabled: true, order: 3 },
    difficulty: { enabled: true, order: 4 },
    timeAlive: { enabled: false, order: 0 },
    artifacts: { enabled: false, order: 0 },
    stagesCompleted: { enabled: false, order: 0 },
    score: { enabled: true, order: 6 },

    // items
    itemsCollected: { enabled: true, order: 5 },

    // drones
    dronesPurchased: { enabled: false, order: 0 },
    turretsPurchased: { enabled: false, order: 0 },

    // combat
    kills: { enabled: false, order: 0 },
    eliteKills: { enabled: false, order: 0 },
    minionKills: { enabled: false, order: 0 },
    deaths: { enabled: false, order: 0 },

    // damage
    damageDealt: { enabled: false, order: 0 },
    minionDamageDealt: { enabled: false, order: 0 },
    damageTaken: { enabled: false, order: 0 },
    highestDamageDealt: { enabled: false, order: 0 },

    // healing
    healingRecieved: { enabled: false, order: 0 },

    // progression
    highestLevel: { enabled: false, order: 0 },
    goldCollected: { enabled: false, order: 0 },
    goldSpent: { enabled: false, order: 0 },
    lunarCoinsSpent: { enabled: false, order: 0 },
    purchases: { enabled: false, order: 0 },
    bloodPurchases: { enabled: false, order: 0 },

    // movement
    distanceTraveledMetres: { enabled: false, order: 0 },
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

  function onDrag(e: PointerEvent) {
    if (!drag.dragging || !drag.elem) return;

    drag.elem.style.transform = "none";
    const normalWidth = drag.elem.getBoundingClientRect().width;
    const normalLeft = drag.elem.getBoundingClientRect().left;
    const normalCenter = normalLeft + normalWidth / 2.0;
    const dx = e.clientX - drag.startX - normalLeft;
    const elemCenter = normalCenter + dx;
    drag.elem.style.transform = `translateX(${dx}px)`;
    drag.elem.style.zIndex = "10";
    console.log(drag);

    const headers = [
      ...document.querySelectorAll("[data-col-header]"),
    ] as HTMLElement[];
    headers.sort((a, b) => properties[a.id].order - properties[b.id].order);
    const curIdx = headers.findIndex((h) => h.id == drag.id);
    if (curIdx == -1) return;

    if (curIdx > 0) {
      const leftRect = headers[curIdx - 1].getBoundingClientRect();
      const leftCenter = leftRect.left + leftRect.width / 2.0;
      const diff = (leftCenter + normalCenter) / 2.0;
      if (elemCenter < diff) {
        const temp = properties[drag.id].order;
        properties[drag.id].order = properties[headers[curIdx - 1].id].order;
        properties[headers[curIdx - 1].id].order = temp;
        drag.elem.style.transform = `translateX(${dx}px)`;
        return;
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
        return;
      }
    }

    // for (const header of headers) {
    //   const el = header as HTMLElement;
    //   const rect = el.getBoundingClientRect();
    //   if (e.clientX > rect.left && e.clientX < rect.right && el.id != drag.id) {

    //     break;
    //   }
    // }
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
  {#snippet visibleProperty(name: string, id: string)}
    <div class="flex flex-row justify-between">
      <span>{name}</span>
      <input type="checkbox" bind:checked={properties[id].enabled} />
    </div>
  {/snippet}
  <div>
    {@render visibleProperty("ID", "id")}
    {@render visibleProperty("Player", "player")}
    {@render visibleProperty("Survivor", "survivor")}
    {@render visibleProperty("Ending", "ending")}
    {@render visibleProperty("Difficulty", "difficulty")}
    {@render visibleProperty("Items", "itemsCollected")}
    {@render visibleProperty("Score", "score")}
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
      style="grid-template-columns: repeat({columnCount}, auto);"
    >
      {#snippet propHeader(name: string, id: string)}
        <div
          role="button"
          tabindex="-1"
          onpointerdown={(e) => startDrag(e, id)}
          onkeydown={null}
          {id}
          data-col-header
          class="items-center justify-center h-12 hover:bg-hover transition-colors cursor-grab flex"
          style="order: {properties[id].order}; grid-row: 1;"
        >
          <h2 class="text-xl tracking-tight text-center font-bold">
            {name}
          </h2>
        </div>
      {/snippet}

      {#if properties.id.enabled}
        {@render propHeader("ID", "id")}
        {#each runs as run, idx}
          <TableBlock order={properties.id.order} {idx}>
            <a href={`/run/${run.id}`}>{run.id}</a>
          </TableBlock>
        {/each}
      {/if}

      {#if properties.player.enabled}
        {@render propHeader("Player", "player")}
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
        {@render propHeader("Survivor", "survivor")}
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
        {@render propHeader("Ending", "ending")}
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
        {@render propHeader("Difficulty", "difficulty")}
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
        {@render propHeader("Items", "itemsCollected")}
        {#each runs as run, idx}
          <TableBlock order={properties.itemsCollected.order} {idx}>
            {run.items_collected}
          </TableBlock>
        {/each}
      {/if}
      {#if properties.score.enabled}
        {@render propHeader("Score", "score")}
        {#each runs as run, idx}
          <TableBlock order={properties.score.order} {idx}>
            {run.score}
          </TableBlock>
        {/each}
      {/if}
    </div>
  {/await}
</div>
