<script lang="ts">
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
<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://svelte.dev/docs/kit">svelte.dev/docs/kit</a> to read the documentation</p>
<button onclick={startGithubOauth}>GITHUB</button>
