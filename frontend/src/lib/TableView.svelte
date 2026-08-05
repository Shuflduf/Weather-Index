<script lang="ts">
  import { api, type Property, type SortMode } from "$lib";
  import { onMount } from "svelte";
  import Table from "$lib/Table.svelte";
  import { defaultProperties } from "./properties";
  import {
    BODIES,
    DIFFICULTIES,
    ENDINGS,
    formatBig,
    type RunReportWithUser,
  } from "./RoR2";
  import ContextMenu from "./ContextMenu.svelte";
  import {
    ArrowDownWideNarrow,
    ArrowUpWideNarrow,
    Check,
    ChevronLeft,
    ChevronRight,
  } from "@lucide/svelte";
  import LoadingIndicator from "./LoadingIndicator.svelte";

  const TABLE_STORAGE_KEY = "table-properties";
  const SORT_STORAGE_KEY = "sort-property";

  type Sort = { by: string; sort: SortMode };

  let {
    sort = null,
    filter = null,
  }: {
    sort?: Sort | null;
    filter?: Record<string, string[]> | null;
  } = $props();

  let properties: Record<string, Property> = $state(
    structuredClone(defaultProperties),
  );

  let fallbackSort: Sort = $state({
    by: "id",
    sort: "DESC",
  });
  let loaded = $state(false);
  let pageNumber: number = $state(0);
  let totalRuns: number = $state(0);
  let propsByCategory = $derived(
    Object.values(properties).reduce(
      (acc, prop) => {
        (acc[prop.category] ??= []).push(prop);
        return acc;
      },
      {} as Record<string, Property[]>,
    ),
  );

  let contextMenu: any = $state();
  let loadMoreObserver: IntersectionObserver;
  let loadingStatus: "LOADING" | "END OF RUNS" | "NOT LOADING" =
    $state("NOT LOADING");
  let runs: RunReportWithUser[] = $state([]);

  function resetTable() {
    runs = [];
    pageNumber = 0;
    totalRuns = 0;
    fetchRuns();
  }
  function observeLoadMore(node: HTMLElement) {
    loadMoreObserver = new IntersectionObserver((entries) => {
      if (entries[0].isIntersecting && loadingStatus == "NOT LOADING") {
        pageNumber += 1;
        fetchRuns();
      }
    });
    loadMoreObserver.observe(node);
    return {
      destroy() {
        loadMoreObserver.disconnect();
      },
    };
  }

  function setSort(sort: SortMode, by: string) {
    fallbackSort.by = by;
    fallbackSort.sort = sort;

    localStorage.setItem(SORT_STORAGE_KEY, JSON.stringify(fallbackSort));

    resetTable();
  }

  function setFilter(prop: string, filter: string[]) {
    properties[prop].filter = filter;

    resetTable();
  }

  function resetProperties() {
    properties = structuredClone(defaultProperties);

    resetTable();
  }

  function fetchRuns() {
    const activeFilters =
      filter ??
      Object.fromEntries(
        Object.entries(properties)
          .filter(([_, data]) => {
            return data.filter && data.filter.length > 0;
          })
          .map(([prop, data]) => [prop, data.filter]),
      );
    loadingStatus = "LOADING";
    fetch(
      api("runs") +
        "?" +
        new URLSearchParams({
          filters: JSON.stringify(activeFilters),
          page: pageNumber.toFixed(0),
          only: JSON.stringify(
            Object.entries(properties)
              .filter(([_, v]) => v.enabled)
              .map(([k, _]) => k),
          ),
          ...(sort ?? fallbackSort),
        }).toString(),
    )
      .then((r) => r.json())
      .then((j: { total: number; runs: RunReportWithUser[] }) => {
        if (j.runs.length != 0) {
          runs = runs.concat(j.runs);
          totalRuns = j.total;
          loadingStatus = "NOT LOADING";
        } else {
          loadingStatus = "END OF RUNS";
        }
      });
  }

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

  onMount(() => {
    const savedSort = localStorage.getItem(SORT_STORAGE_KEY);
    if (savedSort) {
      const parsed = JSON.parse(savedSort);
      fallbackSort = parsed;
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
</script>

<ContextMenu
  bind:this={contextMenu}
  {properties}
  sortProperty={fallbackSort}
  {setSort}
  {setFilter}
  sortEnabled={sort == null}
  filterEnabled={filter == null}
/>
<div
  id="visible-properties"
  popover
  class="fixed border bg-bg-secondary p-2 text-primary"
  style="position-anchor: --visible-properties; position-area: bottom span-right;"
>
  <div class="flex flex-row gap-8">
    {#each Object.entries(propsByCategory) as [category, props] (category)}
      <div class="flex w-40 flex-col gap-1">
        <h2 class="text-xl font-bold tracking-tighter">{category}</h2>
        {#each props as prop (prop.name)}
          <div class="relative flex flex-row items-center justify-between">
            <span>{prop.name}</span>
            <input
              type="checkbox"
              checked={prop.enabled}
              onchange={(e) => {
                prop.enabled = e.currentTarget.checked;
                resetTable();
              }}
              class=""
            />
            <Check
              class="pointer-events-none absolute right-0 h-4 w-4"
              strokeWidth="2"
            />
          </div>
        {/each}
      </div>
    {/each}
  </div>
  <button
    class="cursor-pointer bg-default p-2 font-mono transition-colors hover:bg-hover active:bg-active"
    onclick={resetProperties}
  >
    Reset
  </button>
</div>

<div class="mb-4 flex flex-row flex-wrap items-center gap-4 gap-y-2">
  <button
    popovertarget="visible-properties"
    class="cursor-pointer border bg-default p-2 font-mono hover:bg-hover active:bg-active"
    style="anchor-name: --visible-properties;"
  >
    Visible Properties
  </button>
  {#if sort == null}
    <span class="flex h-min flex-row gap-2 p-2 font-mono text-secondary">
      <span>
        Sorting By: {properties[fallbackSort.by].name}
      </span>
      {#if fallbackSort.sort == "ASC"}
        <ArrowUpWideNarrow />
      {:else}
        <ArrowDownWideNarrow />
      {/if}
    </span>
  {/if}

  {#if filter == null}
    {#each Object.entries(properties) as [key, prop]}
      {#if prop.filter.length > 0}
        <button
          onclick={() => {
            prop.filter = [];
            resetTable();
          }}
          class="flex cursor-pointer flex-row items-center gap-2 border bg-default p-2 font-mono hover:bg-hover active:bg-active"
          title="Click to remove filter"
        >
          <span class="text-lg font-bold">
            {prop.name}
          </span>
          {#if prop.filter[0].startsWith("<")}
            <ChevronLeft />
          {:else if prop.filter[0].startsWith(">")}
            <ChevronRight />
          {/if}
          {#if prop.filter[0].startsWith("<") || prop.filter[0].startsWith(">")}
            {#if !isNaN(Number(prop.filter[0].slice(1)))}
              <span>
                {formatBig(Number(prop.filter[0].slice(1)))}
              </span>
            {:else}
              <span>
                {new Date(prop.filter[0].slice(1)).toLocaleString()}
              </span>
            {/if}
          {/if}
          {#if key == "player"}
            <span>:</span>
            {#if prop.filter[0].startsWith("@")}
              {prop.filter[0]}
            {:else}
              "{prop.filter[0]}"
            {/if}
          {/if}
          {#if key == "survivor"}
            <span>:</span>
            <div class="flex flex-row gap-1">
              {#each prop.filter as survivor}
                <div class="flex flex-row items-center gap-1">
                  <img
                    src="/bodies/{BODIES[survivor].icon}"
                    alt=""
                    class="h-8"
                  />
                  <!-- <span>{BODIES[survivor].displayName}</span> -->
                </div>
              {/each}
            </div>
          {/if}
          {#if key == "ending"}
            <span>:</span>
            <div class="flex flex-row gap-4">
              {#each prop.filter as ending}
                <div
                  class="flex flex-row items-center gap-1 px-2"
                  style="background-color: {ENDINGS[ending].colorBg};"
                >
                  <img
                    src="/endings/{ENDINGS[ending].icon}"
                    alt=""
                    class="h-8"
                  />
                  <span>{ENDINGS[ending].displayName}</span>
                </div>
              {/each}
            </div>
          {/if}
          {#if key == "difficulty"}
            <span>:</span>
            <div class="flex flex-row gap-1">
              {#each prop.filter as difficulty}
                <div class="flex flex-row items-center gap-1">
                  <img
                    src="/difficulties/{DIFFICULTIES[difficulty].icon}"
                    alt=""
                    class="h-8"
                  />
                  <!-- <span>{DIFFICULTIES[difficulty].displayName}</span> -->
                </div>
              {/each}
            </div>
          {/if}
        </button>
      {/if}
    {/each}
  {/if}
</div>

<div class="text-right text-secondary">
  Showing
  <span class="bg-default p-1 font-mono">
    {runs.length}
  </span>
  out of
  <span class="bg-default p-1 font-mono">
    {totalRuns}
  </span>
  total runs
</div>

<Table {properties} {runs} openContextMenu={contextMenu?.open} />

{#if loadingStatus == "LOADING"}
  <LoadingIndicator indicator text="Loading more runs!" />
{:else if loadingStatus == "NOT LOADING"}
  <div class="p-2" use:observeLoadMore></div>
{:else if loadingStatus == "END OF RUNS"}
  <LoadingIndicator text="No more runs!" />
{/if}
