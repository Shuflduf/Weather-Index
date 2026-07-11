<script lang="ts">
  import { BODIES } from "$lib/RoR2";
  let { survivor }: { survivor: string } = $props();
</script>

<img
  src={`/bodies/${BODIES[survivor].icon}`}
  alt={BODIES[survivor].displayName}
  class="h-12 inline mr-2"
/>
<span class="text-lg">
  {BODIES[survivor].displayName}
</span>
