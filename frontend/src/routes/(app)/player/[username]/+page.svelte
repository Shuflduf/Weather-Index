<script lang="ts">
  import { page } from "$app/state";
  import PFP from "$lib/PFP.svelte";
  import { api, type PlayerInfoExtra, type StatsCategory } from "$lib";
  import TableDifficulty from "$lib/TableDifficulty.svelte";
  import TableSurvivor from "$lib/TableSurvivor.svelte";

  import { onMount } from "svelte";
  import LoadingIndicator from "$lib/LoadingIndicator.svelte";
  import TableView from "$lib/TableView.svelte";
  import StatsSurvivors from "$lib/StatsSurvivors.svelte";
  import StatsCombined from "$lib/StatsCombined.svelte";
  import StatsStages from "$lib/StatsStages.svelte";
  import StatsDifficulties from "$lib/StatsDifficulties.svelte";
  import StatsArtifacts from "$lib/StatsArtifacts.svelte";

  let playerInfoPromise: Promise<PlayerInfoExtra> = $state(
    new Promise(() => {}),
  );
  let regionPromise: Promise<any> | null = $state(null);

  let username: string | undefined = $state(page.params.username);

  onMount(() => {
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
          <h3 class="text-xl text-secondary">@{playerInfo.username}</h3>
          <h2 class="mt-4 text-2xl tracking-tighter">About Me</h2>
          {#if playerInfo.about_me}
            {playerInfo.about_me}
          {:else}
            <span class="text-secondary italic">None yet!</span>
          {/if}
        </div>
      </div>
      <table>
        {#snippet playerDataHeader(name: string)}
          <td class="p-4 text-right text-lg tracking-tighter">{name}</td>
        {/snippet}
        <tbody>
          <tr>
            {@render playerDataHeader("Runs")}
            <td class="border bg-bg-secondary p-4">
              {playerInfo.run_count}
            </td>
          </tr>
          <tr>
            {@render playerDataHeader("Wins")}
            <td class="border bg-bg-secondary p-4">
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
            <td class="border bg-bg-secondary p-4">
              {#if playerInfo.favourite_survivor}
                <TableSurvivor survivor={playerInfo.favourite_survivor} />
              {:else}
                <span>No data!</span>
              {/if}
            </td>
          </tr>
          <tr>
            {@render playerDataHeader("Favourite Difficulty")}
            <td class="border bg-bg-secondary p-4">
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
<h1 class="mb-4 text-center text-3xl tracking-tighter">Combined Stats</h1>

<StatsCombined {username} />

<hr class="my-8" />
<h1 class="mb-4 text-center text-3xl tracking-tighter">Survivors</h1>
<StatsSurvivors {username} />

<hr class="my-8" />
<h1 class="mb-4 text-center text-3xl tracking-tighter">Difficulties</h1>
<StatsDifficulties {username} />

<hr class="my-8" />
<h1 class="mb-4 text-center text-3xl tracking-tighter">Stages</h1>
<StatsStages {username} />

<hr class="my-8" />
<h1 class="mb-4 text-center text-3xl tracking-tighter">Artifacts</h1>
<StatsArtifacts {username} />

<hr class="my-8" />
<h1 class="mb-4 text-center text-3xl tracking-tighter">Recent Runs</h1>

{#if username}
  <TableView
    sort={{ by: "uploadTime", sort: "DESC" }}
    filter={{ player: [`@${username}`] }}
  />
{/if}
