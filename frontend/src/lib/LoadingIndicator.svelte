<script lang="ts">
  import { LoaderCircle } from "@lucide/svelte";
  let { indicator = false, text }: { indicator?: boolean; text: string } =
    $props();
</script>

<div class="flex flex-row justify-center gap-4 italic mt-4 text-secondary">
  {#if indicator}
    <div class="animate-spin w-min">
      <LoaderCircle />
    </div>
  {/if}
  {text}
</div>
