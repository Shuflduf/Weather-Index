<script lang="ts">
  import { onMount } from "svelte";
  import { api } from "$lib";
  import { PieChart } from "layerchart";
  import LoadingIndicator from "./LoadingIndicator.svelte";
  import { BODIES } from "./RoR2";
  const SURVIVOR_COLOURS = [
    "var(--color-red-400)",
    "var(--color-orange-400)",
    "var(--color-amber-400)",
    "var(--color-yellow-400)",
    "var(--color-lime-400)",
    "var(--color-green-400)",
    "var(--color-emerald-400)",
    "var(--color-teal-400)",
    "var(--color-cyan-400)",
    "var(--color-sky-400)",
    "var(--color-blue-400)",
    "var(--color-indigo-400)",
    "var(--color-violet-400)",
    "var(--color-purple-400)",
    "var(--color-fuchsia-400)",
    "var(--color-pink-400)",
    "var(--color-rose-400)",
    "var(--color-slate-200)",
  ];

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
    cRange={SURVIVOR_COLOURS}
  />
{/await}
