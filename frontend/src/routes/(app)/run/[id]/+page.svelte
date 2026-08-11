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

<svelte:head>
  <title>WI | Run #{page.params.id}</title>
</svelte:head>

<StageItemContextMenu bind:this={contextMenu} />

{#await runPromise}
  <LoadingIndicator indicator text="Loading run report!" />
{:then run}
  <div
    style={`background-color: ${
      ENDINGS[run.ending] ? ENDINGS[run.ending].colorBg : ""
    };`}
    class="border p-2"
  >
    {#if ENDINGS[run.ending]}
      <img
        src={`/endings/${ENDINGS[run.ending].icon}`}
        class="inline h-24 cursor-help"
        alt={ENDINGS[run.ending].name}
        title={ENDINGS[run.ending].name}
      />
      <h1
        class="inline align-middle font-mono text-4xl font-medium tracking-wide italic"
      >
        {ENDINGS[run.ending].endingMessage}
      </h1>
    {:else}
      <h1
        class="inline align-middle font-mono text-4xl font-medium tracking-wide italic"
      >
        {run.ending}
      </h1>
    {/if}
  </div>

  <div class="mt-4 flex flex-row gap-4">
    <div class="w-full border bg-bg-secondary p-2">
      <h1 class="text-center text-3xl">Stats</h1>
      <div class="flex flex-row items-center justify-between p-2">
        <span class="text-lg">
          {DIFFICULTIES[run.difficulty]?.displayName ?? run.difficulty}
        </span>
        {#if DIFFICULTIES[run.difficulty]}
          <img
            src={`/difficulties/${DIFFICULTIES[run.difficulty].icon}`}
            alt={run.difficulty}
            class="mr-2 inline h-12"
          />
        {/if}
      </div>
      {#if run.artifacts.length > 0}
        <div class="flex flex-row items-center justify-between p-2">
          <span class="text-lg">Artifacts:</span>
          <ArtifactsDisplay
            artifacts={run.artifacts}
            class="min-w-0 flex-wrap-reverse"
          />
        </div>
      {/if}
      {#snippet score(label: string, value: string, points: string)}
        <div class="flex flex-row items-center justify-between p-2">
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
      <div class="mt-2 flex flex-row items-center justify-around p-2 text-xl">
        <span>Total</span>
        <span>
          <span class="text-yellow-200">
            {formatBig(run.score)}
          </span>
          pts.
        </span>
      </div>
    </div>
    <div class="w-full border bg-bg-secondary p-2">
      <h1 class="text-center text-3xl">Info</h1>
      <div class="flex flex-row items-center justify-between p-2">
        <div>
          {#if BODIES[run.survivor]}
            <img
              src={`/bodies/${BODIES[run.survivor].icon}`}
              alt={BODIES[run.survivor].displayName}
              class="mr-2 inline h-12"
            />
          {/if}
          <span class="text-lg">
            Class:
            <span class="text-yellow-200">
              {BODIES[run.survivor]?.displayName ?? run.survivor}
            </span>
          </span>
        </div>

        <div>
          {#each run.skills as skill}
            {#if SKILLS[skill]}
              <img
                src="/skills/{SKILLS[skill].icon}"
                alt={SKILLS[skill].displayName}
                class="inline h-12"
                title={SKILLS[skill].displayName}
              />
            {:else}
              <span
                class="inline-flex h-12 items-center text-xs text-secondary"
                title={`Unknown skill ${skill}`}
              >
                Unknown ({skill})
              </span>
            {/if}
          {/each}
        </div>
      </div>
      {#if run.equipment}
        <div>
          {#if EQUIPMENTS[run.equipment]}
            <img
              src="/equipment/{EQUIPMENTS[run.equipment].icon}"
              alt={EQUIPMENTS[run.equipment].displayName}
              title={EQUIPMENTS[run.equipment].displayName}
              class="mr-2 inline h-12"
            />
          {/if}
          <span class="text-lg">
            Equipment:
            <span class="text-yellow-200">
              {EQUIPMENTS[run.equipment]?.displayName ??
                `Unknown (${run.equipment})`}
            </span>
          </span>
        </div>
      {/if}
      <h1 class="text-center text-2xl">Items Collected</h1>
      <ul class="mt-4 flex flex-row flex-wrap">
        {#each sortItems(run.items) as [itemId, itemCount] (itemId)}
          {@const item = ITEMS[Number(itemId)]}
          {#if !(item?.helper ?? false)}
            <li class="relative">
              {#if item}
                <img
                  src={`/items/${item.icon}`}
                  alt={item.displayName}
                  class="inline size-16"
                  title={item.displayName}
                />
              {:else}
                <span
                  class="flex size-16 items-center justify-center text-center text-xs text-secondary"
                  title={`Unknown item ${itemId}`}
                >
                  Unknown ({itemId})
                </span>
              {/if}
              {#if itemCount != 1}
                <p
                  class="absolute top-0 right-0 font-mono text-xl font-bold text-shadow-lg/50"
                >
                  x{itemCount}
                </p>
              {/if}
            </li>
          {/if}
        {/each}
      </ul>
      <h1 class="mt-4 text-center text-2xl">Metadata</h1>
      <div class="flex flex-row items-center justify-between p-2">
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
      <div class="flex flex-row items-center justify-between p-2">
        <span>Started:</span>
        <span
          class="text-yellow-200"
          title={new Date(run.startTime).toString()}
        >
          {new Date(run.startTime).toLocaleString()}
        </span>
      </div>
      <div class="flex flex-row items-center justify-between p-2">
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

  <h1 class="mb-4 text-center text-3xl">Stage History</h1>

  <div class="flex flex-col gap-8">
    {#each run.stageHistory as stage}
      <div class="flex flex-row gap-4">
        <div>
          {#if stage.name in ENVIRONMENTS}
            <img
              src="/environments/{ENVIRONMENTS[stage.name].icon}"
              alt="stage"
              class="h-32 border"
            />
            <span class="mb-4 block text-center text-secondary italic">
              {ENVIRONMENTS[stage.name].displayName}
            </span>
          {:else}
            <span class="mb-4 block text-secondary italic">
              {stage.name}
            </span>
          {/if}
        </div>
        <div class="flex flex-1 flex-row flex-wrap">
          {#each stage.interactables.toSorted(sortStageInteractables) as interactable}
            {@const table =
              interactable.name == "EQUIPMENTBARREL_NAME" ? EQUIPMENTS : ITEMS}
            {@const imgPath =
              interactable.name == "EQUIPMENTBARREL_NAME"
                ? "equipment"
                : "items"}
            {#if table[interactable.item]}
              <img
                onmouseenter={(e) => contextMenu?.show(e, interactable)}
                onmousemove={(e) => contextMenu?.update(e)}
                onmouseleave={(_) => contextMenu.hide()}
                src="/{imgPath}/{table[interactable.item].icon}"
                alt={table[interactable.item].displayName}
                class="size-16"
                style="filter: grayscale({interactable.time != null ? 0 : 1});"
              />
            {:else}
              <span
                class="flex size-16 items-center justify-center text-center text-xs text-secondary"
                title={`Unknown ${interactable.name} ${interactable.item}`}
              >
                Unknown ({interactable.item})
              </span>
            {/if}
          {/each}
        </div>
      </div>
    {/each}
  </div>

  <hr class="my-8" />

  <h1 class="mb-4 text-center text-3xl">Item History</h1>

  <table class="w-full">
    <thead>
      <tr>
        <th
          class="relative z-2 px-2 text-center text-xl font-bold tracking-tight"
        >
          Item
        </th>
        <th
          class="relative z-2 px-2 text-center text-xl font-bold tracking-tight"
        >
          Count
        </th>
        <th
          class="relative z-2 px-2 text-center text-xl font-bold tracking-tight"
        >
          Time
        </th>
      </tr>
    </thead>
    <tbody>
      {#each run.itemHistory as itemEvent}
        {#if !(ITEMS[itemEvent.id]?.helper ?? false)}
          <tr class="">
            <td class="border bg-bg-secondary px-4">
              {#if ITEMS[itemEvent.id]}
                <img
                  src="/items/{ITEMS[itemEvent.id].icon}"
                  alt="stage"
                  class="inline h-8"
                />
              {/if}
              <span class="">
                {ITEMS[itemEvent.id]?.displayName ??
                  `Unknown (${itemEvent.id})`}
              </span>
            </td>
            <td class="border bg-bg-secondary px-4">
              <span class={itemEvent.count > 0 ? "text-success" : "text-error"}>
                {itemEvent.count > 0 ? "+" : "-"}
                {itemEvent.count.toString().replace("-", "")}
              </span>
            </td>
            <td class="border bg-bg-secondary px-4">
              <span>{formatSeconds(itemEvent.time)}</span>
            </td>
          </tr>
        {/if}
      {/each}
    </tbody>
  </table>

  <hr class="my-8" />

  <h1 class="mb-4 text-center text-3xl">Equipment History</h1>

  <table class="w-full">
    <thead>
      <tr>
        <th
          class="relative z-2 px-2 text-center text-xl font-bold tracking-tight"
        >
          Item
        </th>
        <th
          class="relative z-2 px-2 text-center text-xl font-bold tracking-tight"
        >
          Time
        </th>
      </tr>
    </thead>
    <tbody>
      {#each run.equipmentHistory as equipmentEvent}
        {#if equipmentEvent.id != -1}
          <tr class="">
            <td class="border bg-bg-secondary px-4">
              {#if EQUIPMENTS[equipmentEvent.id]}
                <img
                  src="/equipment/{EQUIPMENTS[equipmentEvent.id].icon}"
                  alt="stage"
                  class="inline h-12"
                />
              {/if}
              <span class="">
                {EQUIPMENTS[equipmentEvent.id]?.displayName ??
                  `Unknown (${equipmentEvent.id})`}
              </span>
            </td>
            <td class="border bg-bg-secondary px-4">
              <span>{formatSeconds(equipmentEvent.time)}</span>
            </td>
          </tr>
        {/if}
      {/each}
    </tbody>
  </table>
{:catch err}
  {err}
{/await}
