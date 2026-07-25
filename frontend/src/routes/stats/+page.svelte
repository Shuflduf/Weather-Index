<script lang="ts">
  import { api, type GlobalStatsCategory } from "$lib";
  import GlobalStatsDisplay from "$lib/GlobalStatsDisplay.svelte";
  import LoadingIndicator from "$lib/LoadingIndicator.svelte";
  import type { RunReportWithUser } from "$lib/RoR2";
  import { onMount } from "svelte";

  // let sumStatsPromise: Promise<RunReportWithUser> = $state(
  //   new Promise(() => {}),
  // );
  // let avgStatsPromise: Promise<RunReportWithUser> = $state(
  //   new Promise(() => {}),
  // );

  let statsPromise: Promise<Record<GlobalStatsCategory, RunReportWithUser>> =
    $state(new Promise(() => {}));

  onMount(() => {
    statsPromise = Promise.all([
      fetch(api("global/sum")).then((r) => r.json()),
      fetch(api("global/avg")).then((r) => r.json()),
    ]).then(([sum, avg]) => ({ SUM: sum, AVG: avg }));
  });
</script>

{#await statsPromise}
  <LoadingIndicator indicator text="Loading stats" />
{:then stats}
  <GlobalStatsDisplay {stats} />
{/await}
