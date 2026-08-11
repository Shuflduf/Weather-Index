<script lang="ts">
  import { ARTIFACTS } from "$lib/RoR2";

  let {
    artifacts,
    class: className = "",
  }: { artifacts: string[]; class: string } = $props();
</script>

<div class="flex max-h-16 flex-col {className}">
  {#each artifacts as artifact (artifact)}
    {#if ARTIFACTS[artifact]}
      <img
        src={`/artifacts/${ARTIFACTS[artifact].icon}`}
        alt={ARTIFACTS[artifact].displayName}
        class="mr-2 inline-block h-8"
      />
    {:else}
      <span class="mr-2 inline-block text-xs">{artifact}</span>
    {/if}
  {/each}
</div>
