<script lang="ts">
  import { User } from "@lucide/svelte";
  import FormEntry from "./FormEntry.svelte";
  import { onMount } from "svelte";

  let user: any = $state();
  let countries: { flag: string; name: string; alpha2Code: string }[] = $state(
    [],
  );
  onMount(async () => {
    let resp = await fetch("/auth/get-session");
    let body = await resp.json();
    if (body.user) {
      user = body.user;
    } else {
      user = false;
    }
    countries = await (await fetch("https://countries.dev/countries")).json();
  });
</script>

<div class="fixed flex flex-col">
  <a
    href="#profile"
    class="p-2 bg-default hover:bg-hover active:bg-active flex flex-row gap-2"
  >
    <User />
    Profile
  </a>
</div>

<form class="ml-48" action="/api/player" method="POST">
  <h1 id="profile" class="text-3xl tracking-tighter">Profile</h1>
  <FormEntry label="Username" id="profile_username">
    <span class="bg-default flex flex-row border w-60">
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
        value={user.display_username}
        placeholder={user.username}
        class="p-2 bg-default flex flex-row border outline-none w-60"
      />
    {/if}
  </FormEntry>
  <FormEntry label="Region" id="profile_region">
    {#if user}
      <select
        name="region"
        id="profile_region"
        value={user.region}
        class="w-60 bg-default border p-2 cursor-pointer"
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
          <img
            src={user.image}
            alt=""
            class="absolute h-full right-full border"
          />
        {/if}
        <input
          type="text"
          name="image"
          id="profile_image_url"
          bind:value={user.image}
          class="p-2 bg-default flex flex-row border outline-none w-60 text-xs font-mono"
        />
      </div>
    {/if}
  </FormEntry>
  <FormEntry label="About Me" id="profile_about_me">
    {#if user}
      <textarea
        name="aboutMe"
        id="profile_about_me"
        value={user.about_me}
        class="w-60 bg-default border p-2 outline-none"></textarea>
    {/if}
  </FormEntry>
  <input
    type="submit"
    value="Save"
    class="bg-default hover:bg-hover active:bg-active p-2 cursor-pointer border"
  />
</form>
