<script lang="ts">
  import { onMount } from "svelte";
  import {
    BODIES,
    countRealItems,
    DIFFICULTIES,
    ENDINGS,
    ITEMS,
    type Item,
  } from "$lib/RoR2";
  import UserDisplay from "$lib/UserDisplay.svelte";

  let runs: any[] = $state([]);
  onMount(async () => {
    runs = await (await fetch("/api/runs")).json();
    console.log(runs);
  });
</script>

<div class="w-full p-8">
  <table class="text-primary w-full">
    <thead class="text-xl tracking-tight text-center font-bold h-12">
      <tr>
        <td>ID</td>
        <td>Player</td>
        <td>Survivor</td>
        <td>Ending</td>
        <td>Difficulty</td>
        <td>Items</td>
        <td>Score</td>
      </tr>
    </thead>
    <tbody>
      {#each runs as run}
        <tr class="border bg-bg-secondary">
          <td class="border p-4"><a href={`/run/${run.id}`}>{run.id}</a></td>
          <td class="border p-4">
            <UserDisplay
              class="h-12"
              user={{
                username: run.user_username,
                image: run.user_image,
                displayName: null,
              }}
            />
          </td>
          <td class="border p-4">
            <img
              src={`/bodies/${BODIES[run.survivor].icon}`}
              alt={BODIES[run.survivor].displayName}
              class="h-12 inline rounded-full mr-2"
            />
            <span class="text-lg">
              {BODIES[run.survivor].displayName}
            </span>
          </td>
          <td
            class="border p-4"
            style={`background-color: ${ENDINGS[run.ending].colorBg};`}
          >
            <img
              src={`/endings/${ENDINGS[run.ending].icon}`}
              alt={run.ending}
              class="h-12 inline mr-2"
            />
            <span class="text-shadow-lg text-lg">
              {ENDINGS[run.ending].displayName}
            </span>
          </td>
          <td class="border p-4">
            <img
              src={`/difficulties/${DIFFICULTIES[run.difficulty].icon}`}
              alt={run.difficulty}
              class="h-12 inline mr-2"
            />
            <span class="text-lg">
              {DIFFICULTIES[run.difficulty].displayName}
            </span>
          </td>
          <td class="border p-4">{countRealItems(run.items)}</td>
          <td class="border p-4">{run.score}</td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>
