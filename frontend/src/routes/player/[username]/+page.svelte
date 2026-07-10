<script lang="ts">
  import { page } from "$app/state";
  import PFP from "$lib/PFP.svelte";
  import type { PlayerInfoExtra } from "$lib";
  import { BODIES } from "$lib/RoR2";
  import TableDifficulty from "$lib/TableDifficulty.svelte";
  import TableSurvivor from "$lib/TableSurvivor.svelte";
  import { Plane } from "@lucide/svelte";

  import { onMount } from "svelte";

  let playerInfoPromise: Promise<PlayerInfoExtra> = $state(
    new Promise(() => {}),
  );
  let regionPromise: Promise<any> | null = $state(null);

  onMount(() => {
    const username = page.params.username;
    if (!username) console.error("no username");
    playerInfoPromise = fetch(`/api/player/${username}`)
      .then((r) => r.json())
      .then((info) => {
        if (info.region) {
          regionPromise = fetch(
            `https://countries.dev/alpha/${info.region}`,
          ).then((r) => r.json());
        }
        return info;
      });
  });
</script>

{#await playerInfoPromise}
  loading
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
