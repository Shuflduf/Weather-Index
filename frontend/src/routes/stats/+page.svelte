<script lang="ts">
  import { api } from "$lib";
  import LoadingIndicator from "$lib/LoadingIndicator.svelte";
  import { onMount } from "svelte";
  import StatsSurvivors from "$lib/StatsSurvivors.svelte";
  import StatsDifficulties from "$lib/StatsDifficulties.svelte";
  import StatsStages from "$lib/StatsStages.svelte";
  import StatsArtifacts from "$lib/StatsArtifacts.svelte";
  import StatsCombined from "$lib/StatsCombined.svelte";

  let overallInfoPromise: Promise<{ runCount: number; winCount: number }> =
    $state(new Promise(() => {}));

  onMount(() => {
    overallInfoPromise = fetch(api("stats/overall")).then((r) => r.json());
  });
</script>

<h1 class="text-center tracking-tighter text-3xl mb-4">Overall Info</h1>

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
    of which are wins, for a total of TODO hours.
  </p>
{/await}

<hr class="my-8" />
<h1 class="text-center tracking-tighter text-3xl mb-4">Combined Stats</h1>

<StatsCombined />

<hr class="my-8" />
<h1 class="text-center tracking-tighter text-3xl">Survivors</h1>
<h2 class="text-center italic text-secondary mb-4">
  Who are the most popular survivors
</h2>

<StatsSurvivors />

<hr class="my-8" />
<h1 class="text-center tracking-tighter text-3xl">Difficulties</h1>
<h2 class="text-center italic text-secondary mb-4">
  What difficulties do people play
</h2>

<StatsDifficulties />

<hr class="my-8" />
<h1 class="text-center tracking-tighter text-3xl">Stages</h1>
<h2 class="text-center italic text-secondary mb-4">When did each run end</h2>

<StatsStages />

<hr class="my-8" />
<h1 class="text-center tracking-tighter text-3xl">Artifacts</h1>
<h2 class="text-center italic text-secondary mb-4">
  What artifacts do people play
</h2>

<StatsArtifacts />
