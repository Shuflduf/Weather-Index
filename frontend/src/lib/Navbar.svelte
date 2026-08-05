<script lang="ts">
  import { onMount } from "svelte";
  import PFP from "./PFP.svelte";
  import { User } from "@lucide/svelte";
  import { auth } from "$lib";
  import { authedFetch, setToken } from "./auth";

  let user: any = $state(null);
  let devBuild: boolean = $state(true);
  let commitHash: string = $state("");
  let commitUrl: string = $state("https://github.com/Shuflduf/Weather-Index");

  onMount(async () => {
    const params = new URLSearchParams(window.location.search);
    const t = params.get("token");
    if (t) {
      setToken(t);
      window.history.replaceState({}, "", window.location.pathname);
    }

    fetch("/env")
      .then((r) => r.json())
      .then((env) => {
        if ("VERCEL" in env) devBuild = false;
        if (devBuild) {
          commitHash = "Development";
        } else {
          const fullCommitHash = env["VERCEL_GIT_COMMIT_SHA"];
          commitUrl = `${commitUrl}/commit/${fullCommitHash}`;
          commitHash = fullCommitHash.slice(0, 8);
        }
      });
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
      class="block border bg-default p-2 hover:bg-hover active:bg-active"
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
  class="flex h-18 flex-row items-center justify-between border bg-bg-secondary"
>
  <div class="flex flex-row items-end gap-8 pl-4">
    <a class="text-2xl font-bold tracking-tighter text-primary" href="/">
      Weather Index
    </a>
    <div class="border py-4"></div>
    <a class="text-xl tracking-tight text-secondary" href="/stats">Stats</a>
    <a class="text-xl tracking-tight text-secondary" href="/docs">Docs</a>
  </div>
  <div class="flex h-full flex-row items-center">
    <a href={commitUrl} class="mr-4 text-secondary underline">
      {commitHash}
    </a>
    {#if user}
      <button
        class="flex h-full cursor-pointer items-center border-l bg-bg-secondary px-4 transition-colors hover:bg-hover active:bg-active"
        popovertarget="user-menu"
        style="anchor-name: --user-menu;"
      >
        <div
          class="inline-flex h-14 w-max flex-row items-center justify-center gap-2"
        >
          <PFP src={user.image} class="h-full border" />
          <span class="text-lg text-primary">
            {user.display_username && user.display_username.length > 0
              ? user.display_username
              : user.username}
          </span>
        </div>
      </button>
    {:else}
      <a
        class={`flex h-full flex-row items-center justify-center gap-2 border-l bg-bg-secondary px-4 text-xl text-primary transition hover:bg-default ${user == false ? "cursor-pointer" : "cursor-not-allowed"}`}
        href={user == false ? "/sign-in" : ""}
      >
        <User />
        <span>Sign In</span>
      </a>
    {/if}
  </div>
</div>
