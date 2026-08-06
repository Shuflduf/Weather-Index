<script lang="ts">
  import { auth } from "$lib";
  import {
    SiDiscord,
    SiGithub,
    SiGoogle,
    SiHackclub,
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

<h2 class="mt-8 text-2xl font-bold tracking-tighter">Alternative Methods</h2>
<div class="mt-4 flex flex-row gap-4">
  <button
    onclick={() => startOauth("github")}
    class="cursor-pointer border bg-default p-2 transition-colors hover:bg-hover active:bg-active"
  >
    <SiGithub title="GitHub" />
  </button>
  <button
    onclick={() => startOauth("google")}
    class="cursor-pointer border bg-default p-2 transition-colors hover:bg-hover active:bg-active"
  >
    <SiGoogle title="Google" />
  </button>
  <button
    onclick={() => startOauth("discord")}
    class="cursor-pointer border bg-default p-2 transition-colors hover:bg-hover active:bg-active"
  >
    <SiDiscord title="Discord" />
  </button>
  <button
    onclick={() => startOauth("hca")}
    class="cursor-pointer border bg-default p-2 transition-colors hover:bg-hover active:bg-active"
  >
    <SiHackclub title="Hack Club" />
  </button>
  <button
    onclick={() => startOauth("slack")}
    class="cursor-pointer border bg-default p-2 transition-colors hover:bg-hover active:bg-active"
  >
    <img src="/slack.svg" alt="Slack" class="size-6" />
  </button>
</div>
