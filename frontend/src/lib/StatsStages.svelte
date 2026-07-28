<script lang="ts">
  import { onMount } from "svelte";
  import { api } from "$lib";
  import { LineChart } from "layerchart";
  import LoadingIndicator from "./LoadingIndicator.svelte";

  let { username }: { username?: string } = $props();

  let stagesPromise: Promise<Record<number, number>> = $state(
    new Promise(() => {}),
  );

  onMount(() => {
    stagesPromise = fetch(
      username
        ? api(`stats/stages?${new URLSearchParams({ username })}`)
        : api("stats/stages"),
    ).then((r) => r.json());
  });
</script>

{#await stagesPromise}
  <LoadingIndicator indicator text="Loading stages data" />
{:then stages}
  <LineChart
    data={Object.entries(stages).map(([stage, count]) => ({
      stage,
      count,
    }))}
    x="stage"
    y="count"
    height={300}
    axis={{
      classes: {
        tickLabel: "stroke-none",
      },
    }}
  />
{/await}
