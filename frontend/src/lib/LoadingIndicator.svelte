<script lang="ts">
  import { LoaderCircle } from "@lucide/svelte";
  let { indicator = false, text }: { indicator?: boolean; text: string } =
    $props();
</script>

<div class="mt-4 flex flex-row justify-center gap-4 text-secondary italic">
  {#if indicator}
    <div class="w-min animate-spin">
      <LoaderCircle />
    </div>
  {/if}
  {text}
</div>
