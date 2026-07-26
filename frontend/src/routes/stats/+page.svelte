<script lang="ts">
  import { api, type StatsCategory } from "$lib";
  import StatsDisplay from "$lib/StatsDisplay.svelte";
  import LoadingIndicator from "$lib/LoadingIndicator.svelte";
  import type { RunReportWithUser } from "$lib/RoR2";
  import { onMount } from "svelte";

  // let sumStatsPromise: Promise<RunReportWithUser> = $state(
  //   new Promise(() => {}),
  // );
  // let avgStatsPromise: Promise<RunReportWithUser> = $state(
  //   new Promise(() => {}),
  // );

  let statsPromise: Promise<Record<StatsCategory, RunReportWithUser>> = $state(
    new Promise(() => {}),
  );

  onMount(() => {
    statsPromise = Promise.all([
      fetch(api("stats/sum")).then((r) => r.json()),
      fetch(api("stats/avg")).then((r) => r.json()),
    ]).then(([sum, avg]) => ({ SUM: sum, AVG: avg }));
  });
</script>

{#await statsPromise}
  <LoadingIndicator indicator text="Loading stats" />
{:then stats}
  <StatsDisplay {stats} />
{/await}
