<script lang="ts">
  import FormEntry from "./FormEntry.svelte";
  import { onMount } from "svelte";
  import PFP from "$lib/PFP.svelte";
  import { api, auth } from "$lib";
  import { authedFetch } from "$lib/auth";
  import LoadingIndicator from "$lib/LoadingIndicator.svelte";

  let infoPromise: Promise<{
    user: any;
    countries: { flag: string; name: string; alpha2Code: string }[];
  }> = $state(new Promise(() => {}));
  // let user: any = $state();
  // let countries:  = $state(
  //   [],
  // );
  let success: boolean = $state(true);
  let updateResult: string | null = $state(null);

  onMount(() => {
    infoPromise = Promise.all([
      authedFetch(auth("get-session")).then((r) => r.json()),
      fetch("https://countries.dev/countries").then((r) => r.json()),
    ]).then(([body, countries]) => ({ user: body.user, countries }));
  });

  function updatePlayer(user: any) {
    console.log(user);
    authedFetch(api("player"), {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        image: user.image,
        username: user.username,
        displayUsername: user.displayUsername,
        aboutMe: user.aboutMe,
        region: user.region,
      }),
    }).then(async (r) => {
      success = r.ok;
      if (r.ok) {
        updateResult = "Profile updated succesfully";
      } else {
        r.json().then((j) => (updateResult = j.error));
      }
    });
  }
</script>

<div>
  <h1 id="profile" class="text-3xl tracking-tighter">Profile</h1>
  {#await infoPromise}
    <LoadingIndicator indicator text="Loading profile" />
  {:then { user, countries }}
    <FormEntry label="Username" id="profile_username">
      <span class="flex w-60 flex-row border bg-default">
        <span class="p-2 pr-0">@</span>
        {#if user}
          <input
            type="text"
            name="username"
            id="profile_username"
            bind:value={user.username}
            class="p-2 pl-0 outline-none"
          />
        {/if}
      </span>
    </FormEntry>
    <FormEntry label="Display Username" id="profile_display_username">
      {#if user}
        <input
          type="text"
          name="displayUsername"
          id="profile_display_username"
          bind:value={user.display_username}
          placeholder={user.username}
          class="flex w-60 flex-row border bg-default p-2 outline-none"
        />
      {/if}
    </FormEntry>
    <FormEntry label="Region" id="profile_region">
      {#if user}
        <select
          name="region"
          id="profile_region"
          bind:value={user.region}
          class="w-60 cursor-pointer border bg-default p-2"
        >
          <option value={null}>None</option>
          {#each countries as country}
            <option value={country.alpha2Code}>
              {country.flag}
              {country.name}
            </option>
          {/each}
        </select>
      {/if}
    </FormEntry>
    <FormEntry label="Image URL" id="profile_image_url">
      {#if user}
        <div class="relative">
          {#if user.image}
            <PFP src={user.image} class="absolute right-full h-full border" />
          {/if}
          <input
            placeholder="https://..."
            type="text"
            name="image"
            id="profile_image_url"
            bind:value={user.image}
            class="flex w-60 flex-row border bg-default p-2 font-mono text-xs outline-none"
          />
        </div>
      {/if}
    </FormEntry>
    <FormEntry label="About Me" id="profile_about_me">
      {#if user}
        <textarea
          name="aboutMe"
          id="profile_about_me"
          bind:value={user.about_me}
          class="w-60 border bg-default p-2 outline-none"></textarea>
      {/if}
    </FormEntry>
    <button
      onclick={() => updatePlayer(user)}
      type="button"
      class="cursor-pointer border bg-default p-2 hover:bg-hover active:bg-active"
    >
      Save
    </button>
    {#if updateResult}
      {#if success == true}
        <span class="ml-2 text-success">{updateResult}</span>
      {:else}
        <span class="ml-2 text-error">{updateResult}</span>
      {/if}
    {/if}
  {/await}
</div>
