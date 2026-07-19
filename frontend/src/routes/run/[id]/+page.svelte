<script lang="ts">
  import { page } from "$app/state";
  import ArtifactsDisplay from "$lib/ArtifactsDisplay.svelte";
  import {
    BODIES,
    DIFFICULTIES,
    ENDINGS,
    ENVIRONMENTS,
    formatBig,
    formatSeconds,
    ITEMS,
    SCORING_TABLE,
    sortItems,
    type RunReportWithUser,
  } from "$lib/RoR2";
  import UserDisplay from "$lib/UserDisplay.svelte";
  import { onMount } from "svelte";

  let runPromise: Promise<RunReportWithUser> = $state(new Promise(() => {}));

  onMount(async () => {
    const runId = page.params.id;
    runPromise = fetch(`/api/runs/${runId}`).then((resp) => resp.json());
  });
</script>

{#await runPromise}
  waiting
{:then run}
  <div
    style={`background-color: ${ENDINGS[run.ending].colorBg};`}
    class="border p-2"
  >
    <img
      src={`/endings/${ENDINGS[run.ending].icon}`}
      class="h-24 inline cursor-help"
      alt={ENDINGS[run.ending].name}
      title={ENDINGS[run.ending].name}
    />
    <h1
      class="inline text-4xl align-middle tracking-wide font-mono italic font-medium"
    >
      {ENDINGS[run.ending].endingMessage}
    </h1>
  </div>

  <div class="flex flex-row gap-4 mt-4">
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
      {#if run.artifacts.length > 0}
        <div class="flex flex-row justify-between items-center p-2">
          <span class="text-lg">Artifacts:</span>
          <ArtifactsDisplay
            artifacts={run.artifacts}
            class="min-w-0 flex-wrap-reverse"
          />
        </div>
      {/if}
      {#snippet score(label: string, value: string, points: string)}
        <div class="flex flex-row justify-between items-center p-2">
          <span>
            {label}:
            <span class="text-yellow-200">
              {value}
            </span>
          </span>
          <span>
            <span class="text-yellow-200">
              {points}
            </span>
            pts.
          </span>
        </div>
      {/snippet}
      {@render score(
        "Time Alive",
        formatSeconds(run.timeAliveSeconds),
        formatBig(run.timeAliveSeconds * SCORING_TABLE.timeAliveSeconds),
      )}
      {@render score(
        "Kills",
        formatBig(run.kills),
        formatBig(run.kills * SCORING_TABLE.kills),
      )}
      {@render score(
        "Minion Kills",
        formatBig(run.minionKills),
        formatBig(run.minionKills * SCORING_TABLE.minionKills),
      )}
      {@render score("Deaths", formatBig(run.deaths), "0")}
      {@render score(
        "Damage Dealt",
        formatBig(run.damageDealt),
        formatBig(run.damageDealt * SCORING_TABLE.damageDealt),
      )}
      {@render score(
        "Minion Damage Dealt",
        formatBig(run.minionDamageDealt),
        formatBig(run.minionDamageDealt * SCORING_TABLE.minionDamageDealt),
      )}
      {@render score(
        "Most Damage Dealt",
        formatBig(run.highestDamageDealt),
        formatBig(run.highestDamageDealt * SCORING_TABLE.highestDamageDealt),
      )}
      {@render score("Damage Taken", formatBig(run.damageTaken), "0")}
      {@render score(
        "Highest Level",
        formatBig(run.highestLevel),
        formatBig(run.highestLevel * SCORING_TABLE.highestLevel),
      )}
      {@render score(
        "Gold Collected",
        formatBig(run.goldCollected),
        formatBig(run.goldCollected * SCORING_TABLE.goldCollected),
      )}
      {@render score(
        "Items Collected",
        formatBig(run.itemsCollected),
        formatBig(run.itemsCollected * SCORING_TABLE.itemsCollected),
      )}
      {@render score(
        "Stages Completed",
        formatBig(run.stagesCompleted),
        formatBig(run.stagesCompleted * SCORING_TABLE.stagesCompleted),
      )}
      {@render score(
        "Purchases",
        formatBig(run.purchases),
        formatBig(run.purchases * SCORING_TABLE.purchases),
      )}
      <div class="flex flex-row justify-around items-center p-2 mt-2 text-xl">
        <span>Total</span>
        <span>
          <span class="text-yellow-200">
            {formatBig(run.score)}
          </span>
          pts.
        </span>
      </div>
    </div>
    <div class="border bg-bg-secondary w-full p-2">
      <h1 class="text-3xl text-center">Info</h1>
      <div class="flex flex-row items-center p-2 gap-4">
        <img
          src={`/bodies/${BODIES[run.survivor].icon}`}
          alt={BODIES[run.survivor].displayName}
          class="h-12 inline mr-2"
        />

        <span class="text-lg">
          Class:
          <span class="text-yellow-200">
            {BODIES[run.survivor].displayName}
          </span>
        </span>
      </div>
      <h1 class="text-2xl text-center">Items Collected</h1>
      <ul class="flex flex-row flex-wrap mt-4">
        {#each sortItems(run.items) as [itemId, itemCount] (itemId)}
          {@const item = ITEMS[Number(itemId)]}
          {#if !item.helper}
            <li class="relative">
              <img
                src={`/items/${item.icon}`}
                alt={item.displayName}
                class="w-16 inline"
                title={item.displayName}
              />
              {#if itemCount != 1}
                <p
                  class="text-xl font-bold absolute top-0 right-0 text-shadow-lg/50 font-mono"
                >
                  x{itemCount}
                </p>
              {/if}
            </li>
          {/if}
        {/each}
      </ul>
      <h1 class="text-2xl text-center mt-4">Metadata</h1>
      <div class="flex flex-row justify-between items-center p-2">
        <span>Player:</span>
        <span>
          <UserDisplay
            class="h-12"
            user={{
              displayUsername: run.userDisplayUsername,
              image: run.userImage,
              username: run.userUsername,
            }}
          />
        </span>
      </div>
      <div class="flex flex-row justify-between items-center p-2">
        <span>Started:</span>
        <span
          class="text-yellow-200"
          title={new Date(run.startTime).toString()}
        >
          {new Date(run.startTime).toLocaleString()}
        </span>
      </div>
      <div class="flex flex-row justify-between items-center p-2">
        <span>Uploaded:</span>
        <span
          class="text-yellow-200"
          title={new Date(run.uploadTime).toString()}
        >
          {new Date(run.uploadTime).toLocaleString()}
        </span>
      </div>
    </div>
  </div>

  <hr class="mt-8" />

  <div class="mt-8">
    <h1 class="text-3xl text-center">Stage History</h1>

    <div class="flex flex-row overflow-x-auto gap-2 mt-4">
      {#each run.stageHistory as stage}
        <div class="shrink-0">
          <img
            src="/environments/{ENVIRONMENTS[stage].icon}"
            alt="stage"
            class="h-32 border"
          />
          <span class="text-secondary italic block text-center mb-4">
            {ENVIRONMENTS[stage].displayName}
          </span>
        </div>
      {/each}
    </div>
  </div>

  <hr />

  <div class="mt-8">
    <h1 class="text-3xl text-center">Item History</h1>

    <table class="w-full">
      <thead>
        <tr>
          <th
            class="text-xl tracking-tight text-center font-bold px-2 relative z-2"
          >
            Item
          </th>
          <th
            class="text-xl tracking-tight text-center font-bold px-2 relative z-2"
          >
            Count
          </th>
          <th
            class="text-xl tracking-tight text-center font-bold px-2 relative z-2"
          >
            Time
          </th>
        </tr>
      </thead>
      <tbody>
        {#each run.itemHistory as itemEvent}
          <tr class="">
            <td class="border px-4 bg-bg-secondary">
              <img
                src="/items/{ITEMS[itemEvent.id].icon}"
                alt="stage"
                class="h-8 inline"
              />
              <span class="">
                {ITEMS[itemEvent.id].displayName}
              </span>
            </td>
            <td class="border px-4 bg-bg-secondary">
              <span class={itemEvent.count > 0 ? "text-success" : "text-error"}>
                {itemEvent.count > 0 ? "+" : "-"}
                {itemEvent.count.toString().replace("-", "")}
              </span>
            </td>
            <td class="border px-4 bg-bg-secondary">
              <span>{formatSeconds(itemEvent.time)}</span>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
{:catch err}
  {err}
{/await}
