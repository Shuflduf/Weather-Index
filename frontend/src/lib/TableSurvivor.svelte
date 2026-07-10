<script lang="ts">
  import { BODIES } from "$lib/RoR2";
  import { onMount } from "svelte";
  let { survivor }: { survivor: string } = $props();
  onMount(() => console.log(survivor));
</script>

<img
  src={`/bodies/${BODIES[survivor].icon}`}
  alt={BODIES[survivor].displayName}
  class="h-12 inline mr-2"
/>
<span class="text-lg">
  {BODIES[survivor].displayName}
</span>
