<script lang="ts">
  import { page } from "$app/state";
  import PFP from "$lib/PFP.svelte";
  import { api, type PlayerInfoExtra } from "$lib";
  import {
    BODIES,
    formatBig,
    formatSeconds,
    type RunReportWithUser,
  } from "$lib/RoR2";
  import TableDifficulty from "$lib/TableDifficulty.svelte";
  import TableSurvivor from "$lib/TableSurvivor.svelte";
  import { Plane } from "@lucide/svelte";

  import { onMount } from "svelte";
  import { defaultProperties } from "$lib/properties";
  import LoadingIndicator from "$lib/LoadingIndicator.svelte";
  import Table from "../../Table.svelte";
  import TableView from "$lib/TableView.svelte";

  let playerInfoPromise: Promise<PlayerInfoExtra> = $state(
    new Promise(() => {}),
  );
  let lifetimeStatsPromise: Promise<RunReportWithUser> = $state(
    new Promise(() => {}),
  );
  let recentRunsPromise: Promise<{ runs: RunReportWithUser[]; total: number }> =
    $state(new Promise(() => {}));
  let regionPromise: Promise<any> | null = $state(null);

  let username: string | null = $state(null);

  onMount(() => {
    if (!page.params.username) console.error("no username");
    username = page.params.username as string | null;

    playerInfoPromise = fetch(api(`player/${username}`))
      .then((r) => r.json())
      .then((info) => {
        if (info.region) {
          regionPromise = fetch(
            `https://countries.dev/alpha/${info.region}`,
          ).then((r) => r.json());
        }
        return info;
      });
    lifetimeStatsPromise = fetch(api(`player/lifetime/${username}`)).then((r) =>
      r.json(),
    );
    recentRunsPromise = fetch(
      api(
        `runs?${new URLSearchParams({ filters: JSON.stringify({ player: [`@${username}`] }) }).toString()}`,
      ),
    ).then((r) => r.json());
  });
</script>

{#await playerInfoPromise}
  <LoadingIndicator indicator text="Loading user info" />
{:then playerInfo}
  {#if !("error" in playerInfo)}
    <div class="flex flex-row justify-between">
      <div class="flex flex-row gap-4">
        <PFP src={playerInfo.image} class="h-60 border" />
        <div>
          <h2 class="text-4xl tracking-tighter">
            {#if regionPromise}
              {#await regionPromise}
                Loading Region
              {:then regionInfo}
                <span title={regionInfo.name}>
                  {regionInfo.flag}
                </span>
              {/await}
            {/if}
            {#if playerInfo.display_username}
              {playerInfo.display_username}
            {:else}
              {playerInfo.username}
            {/if}
          </h2>
          <h3 class="text-secondary text-xl">@{playerInfo.username}</h3>
          <h2 class="mt-4 text-2xl tracking-tighter">About Me</h2>
          {#if playerInfo.about_me}
            {playerInfo.about_me}
          {:else}
            <span class="italic text-secondary">None yet!</span>
          {/if}
        </div>
      </div>
      <table>
        {#snippet playerDataHeader(name: string)}
          <td class="p-4 text-lg tracking-tighter text-right">{name}</td>
        {/snippet}
        <tbody>
          <tr>
            {@render playerDataHeader("Runs")}
            <td class="bg-bg-secondary p-4 border">
              {playerInfo.run_count}
            </td>
          </tr>
          <tr>
            {@render playerDataHeader("Wins")}
            <td class="bg-bg-secondary p-4 border">
              {playerInfo.win_count}
              {#if playerInfo.run_count != 0}
                ({(
                  (playerInfo.win_count / playerInfo.run_count) *
                  100.0
                ).toFixed(1)}%)
              {/if}
            </td>
          </tr>
          <tr>
            {@render playerDataHeader("Favourite Survivor")}
            <td class="bg-bg-secondary p-4 border">
              {#if playerInfo.favourite_survivor}
                <TableSurvivor survivor={playerInfo.favourite_survivor} />
              {:else}
                <span>No data!</span>
              {/if}
            </td>
          </tr>
          <tr>
            {@render playerDataHeader("Favourite Difficulty")}
            <td class="bg-bg-secondary p-4 border">
              {#if playerInfo.favourite_difficulty}
                <TableDifficulty difficulty={playerInfo.favourite_difficulty} />
              {:else}
                <span>No data!</span>
              {/if}
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  {:else}
    {playerInfo.error}
  {/if}
{/await}

<hr class="my-8" />
<h1 class="text-center tracking-tighter text-3xl mb-4">Lifetime Stats</h1>
{#await lifetimeStatsPromise}
  <LoadingIndicator indicator text="Loading lifetime stats" />
{:then stats}
  <div class="flex flex-row flex-wrap">
    {#each Object.entries(stats) as [stat, value]}
      {#snippet basicStat(name: string)}
        {#if stat == name}
          <span class="text-yellow-200">{formatBig(Number(value))}</span>
        {/if}
      {/snippet}
      <div
        class="min-w-80 p-4 bg-bg-secondary border flex flex-row justify-between flex-1"
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

<hr class="my-8" />
<h1 class="text-center tracking-tighter text-3xl mb-4">Recent Runs</h1>

{#await recentRunsPromise}
  <LoadingIndicator indicator text="Loading recent runs" />
{:then recentRuns}
  {#if username}
    <TableView
      sort={{ by: "uploadTime", sort: "DESC" }}
      filter={{ player: [`@${username}`] }}
    />
  {/if}
{/await}
