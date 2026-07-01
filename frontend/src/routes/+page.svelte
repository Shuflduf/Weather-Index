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

{#each runs as run}
  <span class="text-primary">{JSON.stringify(run)}</span>
{/each}
