<script lang="ts">
  import { onMount } from "svelte";
  import { api } from "$lib";
  import { BarChart, LineChart } from "layerchart";
  import LoadingIndicator from "./LoadingIndicator.svelte";
  import { ARTIFACTS } from "./RoR2";

  let { username }: { username?: string } = $props();

  let artifactsPromise: Promise<Record<number, number>> = $state(
    new Promise(() => {}),
  );

  onMount(() => {
    artifactsPromise = fetch(
      username
        ? api(`stats/artifacts?${new URLSearchParams({ username })}`)
        : api("stats/artifacts"),
    ).then((r) => r.json());
  });
</script>

{#await artifactsPromise}
  <LoadingIndicator indicator text="Loading artifacts data" />
{:then artifacts}
  <BarChart
    data={Object.entries(artifacts).map(([artif, count]) => ({
      artif: ARTIFACTS[artif].displayName,
      count,
    }))}
    x="artif"
    y="count"
    height={300}
    axis={{ classes: { tickLabel: "stroke-none" } }}
  />
{/await}
