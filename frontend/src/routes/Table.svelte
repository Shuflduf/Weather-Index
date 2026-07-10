<script lang="ts">
  import type { Property } from "$lib";
  import {
    BODIES,
    DIFFICULTIES,
    ENDINGS,
    formatBig,
    formatSeconds,
    type RunReportWithUser,
  } from "$lib/RoR2";
  import { ExternalLink } from "@lucide/svelte";
  import TableBlock from "./TableBlock.svelte";
  import UserDisplay from "$lib/UserDisplay.svelte";
  import ArtifactsDisplay from "$lib/ArtifactsDisplay.svelte";
  import { tick } from "svelte";
  import TableSurvivor from "$lib/TableSurvivor.svelte";
  import TableDifficulty from "$lib/TableDifficulty.svelte";

  const LEFT_MOUSE_BUTTON = 0;

  let {
    properties,
    runs,
    openContextMenu,
  }: {
    properties: Record<string, Property>;
    runs: RunReportWithUser[];
    openContextMenu: (e: MouseEvent, id: string) => null;
  } = $props();

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
</script>

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
      {#each runs as run, idx (run.id)}
        <TableBlock order={properties[id].order} {idx}>
          <span>{formatBig((run as Record<string, any>)[runId])}</span>
        </TableBlock>
      {/each}
    {/if}
  {/snippet}

  {#if properties.id.enabled}
    {@render propHeader("id")}
    {#each runs as run, idx (run.id)}
      <TableBlock order={properties.id.order} {idx}>
        <a
          href={`/run/${run.id}`}
          class="flex flex-row gap-2 underline text-lg"
        >
          {run.id}
          <ExternalLink />
        </a>
      </TableBlock>
    {/each}
  {/if}
  {#if properties.player.enabled}
    {@render propHeader("player")}
    {#each runs as run, idx (run.id)}
      <TableBlock order={properties.player.order} {idx}>
        <UserDisplay
          class="h-12"
          user={{
            username: run.userUsername,
            image: run.userImage,
            displayUsername: run.userDisplayUsername,
          }}
        />
      </TableBlock>
    {/each}
  {/if}
  {#if properties.uploadTime.enabled}
    {@render propHeader("uploadTime")}
    {#each runs as run, idx (run.id)}
      <TableBlock order={properties.uploadTime.order} {idx}>
        <span
          title={new Date(run.uploadTime).toString()}
          class="text-sm text-secondary"
        >
          {new Date(run.uploadTime).toLocaleString()}
        </span>
      </TableBlock>
    {/each}
  {/if}

  {#if properties.survivor.enabled}
    {@render propHeader("survivor")}
    {#each runs as run, idx (run.id)}
      <TableBlock order={properties.survivor.order} {idx}>
        <TableSurvivor survivor={run.survivor} />
      </TableBlock>
    {/each}
  {/if}
  {#if properties.startTime.enabled}
    {@render propHeader("startTime")}
    {#each runs as run, idx (run.id)}
      <TableBlock order={properties.startTime.order} {idx}>
        <span
          title={new Date(run.startTime).toString()}
          class="text-sm text-secondary"
        >
          {new Date(run.startTime).toLocaleString()}
        </span>
      </TableBlock>
    {/each}
  {/if}
  {#if properties.ending.enabled}
    {@render propHeader("ending")}
    {#each runs as run, idx (run.id)}
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
    {#each runs as run, idx (run.id)}
      <TableBlock order={properties.difficulty.order} {idx}>
        <TableDifficulty difficulty={run.difficulty} />
      </TableBlock>
    {/each}
  {/if}
  {#if properties.timeAlive.enabled}
    {@render propHeader("timeAlive")}
    {#each runs as run, idx (run.id)}
      <TableBlock order={properties.timeAlive.order} {idx}>
        <span class="">
          {formatSeconds(run.timeAliveSeconds)}
        </span>
      </TableBlock>
    {/each}
  {/if}
  {#if properties.artifacts.enabled}
    {@render propHeader("artifacts")}
    {#each runs as run, idx (run.id)}
      <TableBlock order={properties.artifacts.order} {idx}>
        <ArtifactsDisplay
          artifacts={run.artifacts}
          class="w-full overflow-x-auto flex-wrap"
        />
      </TableBlock>
    {/each}
  {/if}
  {@render basicCol("stagesCompleted", "stagesCompleted")}
  {@render basicCol("score", "score")}

  {@render basicCol("itemsCollected", "itemsCollected")}
  {@render basicCol("dronesPurchased", "dronesPurchased")}
  {@render basicCol("turretsPurchased", "turretsPurchased")}

  {@render basicCol("kills", "kills")}
  {@render basicCol("eliteKills", "eliteKills")}
  {@render basicCol("minionKills", "minionKills")}
  {@render basicCol("deaths", "deaths")}
  {@render basicCol("damageDealt", "damageDealt")}
  {@render basicCol("minionDamageDealt", "minionDamageDealt")}
  {@render basicCol("damageTaken", "damageTaken")}
  {@render basicCol("highestDamageDealt", "highestDamageDealt")}
  {@render basicCol("healingRecieved", "healingRecieved")}

  {@render basicCol("highestLevel", "highestLevel")}
  {@render basicCol("goldCollected", "goldCollected")}
  {@render basicCol("purchases", "purchases")}
  {@render basicCol("goldPurchases", "goldPurchases")}
  {@render basicCol("bloodPurchases", "bloodPurchases")}
  {@render basicCol("lunarPurchases", "lunarPurchases")}

  {#if properties.distanceTraveled.enabled}
    {@render propHeader("distanceTraveled")}
    {#each runs as run, idx (run.id)}
      <TableBlock order={properties.distanceTraveled.order} {idx}>
        {formatBig(run.distanceTraveledMetres)}

        <span class="text-yellow-200 ml-1">metres</span>
      </TableBlock>
    {/each}
  {/if}
</div>
