<script lang="ts">
  import { SiGithub } from "@icons-pack/svelte-simple-icons";
  import { onMount } from "svelte";
  import UserDisplay from "./UserDisplay.svelte";
  import PFP from "./PFP.svelte";

  let user: any = $state(null);

  async function startGithubOauth() {
    let resp = await fetch("/auth/sign-in/social", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ provider: "discord" }),
    });
    let { url } = await resp.json();
    if (url) window.location.href = url;
  }

  onMount(async () => {
    let resp = await fetch("/auth/get-session");
    let body = await resp.json();
    if (body.user) {
      user = body.user;
    } else {
      user = false;
    }
  });
</script>

<div
  class="bg-bg-secondary text-primary"
  popover
  id="user-menu"
  style="position-anchor: --user-menu; position-area: bottom span-left;"
>
  {#snippet item(href: string, text: string)}
    <a
      data-sveltekit-reload
      {href}
      class="bg-default hover:bg-hover active:bg-active p-2 block border"
    >
      {text}
    </a>
  {/snippet}
  {#if user}
    {@render item(`/player/${user.username}`, "Open Profile")}
  {/if}
  {@render item("/settings", "Settings")}
  {@render item("/sign-out", "Sign Out")}
</div>

<div
  class="bg-bg-secondary flex flex-row items-center border justify-between h-18"
>
  <a class="text-primary text-2xl font-bold tracking-tighter pl-4" href="/">
    Weather Index
  </a>
  {#if user}
    <button
      class="h-full border-l px-4 flex items-center cursor-pointer transition-colors bg-bg-secondary hover:bg-hover active:bg-active"
      popovertarget="user-menu"
      style="anchor-name: --user-menu;"
    >
      <div
        class="inline-flex flex-row justify-center items-center gap-2 w-max h-14"
      >
        <PFP src={user.image} class="h-full border" />
        <span class="text-primary text-lg">
          {user.display_username ?? user.username}
        </span>
      </div>
    </button>
  {:else}
    <button
      class={`flex bg-bg-secondary hover:bg-default flex-row gap-2 text-xl border-l px-4 h-full text-primary transition justify-center items-center ${user == false ? "cursor-pointer" : "cursor-not-allowed"}`}
      onclick={user == false ? startGithubOauth : null}
    >
      <SiGithub />
      <span>Log In</span>
    </button>
  {/if}
</div>
