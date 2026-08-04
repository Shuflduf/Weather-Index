<script lang="ts">
  import { onMount } from "svelte";
  import PFP from "./PFP.svelte";
  import { User } from "@lucide/svelte";
  import { auth } from "$lib";
  import { authedFetch, setToken } from "./auth";
  import { env } from "$env/dynamic/public";

  let user: any = $state(null);
  let commit: string = $state(env.VERCEL_GIT_COMMIT_MESSAGE);

  onMount(async () => {
    const params = new URLSearchParams(window.location.search);
    const t = params.get("token");
    if (t) {
      setToken(t);
      window.history.replaceState({}, "", window.location.pathname);
    }

    let resp = await authedFetch(auth("get-session"));
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
  <div class="flex flex-row gap-8 items-end pl-4">
    <a class="text-primary text-2xl font-bold tracking-tighter" href="/">
      Weather Index
    </a>
    <div class="border py-4"></div>
    <a class="text-xl tracking-tight text-secondary" href="/stats">Stats</a>
    <a class="text-xl tracking-tight text-secondary" href="/docs">Docs</a>
  </div>
  {commit}
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
          {user.display_username && user.display_username.length > 0
            ? user.display_username
            : user.username}
        </span>
      </div>
    </button>
  {:else}
    <a
      class={`flex bg-bg-secondary hover:bg-default flex-row gap-2 text-xl border-l px-4 h-full text-primary transition justify-center items-center ${user == false ? "cursor-pointer" : "cursor-not-allowed"}`}
      href={user == false ? "/sign-in" : ""}
    >
      <User />
      <span>Sign In</span>
    </a>
  {/if}
</div>
