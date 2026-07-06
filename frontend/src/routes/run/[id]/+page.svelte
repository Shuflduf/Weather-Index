<script lang="ts">
  import { page } from "$app/state";
  import {
    BODIES,
    DIFFICULTIES,
    ENDINGS,
    formatBig,
    formatSeconds,
    ITEMS,
    SCORING_TABLE,
    sortItems,
  } from "$lib/RoR2";
  import UserDisplay from "$lib/UserDisplay.svelte";
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
      class="m-4 border mb-0 p-2"
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
          formatSeconds(run.time_alive_seconds),
          formatBig(run.time_alive_seconds * SCORING_TABLE.timeAliveSeconds),
        )}
        {@render score(
          "Kills",
          formatBig(run.kills),
          formatBig(run.kills * SCORING_TABLE.kills),
        )}
        {@render score(
          "Minion Kills",
          formatBig(run.minion_kills),
          formatBig(run.minion_kills * SCORING_TABLE.minionKills),
        )}
        {@render score("Deaths", formatBig(run.deaths), "0")}
        {@render score(
          "Damage Dealt",
          formatBig(run.damage_dealt),
          formatBig(run.damage_dealt * SCORING_TABLE.damageDealt),
        )}
        {@render score(
          "Minion Damage Dealt",
          formatBig(run.minion_damage_dealt),
          formatBig(run.minion_damage_dealt * SCORING_TABLE.minionDamageDealt),
        )}
        {@render score(
          "Most Damage Dealt",
          formatBig(run.highest_damage_dealt),
          formatBig(
            run.highest_damage_dealt * SCORING_TABLE.highestDamageDealt,
          ),
        )}
        {@render score("Damage Taken", formatBig(run.damage_taken), "0")}
        {@render score(
          "Highest Level",
          formatBig(run.highest_level),
          formatBig(run.highest_level * SCORING_TABLE.highestLevel),
        )}
        {@render score(
          "Gold Collected",
          formatBig(run.gold_collected),
          formatBig(run.gold_collected * SCORING_TABLE.goldCollected),
        )}
        {@render score(
          "Items Collected",
          formatBig(run.items_collected),
          formatBig(run.items_collected * SCORING_TABLE.itemsCollected),
        )}
        {@render score(
          "Stages Completed",
          formatBig(run.stages_completed),
          formatBig(run.stages_completed * SCORING_TABLE.stagesCompleted),
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
          {#each sortItems(run.items) as [itemId, itemCount]}
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
                    class="text-xl font-bold absolute top-0 right-0 text-shadow-lg/50"
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
                displayName: null,
                image: run.user_image,
                username: run.user_username,
              }}
            />
          </span>
        </div>
        <div class="flex flex-row justify-between items-center p-2">
          <span>Started:</span>
          <span class="text-yellow-200">
            {new Date(run.start_time).toLocaleString()}
          </span>
        </div>
        <div class="flex flex-row justify-between items-center p-2">
          <span>Uploaded:</span>
          <span class="text-yellow-200">
            {new Date(run.upload_time).toLocaleString()}
          </span>
        </div>
      </div>
    </div>
  {:catch err}
    {err}
  {/await}
</div>
