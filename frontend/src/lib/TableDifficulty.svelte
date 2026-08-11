<script lang="ts">
  import { DIFFICULTIES } from "$lib/RoR2";

  let { difficulty }: { difficulty: string } = $props();
</script>

{#if DIFFICULTIES[difficulty]}
  <img
    src={`/difficulties/${DIFFICULTIES[difficulty].icon}`}
    alt={difficulty}
    class="mr-2 inline h-12"
  />
  <span class="text-lg">
    {DIFFICULTIES[difficulty].displayName}
  </span>
{:else}
  <span class="text-lg">{difficulty}</span>
{/if}
