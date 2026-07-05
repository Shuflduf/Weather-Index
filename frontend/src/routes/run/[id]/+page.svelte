<script lang="ts">
  import { page } from "$app/state";
  import { ENDINGS, ITEMS } from "$lib/RoR2";
  import { onMount } from "svelte";

  let runPromise: Promise<any> = $state(new Promise(() => {}));

  onMount(async () => {
    const runId = page.params.id;
    runPromise = fetch(`/api/runs/${runId}`).then((resp) => resp.json());
  });
</script>

<div class="text-primary">
  {#await runPromise}
    waiting
  {:then run}
    <div
      style={`background-color: ${ENDINGS[run.ending].colorBg};`}
      class="m-4 border"
    >
      <img
        src={`/endings/${ENDINGS[run.ending].icon}`}
        class="h-24 inline cursor-help"
        alt={ENDINGS[run.ending].name}
        title={ENDINGS[run.ending].name}
      />
      <h1 class="inline text-4xl align-middle tracking-wide italic font-medium">
        {ENDINGS[run.ending].endingMessage}
      </h1>
    </div>
    <ul class="flex flex-row">
      {#each Object.entries(run.items) as [itemId, itemCount]}
        {@const item = ITEMS[Number(itemId)]}
        {#if !item.helper}
          <li class="relative">
            <img
              src={`/items/${item.icon}`}
              alt={item.displayName}
              class="w-16 inline"
              title={item.displayName}
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
  {:catch err}
    {err}
  {/await}
</div>
