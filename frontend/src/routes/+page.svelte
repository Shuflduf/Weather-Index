<script lang="ts">
  import { onMount } from "svelte";
  import {
    BODIES,
    countRealItems,
    DIFFICULTIES,
    ENDINGS,
    ITEMS,
    type Item,
  } from "$lib/RoR2";
  import UserDisplay from "$lib/UserDisplay.svelte";

  let visibleProperties: Record<string, boolean> = $state({
    id: true,
    player: true,
    uploadTime: false,

    // run info
    survivor: true,
    startTime: false,
    ending: true,
    difficulty: true,
    timeAlive: false,
    artifacts: false,
    stagesCompleted: false,
    score: true,

    // items
    itemsCollected: true,

    // drones
    dronesPurchased: false,
    turretsPurchased: false,

    // combat
    kills: false,
    eliteKills: false,
    minionKills: false,
    deaths: false,

    // damage
    damageDealt: false,
    minionDamageDealt: false,
    damageTaken: false,
    highestDamageDealt: false,

    // healing
    healingRecieved: false,

    // progression
    highestLevel: false,
    goldCollected: false,
    goldSpent: false,
    lunarCoinsSpent: false,
    purchases: false,
    bloodPurchases: false,

    // movement
    distanceTraveledMetres: false,
  });

  let runPromise: Promise<any[]> = $state(new Promise(() => {}));
  onMount(async () => {
    runPromise = fetch("/api/runs").then((r) => r.json());
  });
</script>

<div
  id="visible-properties"
  popover
  class="fixed bg-bg-secondary border text-primary p-2 w-64"
  style="position-anchor: --visible-properties; position-area: bottom span-right;"
>
  {#snippet visibleProperty(name: string, id: string)}
    <div class="flex flex-row justify-between">
      <span>{name}</span>
      <input type="checkbox" bind:checked={visibleProperties[id]} />
    </div>
  {/snippet}
  <div>
    {@render visibleProperty("ID", "id")}
    {@render visibleProperty("Player", "player")}
    {@render visibleProperty("Survivor", "survivor")}
    {@render visibleProperty("Ending", "ending")}
    {@render visibleProperty("Difficulty", "difficulty")}
    {@render visibleProperty("Items", "itemsCollected")}
    {@render visibleProperty("Score", "score")}
  </div>
</div>

<div class="w-full p-8">
  {#await runPromise}
    <span class="text-primary">loading</span>
  {:then runs}
    <button
      popovertarget="visible-properties"
      class="text-primary cursor-pointer bg-default hover:bg-hover active:bg-active p-2 font-mono mb-4"
      style="anchor-name: --visible-properties;"
    >
      Visible Properties
    </button>
    <table class="text-primary w-full">
      <thead class="text-xl tracking-tight text-center font-bold h-12">
        <tr>
          {#if visibleProperties.id}
            <td>ID</td>
          {/if}
          {#if visibleProperties.player}
            <td>Player</td>
          {/if}
          {#if visibleProperties.survivor}
            <td>Survivor</td>
          {/if}
          {#if visibleProperties.ending}
            <td>Ending</td>
          {/if}
          {#if visibleProperties.difficulty}
            <td>Difficulty</td>
          {/if}
          {#if visibleProperties.itemsCollected}
            <td>Items</td>
          {/if}
          {#if visibleProperties.score}
            <td>Score</td>
          {/if}
        </tr>
      </thead>
      <tbody>
        {#each runs as run}
          <tr class="border bg-bg-secondary">
            {#if visibleProperties.id}
              <td class="border p-4">
                <a href={`/run/${run.id}`}>{run.id}</a>
              </td>
            {/if}
            {#if visibleProperties.player}
              <td class="border p-4">
                <UserDisplay
                  class="h-12"
                  user={{
                    username: run.user_username,
                    image: run.user_image,
                    displayName: null,
                  }}
                />
              </td>
            {/if}
            {#if visibleProperties.survivor}
              <td class="border p-4">
                <img
                  src={`/bodies/${BODIES[run.survivor].icon}`}
                  alt={BODIES[run.survivor].displayName}
                  class="h-12 inline mr-2"
                />
                <span class="text-lg">
                  {BODIES[run.survivor].displayName}
                </span>
              </td>
            {/if}
            {#if visibleProperties.ending}
              <td
                class="border p-4"
                style={`background-color: ${ENDINGS[run.ending].colorBg};`}
              >
                <img
                  src={`/endings/${ENDINGS[run.ending].icon}`}
                  alt={run.ending}
                  class="h-12 inline mr-2"
                />
                <span class="text-shadow-lg text-lg">
                  {ENDINGS[run.ending].displayName}
                </span>
              </td>
            {/if}
            {#if visibleProperties.difficulty}
              <td class="border p-4">
                <img
                  src={`/difficulties/${DIFFICULTIES[run.difficulty].icon}`}
                  alt={run.difficulty}
                  class="h-12 inline mr-2"
                />
                <span class="text-lg">
                  {DIFFICULTIES[run.difficulty].displayName}
                </span>
              </td>
            {/if}
            {#if visibleProperties.itemsCollected}
              <td class="border p-4">{countRealItems(run.items)}</td>
            {/if}
            {#if visibleProperties.score}
              <td class="border p-4">{run.score}</td>
            {/if}
          </tr>
        {/each}
      </tbody>
    </table>
  {/await}
</div>
