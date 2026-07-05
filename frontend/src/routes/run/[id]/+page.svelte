<script lang="ts">
  import { page } from "$app/state";
  import { onMount } from "svelte";

  let run = $state();

  onMount(async () => {
    const runId = page.params.id;
    run = await (await fetch(`/api/runs/${runId}`)).json();
    console.log(run);
  });
</script>
