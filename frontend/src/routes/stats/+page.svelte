<script lang="ts">
  import { api } from "$lib";
  import GlobalStatsDisplay from "$lib/GlobalStatsDisplay.svelte";
  import LoadingIndicator from "$lib/LoadingIndicator.svelte";
  import type { RunReportWithUser } from "$lib/RoR2";
  import { onMount } from "svelte";

  let sumStatsPromise: Promise<RunReportWithUser> = $state(
    new Promise(() => {}),
  );
  let avgStatsPromise: Promise<RunReportWithUser> = $state(
    new Promise(() => {}),
  );

  onMount(() => {
    sumStatsPromise = fetch(api("global/sum")).then((r) => r.json());
    avgStatsPromise = fetch(api("global/avg")).then((r) => r.json());
  });
</script>

{#await sumStatsPromise}
  <LoadingIndicator indicator text="Loading stats" />
{:then stats}
  <GlobalStatsDisplay {stats} />
{/await}
