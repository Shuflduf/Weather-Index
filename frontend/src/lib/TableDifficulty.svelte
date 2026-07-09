<script lang="ts">
  import { DIFFICULTIES } from "$lib/RoR2";

  let { difficulty }: { difficulty: string } = $props();
</script>

<img
  src={`/difficulties/${DIFFICULTIES[difficulty].icon}`}
  alt={difficulty}
  class="h-12 inline mr-2"
/>
<span class="text-lg">
  {DIFFICULTIES[difficulty].displayName}
</span>
