<script lang="ts">
  import { api } from "$lib";
  import GlobalStatsDisplay from "$lib/GlobalStatsDisplay.svelte";
  import LoadingIndicator from "$lib/LoadingIndicator.svelte";
  import type { RunReportWithUser } from "$lib/RoR2";
  import { onMount } from "svelte";

  let statsPromise: Promise<RunReportWithUser> = $state(new Promise(() => {}));

  onMount(() => {
    statsPromise = fetch(api("global")).then((r) => r.json());
  });
</script>

{#await statsPromise}
  <LoadingIndicator indicator text="Aggregating stats" />
{:then stats}
  <GlobalStatsDisplay {stats} />
{/await}
