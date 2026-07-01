<script lang="ts">
  import Navbar from "$lib/Navbar.svelte";
  import { onMount } from "svelte";

  let runs = $state([]);
  onMount(async () => {
    runs = await (await fetch("/api/get-run-reports")).json();
    console.log(runs);
  });
</script>

<Navbar />

<div class="w-full p-8">
  <table class="text-primary w-full">
    <thead class="text-xl tracking-tight text-center font-bold">
      <tr>
        <td>ID</td>
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
          <td class="border p-4">{run.survivor}</td>
          <td class="border p-4">{run.ending}</td>
          <td class="border p-4">{run.difficulty}</td>
          <td class="border p-4">{run.items_collected}</td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>
