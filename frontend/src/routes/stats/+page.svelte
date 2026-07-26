<script lang="ts">
  import { api, type StatsCategory } from "$lib";
  import StatsDisplay from "$lib/StatsDisplay.svelte";
  import LoadingIndicator from "$lib/LoadingIndicator.svelte";
  import { BODIES, type Body, type RunReportWithUser } from "$lib/RoR2";
  import { onMount } from "svelte";
  import { PieChart } from "layerchart";
  import { MotorbikeIcon } from "@lucide/svelte";

  // let sumStatsPromise: Promise<RunReportWithUser> = $state(
  //   new Promise(() => {}),
  // );
  // let avgStatsPromise: Promise<RunReportWithUser> = $state(
  //   new Promise(() => {}),
  // );

  let statsPromise: Promise<Record<StatsCategory, RunReportWithUser>> = $state(
    new Promise(() => {}),
  );

  let survivorsPromise: Promise<Record<string, Body>> = $state(
    new Promise(() => {}),
  );
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

  onMount(() => {
    statsPromise = Promise.all([
      fetch(api("stats/sum")).then((r) => r.json()),
      fetch(api("stats/avg")).then((r) => r.json()),
    ]).then(([sum, avg]) => ({ SUM: sum, AVG: avg }));
    survivorsPromise = fetch(api("stats/survivors")).then((r) => r.json());
  });
</script>

<h1 class="text-center tracking-tighter text-3xl mb-4">Combined Stats</h1>

{#await statsPromise}
  <LoadingIndicator indicator text="Loading stats" />
{:then stats}
  <StatsDisplay {stats} />
{/await}

<hr class="my-8" />
<h1 class="text-center tracking-tighter text-3xl mb-4">Survivors</h1>

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
    legend={{ orientation: "vertical", placement: "right" }}
    cRange={SURVIVOR_COLOURS}
  />
{/await}
