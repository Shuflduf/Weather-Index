<script lang="ts">
  import { onMount } from "svelte";
  import { api } from "$lib";
  import { PieChart } from "layerchart";
  import LoadingIndicator from "./LoadingIndicator.svelte";
  import { BODIES } from "./RoR2";

  let { username }: { username?: string } = $props();

  let survivorsPromise: Promise<Record<string, number>> = $state(
    new Promise(() => {}),
  );

  onMount(() => {
    survivorsPromise = fetch(
      username
        ? api(`stats/survivors?${new URLSearchParams({ username })}`)
        : api("stats/survivors"),
    ).then((r) => r.json());
  });
</script>

{#await survivorsPromise}
  <LoadingIndicator indicator text="Loading survivor data" />
{:then survivors}
  <PieChart
    data={Object.entries(survivors).map(([survivor, count]) => ({
      survivor: BODIES[survivor].displayName,
      color: BODIES[survivor].survivorColor ?? "black",
      count,
    }))}
    key="survivor"
    value="count"
    height={300}
    props={{
      pie: { motion: "spring" },
    }}
    legend={{
      orientation: "vertical",
      placement: "right",
      classes: { label: "text-xs", swatch: "h-2", items: "gap-0" },
    }}
    c="color"
  />
{/await}
