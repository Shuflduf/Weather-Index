<script lang="ts">
  import type { Property, SortMode } from "$lib";
  import {
    BODIES,
    DIFFICULTIES,
    ENDINGS,
    ORDERED_DIFFICULTIES,
    ORDERED_ENDINGS,
    ORDERED_SURVIVORS,
  } from "$lib/RoR2";
  import {
    ArrowDownWideNarrow,
    ArrowUpWideNarrow,
    Check,
    ChevronLeft,
    ChevronRight,
  } from "@lucide/svelte";

  let {
    properties,
    sortProperty,
    setSort,
    setFilter,
    sortEnabled = true,
    filterEnabled = true,
  }: {
    properties: Record<string, Property>;
    sortProperty: { by: string; sort: SortMode };
    setSort: (sort: SortMode, by: string) => void;
    setFilter: (prop: string, filter: string[]) => void;
    sortEnabled?: boolean;
    filterEnabled?: boolean;
  } = $props();

  let contextMenu: {
    popup?: HTMLElement;
    id: string;
    pos: [number, number];
  } = $state({ id: "", pos: [0.0, 0.0] });

  // let survivorChecked: Record<string, boolean> = $state({});
  // let difficultyChecked: Record<string, boolean> = $state({});
  // let endingChecked: Record<string, boolean> = $state({});

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
    contextMenu.pos = [
      Math.min(
        e.clientX,
        window.innerWidth - contextMenu.popup.getBoundingClientRect().width,
      ),
      e.clientY,
    ];

    // if (id == "survivor" && Object.keys(survivorChecked).length == 0) {
    //   survivorChecked = Object.fromEntries(
    //     ORDERED_SURVIVORS.map((s) => [
    //       s,
    //       properties.survivor.filter.includes(s),
    //     ]),
    //   );
    // }
    // if (id == "ending" && Object.keys(endingChecked).length == 0) {
    //   endingChecked = Object.fromEntries(
    //     ORDERED_ENDINGS.map((s) => [s, properties.ending.filter.includes(s)]),
    //   );
    // }
    // if (id == "difficulty" && Object.keys(difficultyChecked).length == 0) {
    //   difficultyChecked = Object.fromEntries(
    //     ORDERED_DIFFICULTIES.map((s) => [
    //       s,
    //       properties.difficulty.filter.includes(s),
    //     ]),
    //   );
    // }
  }

  function swapSign(prop: string) {
    const p = properties[prop];
    if (p.filter.length == 0) {
      setFilter(prop, ["<0"]);
      return;
    }

    const filter = p.filter[0];
    if (filter.startsWith("<")) {
      const value = filter.slice(1);
      setFilter(prop, [`>${value}`]);

      return;
    } else if (filter.startsWith(">")) {
      const value = filter.slice(1);
      setFilter(prop, [`<${value}`]);
    }
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
      <Check
        class="absolute right-2 w-4 h-4 pointer-events-none"
        strokeWidth="2"
      />
    </div>
    {#if sortEnabled}
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

    {#if filterEnabled}
      {#if contextMenu.id == "survivor"}
        <div class="mt-2">
          {#each ORDERED_SURVIVORS as survivor}
            <div class="flex flex-row items-center gap-2 relative">
              <input
                type="checkbox"
                class="aspect-square"
                checked={properties["survivor"].filter.includes(survivor)}
                onchange={(e) => {
                  if (!e.currentTarget.checked) {
                    setFilter(
                      "survivor",
                      properties["survivor"].filter.filter(
                        (f) => f != survivor,
                      ),
                    );
                  } else {
                    setFilter(
                      "survivor",
                      [...properties["survivor"].filter, survivor].toSorted(
                        (a, b) =>
                          ORDERED_SURVIVORS.indexOf(a) -
                          ORDERED_SURVIVORS.indexOf(b),
                      ),
                    );
                  }
                }}
              />
              <Check
                class="absolute left-0 w-4 h-4 pointer-events-none"
                strokeWidth="2"
              />
              <img
                src="/bodies/{BODIES[survivor].icon}"
                alt="survivor"
                class="h-4"
              />
              {BODIES[survivor].displayName}
            </div>
          {/each}
        </div>
      {/if}

      {#if contextMenu.id == "difficulty"}
        <div class="mt-2">
          {#each ORDERED_DIFFICULTIES as difficulty}
            <div class="flex flex-row items-center gap-2 relative">
              <input
                type="checkbox"
                class="aspect-square"
                checked={properties["difficulty"].filter.includes(difficulty)}
                onchange={(e) => {
                  if (!e.currentTarget.checked) {
                    setFilter(
                      "difficulty",
                      properties["difficulty"].filter.filter(
                        (f) => f != difficulty,
                      ),
                    );
                  } else {
                    setFilter(
                      "difficulty",
                      [...properties["difficulty"].filter, difficulty].toSorted(
                        (a, b) =>
                          ORDERED_DIFFICULTIES.indexOf(a) -
                          ORDERED_DIFFICULTIES.indexOf(b),
                      ),
                    );
                  }
                }}
              />
              <Check
                class="absolute left-0 w-4 h-4 pointer-events-none"
                strokeWidth="2"
              />
              <img
                src="/difficulties/{DIFFICULTIES[difficulty].icon}"
                alt="difficulty"
                class="h-4"
              />
              {DIFFICULTIES[difficulty].displayName}
            </div>
          {/each}
        </div>
      {/if}
      {#if contextMenu.id == "ending"}
        <div class="mt-2">
          {#each ORDERED_ENDINGS as ending}
            <div
              class="flex flex-row items-center gap-2 relative px-2 border"
              style="background-color: {ENDINGS[ending].colorBg};"
            >
              <input
                type="checkbox"
                class="aspect-square"
                checked={properties["ending"].filter.includes(ending)}
                onchange={(e) => {
                  if (!e.currentTarget.checked) {
                    setFilter(
                      "ending",
                      properties["ending"].filter.filter((f) => f != ending),
                    );
                  } else {
                    setFilter(
                      "ending",
                      [...properties["ending"].filter, ending].toSorted(
                        (a, b) =>
                          ORDERED_ENDINGS.indexOf(a) -
                          ORDERED_ENDINGS.indexOf(b),
                      ),
                    );
                  }
                }}
              />
              <Check
                class="absolute left-2 w-4 h-4 pointer-events-none"
                strokeWidth="2"
              />
              <img
                src="/endings/{ENDINGS[ending].icon}"
                alt="ending"
                class="h-4"
              />
              {ENDINGS[ending].displayName}
            </div>
          {/each}
        </div>
      {/if}
      {#if contextMenu.id == "player"}
        <div class="mt-2">
          <input
            type="text"
            title="Use @ to specify exact username"
            placeholder="Username"
            value={properties["player"].filter.length != 0
              ? properties["player"].filter[0]
              : ""}
            class="w-full bg-default hover:bg-hover active:bg-active transition-colors p-2 outline-none font-mono text-sm"
            onchange={(e) => {
              const value = e.currentTarget.value;
              setFilter("player", [value]);
            }}
          />
        </div>
      {/if}

      {#snippet numberFilter(prop: string)}
        {#if contextMenu.id == prop}
          <div class="mt-2 flex flex-row w-full">
            <!--
          <span class="h-full block p-2 font-bold">
            {properties[prop].name}
          </span>
          -->
            <button
              onclick={() => swapSign(prop)}
              class="p-2 cursor-pointer border bg-default hover:bg-hover active:bg-hover transition-colors"
            >
              {#if properties[prop].filter.length == 0 || properties[prop].filter[0].startsWith(">")}
                <ChevronRight />
              {:else}
                <ChevronLeft />
              {/if}
            </button>
            <input
              type="number"
              min="0"
              value={properties[prop].filter.length != 0
                ? properties[prop].filter[0].slice(1)
                : 0}
              class="p-2 border bg-default hover:bg-hover active:bg-active outline-none font-mono w-full transition-colors text-xs"
              onchange={(e) => {
                const value = Math.abs(
                  Math.floor(Number(e.currentTarget.value)),
                );
                e.currentTarget.value = value.toString();
                if (properties[prop].filter.length == 0) {
                  setFilter(prop, [`>${value}`]);
                  return;
                }

                const sign = properties[prop].filter[0].slice(0, 1);
                setFilter(prop, [`${sign}${value}`]);
              }}
            />
          </div>
        {/if}
      {/snippet}

      {#snippet timeFilter(prop: string)}
        {#if contextMenu.id == prop}
          <div class="mt-2 w-full">
            <button
              onclick={() => swapSign(prop)}
              class="p-2 cursor-pointer border bg-default hover:bg-hover active:bg-hover transition-colors w-full flex justify-center"
            >
              {#if properties[prop].filter.length == 0 || properties[prop].filter[0].startsWith(">")}
                <ChevronRight />
              {:else}
                <ChevronLeft />
              {/if}
            </button>
            <input
              type="datetime-local"
              class="w-full text-xs"
              value={properties[prop].filter.length != 0
                ? properties[prop].filter[0].slice(1)
                : "0001-01-01T00:00"}
              onchange={(e) => {
                const oldValue =
                  properties[prop].filter.length > 0
                    ? properties[prop].filter[0].slice(1)
                    : "0001-01-01T00:00";

                const value = e.currentTarget.value;
                if (value == "") {
                  e.currentTarget.value = oldValue;
                }
                if (properties[prop].filter.length == 0) {
                  setFilter(prop, [`>${value}`]);
                  return;
                }

                const sign = properties[prop].filter[0].slice(0, 1);
                setFilter(prop, [`${sign}${value}`]);
              }}
            />
          </div>
        {/if}
      {/snippet}

      {@render numberFilter("id")}
      {@render numberFilter("timeAlive")}
      {@render numberFilter("stagesCompleted")}
      {@render numberFilter("score")}
      {@render numberFilter("itemsCollected")}
      {@render numberFilter("dronesPurchased")}
      {@render numberFilter("turretsPurchased")}
      {@render numberFilter("kills")}
      {@render numberFilter("eliteKills")}
      {@render numberFilter("minionKills")}
      {@render numberFilter("deaths")}
      {@render numberFilter("damageDealt")}
      {@render numberFilter("minionDamageDealt")}
      {@render numberFilter("damageTaken")}
      {@render numberFilter("highestDamageDealt")}
      {@render numberFilter("healingRecieved")}
      {@render numberFilter("highestLevel")}
      {@render numberFilter("goldCollected")}
      {@render numberFilter("purchases")}
      {@render numberFilter("goldPurchases")}
      {@render numberFilter("bloodPurchases")}
      {@render numberFilter("lunarPurchases")}
      {@render numberFilter("distanceTraveled")}

      {@render timeFilter("startTime")}
      {@render timeFilter("uploadTime")}
    {/if}
  {/if}
</div>
