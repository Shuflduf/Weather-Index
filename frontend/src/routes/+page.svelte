<script lang="ts">
  import {
    ArrowUpWideNarrow,
    ArrowDownWideNarrow,
    Check,
    ChevronLeft,
    ChevronRight,
    LoaderCircle,
  } from "@lucide/svelte";
  import { defaultProperties } from "$lib/properties";
  import { onMount } from "svelte";
  import Table from "./Table.svelte";
  import { api, type Property, type SortMode } from "$lib";
  import ContextMenu from "./ContextMenu.svelte";
  import LoadingIndicator from "$lib/LoadingIndicator.svelte";
  import {
    BODIES,
    DIFFICULTIES,
    ENDINGS,
    formatBig,
    type RunReportWithUser,
  } from "$lib/RoR2";
  import TableView from "$lib/TableView.svelte";

  const TABLE_STORAGE_KEY = "table-properties";
  const SORT_STORAGE_KEY = "sort-property";

  let loaded = $state(false);
  let properties: Record<string, Property> = $state(
    structuredClone(defaultProperties),
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
    sort: "ASC",
  });
  let contextMenu: any = $state();
  let loadMoreObserver: IntersectionObserver;
  let runPromise: Promise<any> = $state(new Promise(() => {}));
  let loadingStatus: "LOADING" | "END OF RUNS" | "NOT LOADING" =
    $state("NOT LOADING");
  let runs: RunReportWithUser[] = $state([]);
  let pageNumber: number = $state(0);
  let totalRuns: number = $state(0);

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

  function resetTable() {
    runs = [];
    pageNumber = 0;
    totalRuns = 0;
    fetchRuns();
  }

  function fetchRuns() {
    const filters = Object.fromEntries(
      Object.entries(properties)
        .filter(([_, data]) => {
          return data.filter && data.filter.length > 0;
        })
        .map(([prop, data]) => [prop, data.filter]),
    );
    loadingStatus = "LOADING";
    runPromise = fetch(
      api("runs") +
        "?" +
        new URLSearchParams({
          filters: JSON.stringify(filters),
          page: pageNumber.toFixed(0),
          only: JSON.stringify(
            Object.entries(properties)
              .filter(([_, v]) => v.enabled)
              .map(([k, _]) => k),
          ),
          ...sortProperty,
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

  function setSort(sort: SortMode, by: string) {
    sortProperty.by = by;
    sortProperty.sort = sort;

    localStorage.setItem(SORT_STORAGE_KEY, JSON.stringify(sortProperty));

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

<div class="flex flex-row gap-4 flex-wrap gap-y-2 mb-4 items-center">
  <button
    popovertarget="visible-properties"
    class="cursor-pointer bg-default hover:bg-hover active:bg-active p-2 font-mono border"
    style="anchor-name: --visible-properties;"
  >
    Visible Properties
  </button>
  <span class="font-mono p-2 text-secondary flex flex-row gap-2 h-min">
    <span>
      Sorting By: {properties[sortProperty.by].name}
    </span>
    {#if sortProperty.sort == "ASC"}
      <ArrowUpWideNarrow />
    {:else}
      <ArrowDownWideNarrow />
    {/if}
  </span>

  {#each Object.entries(properties) as [key, prop]}
    {#if prop.filter.length > 0}
      <button
        onclick={() => {
          prop.filter = [];
          resetTable();
        }}
        class="p-2 bg-default hover:bg-hover active:bg-active border cursor-pointer flex flex-row gap-2 items-center font-mono"
        title="Click to remove filter"
      >
        <span class="font-bold text-lg">
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
                <img src="/bodies/{BODIES[survivor].icon}" alt="" class="h-8" />
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
                <img src="/endings/{ENDINGS[ending].icon}" alt="" class="h-8" />
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
</div>

<div class="text-secondary text-right">
  Showing
  <span class="font-mono bg-default p-1">
    {runs.length}
  </span>
  out of
  <span class="font-mono bg-default p-1">
    {totalRuns}
  </span>
  total runs
</div>

<TableView {runs} openContextMenu={contextMenu?.open} />

{#if loadingStatus == "LOADING"}
  <LoadingIndicator indicator text="Loading more runs!" />
{:else if loadingStatus == "NOT LOADING"}
  <div class="p-2" use:observeLoadMore></div>
{:else if loadingStatus == "END OF RUNS"}
  <LoadingIndicator text="No more runs!" />
{/if}
