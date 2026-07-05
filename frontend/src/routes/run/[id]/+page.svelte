<script lang="ts">
  import { page } from "$app/state";
  import { ITEMS } from "$lib/Items";
  import { onMount } from "svelte";

  let run: any = $state();

  onMount(async () => {
    const runId = page.params.id;
    run = await (await fetch(`/api/runs/${runId}`)).json();
    console.log(run);
  });
</script>

<div class="text-primary">
  {#if run}
    <ul class="flex flex-row">
      {#each Object.entries(run.items) as [itemId, itemCount]}
        {@const item = ITEMS[Number(itemId)]}
        {#if !item.helper}
          <li class="relative">
            <img
              src={`/items/${item.icon}`}
              alt={item.displayName}
              class="w-16 inline"
            />
            <p
              class="font-ror2 text-2xl absolute top-0 right-0 text-shadow-lg/50"
            >
              x{itemCount}
            </p>
          </li>
        {/if}
      {/each}
    </ul>
  {/if}
</div>
