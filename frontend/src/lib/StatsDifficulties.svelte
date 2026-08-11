<script lang="ts">
  import { api } from "$lib";
  import LoadingIndicator from "$lib/LoadingIndicator.svelte";
  import { DIFFICULTIES } from "$lib/RoR2";
  import { onMount } from "svelte";
  import { PieChart } from "layerchart";

  let { username }: { username?: string } = $props();

  let difficultiesPromise: Promise<Record<string, number>> = $state(
    new Promise(() => {}),
  );

  onMount(() => {
    difficultiesPromise = fetch(
      username
        ? api(`stats/difficulties?${new URLSearchParams({ username })}`)
        : api("stats/difficulties"),
    ).then((r) => r.json());
  });
</script>

{#await difficultiesPromise}
  <LoadingIndicator indicator text="Loading difficulty data" />
{:then difficulties}
  <PieChart
    data={Object.entries(difficulties).map(([diff, count]) => ({
      diff: DIFFICULTIES[diff]?.displayName ?? diff,
      color: DIFFICULTIES[diff]?.color ?? "gray",
      count,
    }))}
    key="diff"
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
