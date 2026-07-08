<script lang="ts">
  import { page } from "$app/state";
  import type { PlayerInfo } from "$lib";

  import { onMount } from "svelte";

  let playerInfoPromise: Promise<PlayerInfo> = $state(new Promise(() => {}));
  let regionPromise: Promise<any> = $state(new Promise(() => {}));

  onMount(() => {
    const username = page.params.username;
    if (!username) console.error("no username");
    playerInfoPromise = fetch(`/api/player/${username}`)
      .then((r) => r.json())
      .then((info) => {
        if (info.region) {
          regionPromise = fetch(
            `https://countries.dev/alpha/${info.region}`,
          ).then((r) => r.json());
        }
        return info;
      });
  });
</script>

<div class="text-primary p-8">
  {#await playerInfoPromise}
    loading
  {:then playerInfo}
    <div class="flex flex-row gap-4">
      <img
        src={playerInfo.image}
        alt={playerInfo.username}
        class="h-80 border"
      />
      <div>
        <h2 class="text-4xl tracking-tighter">
          {#await regionPromise}
            Loading Region
          {:then regionInfo}
            <span title={regionInfo.name}>
              {regionInfo.flag}
            </span>
          {/await}
          {#if playerInfo.display_username}
            {playerInfo.display_username}
          {:else}
            {playerInfo.username}
          {/if}
        </h2>
        <h3 class="text-secondary text-xl">@{playerInfo.username}</h3>
        {#if playerInfo.region}

        {/if}
        <h2 class="mt-4 text-2xl tracking-tighter">About Me</h2>
        {#if playerInfo.about_me}
          {playerInfo.about_me}
        {:else}
          <span class="italic text-secondary">None yet!</span>
        {/if}
      </div>
    </div>
    {JSON.stringify(playerInfo)}
  {/await}
</div>
