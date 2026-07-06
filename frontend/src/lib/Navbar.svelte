<script lang="ts">
  import { SiGithub } from "@icons-pack/svelte-simple-icons";
  import { onMount } from "svelte";
  import UserDisplay from "./UserDisplay.svelte";

  let user: any = $state(null);

  async function startGithubOauth() {
    let resp = await fetch("/auth/sign-in/social", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ provider: "github" }),
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
  class="bg-bg-secondary flex flex-row items-center border justify-between h-18"
>
  <a class="text-primary text-2xl font-bold tracking-tighter pl-4" href="/">
    Weather Index
  </a>
  {#if user}
    <div class="h-full border-l px-4 flex items-center">
      <UserDisplay class="h-16" {user} />
    </div>
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
