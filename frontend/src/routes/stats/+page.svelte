<script lang="ts">
  import { api, type StatsCategory } from "$lib";
  import StatsDisplay from "$lib/StatsDisplay.svelte";
  import LoadingIndicator from "$lib/LoadingIndicator.svelte";
  import {
    ARTIFACTS,
    BODIES,
    DIFFICULTIES,
    type RunReportWithUser,
  } from "$lib/RoR2";
  import { onMount } from "svelte";
  import { BarChart, LineChart, PieChart } from "layerchart";
  import StatsSurvivors from "$lib/StatsSurvivors.svelte";

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

  let overallInfoPromise: Promise<{ runCount: number; winCount: number }> =
    $state(new Promise(() => {}));

  let statsPromise: Promise<Record<StatsCategory, RunReportWithUser>> = $state(
    new Promise(() => {}),
  );

  let difficultiesPromise: Promise<Record<string, number>> = $state(
    new Promise(() => {}),
  );

  let stagesPromise: Promise<Record<number, number>> = $state(
    new Promise(() => {}),
  );

  let artifactsPromise: Promise<Record<number, number>> = $state(
    new Promise(() => {}),
  );

  onMount(() => {
    overallInfoPromise = fetch(api("stats/overall")).then((r) => r.json());
    statsPromise = Promise.all([
      fetch(api("stats/sum")).then((r) => r.json()),
      fetch(api("stats/avg")).then((r) => r.json()),
    ]).then(([sum, avg]) => ({ SUM: sum, AVG: avg }));
    difficultiesPromise = fetch(api("stats/difficulties")).then((r) =>
      r.json(),
    );
    stagesPromise = fetch(api("stats/stages")).then((r) => r.json());
    artifactsPromise = fetch(api("stats/artifacts")).then((r) => r.json());
  });
</script>

<h1 class="text-center tracking-tighter text-3xl mb-4">Overall Info</h1>

{#await overallInfoPromise}
  <LoadingIndicator indicator text="Loading info" />
{:then info}
  <p class="text-center text-lg">
    A total of
    <b>
      {info.runCount}
    </b>
    runs have been played,
    <b>
      {info.winCount}
    </b>
    of which are wins.
  </p>
{/await}

<hr class="my-8" />
<h1 class="text-center tracking-tighter text-3xl mb-4">Combined Stats</h1>

{#await statsPromise}
  <LoadingIndicator indicator text="Loading stats" />
{:then stats}
  <StatsDisplay {stats} />
{/await}

<hr class="my-8" />
<h1 class="text-center tracking-tighter text-3xl">Survivors</h1>
<h2 class="text-center italic text-secondary mb-4">
  Who are the most popular survivors
</h2>

<StatsSurvivors />

<hr class="my-8" />
<h1 class="text-center tracking-tighter text-3xl">Difficulties</h1>
<h2 class="text-center italic text-secondary mb-4">
  What difficulties do people play
</h2>

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

<hr class="my-8" />
<h1 class="text-center tracking-tighter text-3xl">Stages</h1>
<h2 class="text-center italic text-secondary mb-4">When did each run end</h2>

{#await stagesPromise}
  <LoadingIndicator indicator text="Loading stage data" />
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

<hr class="my-8" />
<h1 class="text-center tracking-tighter text-3xl">Artifacts</h1>
<h2 class="text-center italic text-secondary mb-4">
  What artifacts do people play
</h2>

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
