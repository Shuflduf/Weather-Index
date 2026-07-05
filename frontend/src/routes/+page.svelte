<script lang="ts">
  import { onMount } from "svelte";
  import { BODIES, ITEMS, type Item } from "$lib/RoR2";
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
            <span>
              {BODIES[run.survivor].displayName}
            </span>
          </td>
          <td class="border p-4">{run.ending}</td>
          <td class="border p-4">{run.difficulty}</td>
          <td class="border p-4">{run.items_collected}</td>
          <td class="border p-4">{run.score}</td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>
