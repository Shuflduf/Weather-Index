<script lang="ts">
  import { page } from "$app/state";
  import { api } from "$lib";
  import ArtifactsDisplay from "$lib/ArtifactsDisplay.svelte";
  import LoadingIndicator from "$lib/LoadingIndicator.svelte";
  import {
    BODIES,
    DIFFICULTIES,
    ENDINGS,
    ENVIRONMENTS,
    EQUIPMENTS,
    formatBig,
    formatSeconds,
    ITEMS,
    SCORING_TABLE,
    SKILLS,
    sortItems,
    sortStageInteractables,
    type RunReportWithUser,
  } from "$lib/RoR2";
  import UserDisplay from "$lib/UserDisplay.svelte";
  import { onMount } from "svelte";
  import StageItemContextMenu from "./StageItemContextMenu.svelte";

  let runPromise: Promise<RunReportWithUser> = $state(new Promise(() => {}));
  let contextMenu: any = $state();

  onMount(async () => {
    const runId = page.params.id;
    runPromise = fetch(api(`runs/${runId}`)).then((resp) => resp.json());
  });
</script>

<StageItemContextMenu bind:this={contextMenu} />

{#await runPromise}
  <LoadingIndicator indicator text="Loading run report!" />
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
      <div class="flex flex-row items-center justify-between p-2">
        <div>
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

        <div>
          {#each run.skills as skill}
            <img
              src="/skills/{SKILLS[skill].icon}"
              alt={SKILLS[skill].displayName}
              class="inline h-12"
              title={SKILLS[skill].displayName}
            />
          {/each}
        </div>
      </div>
      {#if run.equipment}
        <div>
          <img
            src="/equipment/{EQUIPMENTS[run.equipment].icon}"
            alt={EQUIPMENTS[run.equipment].displayName}
            title={EQUIPMENTS[run.equipment].displayName}
            class="h-12 inline mr-2"
          />
          <span class="text-lg">
            Equipment:
            <span class="text-yellow-200">
              {EQUIPMENTS[run.equipment].displayName}
            </span>
          </span>
        </div>
      {/if}
      <h1 class="text-2xl text-center">Items Collected</h1>
      <ul class="flex flex-row flex-wrap mt-4">
        {#each sortItems(run.items) as [itemId, itemCount] (itemId)}
          {@const item = ITEMS[Number(itemId)]}
          {#if !item.helper}
            <li class="relative">
              <img
                src={`/items/${item.icon}`}
                alt={item.displayName}
                class="size-16 inline"
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

  <hr class="my-8" />

  <h1 class="text-3xl text-center mb-4">Stage History</h1>

  <div class="flex flex-col gap-8">
    {#each run.stageHistory as stage}
      <div class="flex flex-row gap-4">
        <div>
          <img
            src="/environments/{ENVIRONMENTS[stage.name].icon}"
            alt="stage"
            class="h-32 border"
          />
          <span class="text-secondary italic block text-center mb-4">
            {ENVIRONMENTS[stage.name].displayName}
          </span>
        </div>
        <div class="flex flex-row flex-wrap flex-1">
          {#each stage.interactables.toSorted(sortStageInteractables) as interactable}
            {@const table =
              interactable.name == "EQUIPMENTBARREL_NAME" ? EQUIPMENTS : ITEMS}
            {@const imgPath =
              interactable.name == "EQUIPMENTBARREL_NAME"
                ? "equipment"
                : "items"}
            <img
              onmouseenter={(e) => contextMenu?.show(e, interactable)}
              onmousemove={(e) => contextMenu?.update(e)}
              onmouseleave={(_) => contextMenu.hide()}
              src="/{imgPath}/{table[interactable.item].icon}"
              alt={table[interactable.item].displayName}
              class="size-16"
              style="filter: grayscale({interactable.time != null ? 0 : 1});"
            />
          {/each}
        </div>
      </div>
    {/each}
  </div>

  <hr class="my-8" />

  <h1 class="text-3xl text-center mb-4">Item History</h1>

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
        {#if !ITEMS[itemEvent.id].helper}
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
        {/if}
      {/each}
    </tbody>
  </table>

  <hr class="my-8" />

  <h1 class="text-3xl text-center mb-4">Equipment History</h1>

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
          Time
        </th>
      </tr>
    </thead>
    <tbody>
      {#each run.equipmentHistory as equipmentEvent}
        <tr class="">
          <td class="border px-4 bg-bg-secondary">
            <img
              src="/equipment/{EQUIPMENTS[equipmentEvent.id].icon}"
              alt="stage"
              class="h-12 inline"
            />
            <span class="">
              {EQUIPMENTS[equipmentEvent.id].displayName}
            </span>
          </td>
          <td class="border px-4 bg-bg-secondary">
            <span>{formatSeconds(equipmentEvent.time)}</span>
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
{:catch err}
  {err}
{/await}
