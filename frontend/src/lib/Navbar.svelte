<script lang="ts">
  import { SiGithub } from "@icons-pack/svelte-simple-icons";
  import { onMount } from "svelte";

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
  <h1 class="text-primary text-2xl font-bold tracking-tighter pl-4">
    Weather Index
  </h1>
  {#if user}
    <div
      class="flex flex-row h-full justify-center items-center gap-2 px-4 border-l"
    >
      <img src={user.image} alt="profile" class="h-4/5 rounded-full" />
      <span class="text-primary text-xl"
        >{user.displayName ?? user.username}</span
      >
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
