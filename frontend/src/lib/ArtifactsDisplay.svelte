<script lang="ts">
  import { ARTIFACTS } from "$lib/RoR2";

  let { artifacts }: { artifacts: string[] } = $props();
</script>

<div class="flex flex-col flex-wrap max-h-16 overflow-x-auto w-full">
  {#each artifacts as artifact}
    <img
      src={`/artifacts/${ARTIFACTS[artifact].icon}`}
      alt={ARTIFACTS[artifact].displayName}
      class="h-8 inline-block mr-2"
    />
  {/each}
</div>
