<script lang="ts">
  import type { Property, SortMode } from "$lib";
  import { ArrowDownWideNarrow, ArrowUpWideNarrow } from "@lucide/svelte";

  let {
    properties,
    sortProperty,
    setSort,
  }: {
    properties: Record<string, Property>;
    sortProperty: { by: string; sort: SortMode };
    setSort: (sort: SortMode, by: string) => void;
  } = $props();

  let contextMenu: {
    popup?: HTMLElement;
    id: string;
    pos: [number, number];
  } = $state({ id: "", pos: [0.0, 0.0] });

  export function open(e: MouseEvent, id: string) {
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
</script>

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
        onclick={() => setSort(sort, contextMenu.id)}
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
