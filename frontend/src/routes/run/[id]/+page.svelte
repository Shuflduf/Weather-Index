<script lang="ts">
  import { page } from "$app/state";
  import {
    DIFFICULTIES,
    ENDINGS,
    formatBig,
    formatSeconds,
    ITEMS,
    SCORING_TABLE,
  } from "$lib/RoR2";
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
      class="m-4 border mb-0"
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
    <div class="flex flex-row gap-4 m-4">
      <div class="border bg-bg-secondary w-full p-2">
        <h1 class="text-3xl text-center">Stats</h1>
        <div class="flex flex-row justify-between items-center p-2">
          <span class="text-lg">
            {DIFFICULTIES[run.difficulty].displayName}
          </span>
          <img
            src={`/difficulties/${DIFFICULTIES[run.difficulty].icon}`}
            alt={run.difficulty}
            class="h-12 inline mr-2"
          />
        </div>
        <div class="flex flex-row justify-between items-center p-2">
          <span>
            Time Alive: <span class="text-yellow-200">
              {formatSeconds(run.time_alive_seconds)}
            </span>
          </span>
          <span>
            <span class="text-yellow-200">
              {formatBig(
                run.time_alive_seconds * SCORING_TABLE.timeAliveSeconds,
              )}
            </span>
            pts
          </span>
        </div>
        <div class="flex flex-row justify-between items-center p-2">
          <span>
            Kills: <span class="text-yellow-200">
              {formatBig(run.kills)}
            </span>
          </span>
          <span>
            <span class="text-yellow-200">
              {formatBig(run.kills * SCORING_TABLE.kills)}
            </span>
            pts
          </span>
        </div>
      </div>
      <div class="bg-red-500 w-full">
        <h1>Info</h1>
      </div>
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
