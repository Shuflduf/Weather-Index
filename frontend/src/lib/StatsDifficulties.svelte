<script lang="ts">
  import { api } from "$lib";
  import LoadingIndicator from "$lib/LoadingIndicator.svelte";
  import { DIFFICULTIES } from "$lib/RoR2";
  import { onMount } from "svelte";
  import { PieChart } from "layerchart";

  const DIFFICULTY_COLOURS = [
    "var(--color-green-400)",
    "var(--color-orange-400)",
    "var(--color-red-400)",
    "var(--color-slate-100)",
    "var(--color-slate-200)",
    "var(--color-slate-300)",
    "var(--color-slate-400)",
    "var(--color-slate-500)",
    "var(--color-slate-600)",
    "var(--color-slate-700)",
    "var(--color-slate-800)",
  ];

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
      diff: DIFFICULTIES[diff].displayName,
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
    cRange={DIFFICULTY_COLOURS}
  />
{/await}
