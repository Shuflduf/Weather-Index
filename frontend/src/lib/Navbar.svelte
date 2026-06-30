<script lang="ts">
	import { SiGithub } from "@icons-pack/svelte-simple-icons";

  let loggedIn = $state(false)
  async function startGithubOauth() {
    let resp = await fetch("/auth/sign-in/social", {
      method: "POST",
      headers: {"Content-Type": "application/json"},
      body: JSON.stringify({provider: "github"})
    })
    let { url } = await resp.json()
    if (url) window.location.href = url
  }
  </script>

<div class="bg-bg-secondary flex flex-row items-center border justify-between h-18">
  <h1 class="text-primary  text-2xl font-bold tracking-tighter pl-4">Weather Index</h1>
  {#if loggedIn}
sdoifjskdf
{:else} <button class="flex bg-bg-secondary hover:bg-default transition flex-row gap-2 text-xl border-l px-4 h-full text-primary justify-center items-center cursor-pointer" onclick={startGithubOauth}>
	<SiGithub />
	<span>Log In</span>
</button>
{/if}
</div>
