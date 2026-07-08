<script lang="ts">
  import { onMount } from "svelte";

  onMount(() => {
    let userPromise = fetch("/");
  });
</script>
