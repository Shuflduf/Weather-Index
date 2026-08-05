<script lang="ts">
  import { api, type StatsCategory } from "$lib";
  import { onMount } from "svelte";
  import { defaultProperties } from "./properties";
  import { formatBig, formatSeconds, type RunReportWithUser } from "./RoR2";
  import LoadingIndicator from "./LoadingIndicator.svelte";

  let { username }: { username?: string } = $props();

  let mode: StatsCategory = $state("SUM");
  let statsPromise: Promise<Record<StatsCategory, RunReportWithUser>> = $state(
    new Promise(() => {}),
  );

  onMount(() => {
    statsPromise = Promise.all([
      fetch(
        username
          ? api(`stats/sum?${new URLSearchParams({ username })}`)
          : api("stats/sum"),
      ).then((r) => r.json()),
      fetch(
        username
          ? api(`stats/avg?${new URLSearchParams({ username })}`)
          : api("stats/avg"),
      ).then((r) => r.json()),
    ]).then(([sum, avg]) => ({ SUM: sum, AVG: avg }));
  });
</script>

{#await statsPromise}
  <LoadingIndicator indicator text="Loading stats" />
{:then stats}
  <div class="flex flex-row flex-wrap gap-4">
    <button
      class="w-full min-w-60 flex-1 cursor-pointer border p-2 font-mono transition-colors hover:bg-hover {mode ==
      'AVG'
        ? 'bg-default text-secondary active:bg-active'
        : 'bg-active text-primary active:bg-default'}"
      onclick={() => (mode = "SUM")}
    >
      Total
    </button>
    <button
      class="w-full min-w-60 flex-1 cursor-pointer border p-2 font-mono transition-colors hover:bg-hover {mode ==
      'SUM'
        ? 'bg-default text-secondary active:bg-active'
        : 'bg-active text-primary active:bg-default'}"
      onclick={() => (mode = "AVG")}
    >
      Average
    </button>
  </div>
  <div class="mt-4 flex flex-row flex-wrap">
    {#each Object.entries(stats[mode]) as [stat, value]}
      {#snippet basicStat(name: string)}
        {#if stat == name}
          <span class="text-yellow-200">{formatBig(Number(value))}</span>
        {/if}
      {/snippet}
      <div
        class="flex min-w-80 flex-1 flex-row justify-between border bg-bg-secondary p-4"
      >
        <span>{defaultProperties[stat].name}</span>

        {#if stat == "timeAliveSeconds"}
          <span class="text-yellow-200">{formatSeconds(Number(value))}</span>
        {/if}
        {#if stat == "distanceTraveled"}
          <span>
            <span class="text-yellow-200">{formatBig(Number(value))}</span>
            m
          </span>
        {/if}
        {@render basicStat("stagesCompleted")}
        {@render basicStat("score")}
        {@render basicStat("itemsCollected")}
        {@render basicStat("dronesPurchased")}
        {@render basicStat("turretsPurchased")}
        {@render basicStat("kills")}
        {@render basicStat("eliteKills")}
        {@render basicStat("minionKills")}
        {@render basicStat("deaths")}
        {@render basicStat("damageDealt")}
        {@render basicStat("minionDamageDealt")}
        {@render basicStat("damageTaken")}
        {@render basicStat("highestDamageDealt")}
        {@render basicStat("healingRecieved")}
        {@render basicStat("highestLevel")}
        {@render basicStat("goldCollected")}
        {@render basicStat("purchases")}
        {@render basicStat("goldPurchases")}
        {@render basicStat("bloodPurchases")}
        {@render basicStat("lunarPurchases")}
      </div>
    {/each}
  </div>
{/await}
