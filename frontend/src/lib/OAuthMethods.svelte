<script lang="ts">
  import { auth } from "$lib";
  import {
    SiDiscord,
    SiGithub,
    SiGoogle,
  } from "@icons-pack/svelte-simple-icons";

  async function startOauth(provider: string) {
    let resp = await fetch(auth("sign-in/social"), {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      credentials: "include",
      body: JSON.stringify({ provider: provider }),
    });
    let { url } = await resp.json();
    if (url) window.location.href = url;
  }
</script>

<h2 class="text-2xl tracking-tighter font-bold mt-8">Alternative Methods</h2>
<div class="mt-4 flex flex-row gap-4">
  <button
    onclick={() => startOauth("github")}
    class="p-2 bg-default hover:bg-hover active:bg-active transition-colors border cursor-pointer"
  >
    <SiGithub title="GitHub" />
  </button>
  <button
    onclick={() => startOauth("google")}
    class="p-2 bg-default hover:bg-hover active:bg-active transition-colors border cursor-pointer"
  >
    <SiGoogle title="Google" />
  </button>
  <button
    onclick={() => startOauth("discord")}
    class="p-2 bg-default hover:bg-hover active:bg-active transition-colors border cursor-pointer"
  >
    <SiDiscord title="Discord" />
  </button>
</div>
