<script lang="ts">
  import { api } from "$lib";
  import LoadingIndicator from "$lib/LoadingIndicator.svelte";
  import { onMount } from "svelte";
  import StatsSurvivors from "$lib/StatsSurvivors.svelte";
  import StatsDifficulties from "$lib/StatsDifficulties.svelte";
  import StatsStages from "$lib/StatsStages.svelte";
  import StatsArtifacts from "$lib/StatsArtifacts.svelte";
  import StatsCombined from "$lib/StatsCombined.svelte";
  import { formatSeconds } from "$lib/RoR2";

  let overallInfoPromise: Promise<{
    runCount: number;
    winCount: number;
    playtimeSeconds: number;
  }> = $state(new Promise(() => {}));

  onMount(() => {
    overallInfoPromise = fetch(api("stats/overall")).then((r) => r.json());
  });
</script>

<svelte:head>
  <title>WI | Stats</title>
</svelte:head>

<h1 class="mb-4 text-center text-3xl tracking-tighter">Overall Info</h1>

{#await overallInfoPromise}
  <LoadingIndicator indicator text="Loading info" />
{:then info}
  <p class="text-center text-lg">
    A total of
    <b>
      {info.runCount}
    </b>
    runs have been played,
    <b>
      {info.winCount}
    </b>
    of which are wins, for a total of
    <b>
      {formatSeconds(info.playtimeSeconds)}
    </b>
    of playtime.
  </p>
{/await}

<hr class="my-8" />
<h1 class="mb-4 text-center text-3xl tracking-tighter">Combined Stats</h1>

<StatsCombined />

<hr class="my-8" />
<h1 class="text-center text-3xl tracking-tighter">Survivors</h1>
<h2 class="mb-4 text-center text-secondary italic">
  Who are the most popular survivors
</h2>

<StatsSurvivors />

<hr class="my-8" />
<h1 class="text-center text-3xl tracking-tighter">Difficulties</h1>
<h2 class="mb-4 text-center text-secondary italic">
  What difficulties do people play
</h2>

<StatsDifficulties />

<hr class="my-8" />
<h1 class="text-center text-3xl tracking-tighter">Stages</h1>
<h2 class="mb-4 text-center text-secondary italic">When did each run end</h2>

<StatsStages />

<hr class="my-8" />
<h1 class="text-center text-3xl tracking-tighter">Artifacts</h1>
<h2 class="mb-4 text-center text-secondary italic">
  What artifacts do people play
</h2>

<StatsArtifacts />
