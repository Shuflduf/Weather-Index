<script lang="ts">
  import { onMount } from "svelte";
  import { ITEMS, type Item } from "$lib/Items";
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
      </tr>
    </thead>
    <tbody>
      {#each runs as run}
        <tr class="border bg-bg-secondary">
          <td class="border p-4">{run.id}</td>
          <td class="border p-4">
            <UserDisplay
              user={{
                username: run.user_username,
                image: run.user_image,
                displayName: null,
              }}
            />
          </td>
          <td class="border p-4">{run.survivor}</td>
          <td class="border p-4">{run.ending}</td>
          <td class="border p-4">{run.difficulty}</td>
          <td class="border p-4">
            {Object.entries(run.items)
              .filter(([key, value]) => !ITEMS[Number(key)].helper)
              .reduce((accum, [_, count]) => accum + (count as number), 0)}
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>
