<script lang="ts">
  import { ARTIFACTS } from "$lib/RoR2";

  let {
    artifacts,
    class: className = "",
  }: { artifacts: string[]; class: string } = $props();
</script>

<div class="flex flex-col max-h-16 {className}">
  {#each artifacts as artifact}
    <img
      src={`/artifacts/${ARTIFACTS[artifact].icon}`}
      alt={ARTIFACTS[artifact].displayName}
      class="h-8 inline-block mr-2"
    />
  {/each}
</div>
