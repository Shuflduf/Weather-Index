<script lang="ts">
  import { auth } from "$lib";
  import { authedFetch, setToken } from "$lib/auth";
  import { onMount } from "svelte";

  let status: string = $state("Signing out...");

  onMount(async () => {
    const resp = await authedFetch(auth("sign-out"), {
      method: "POST",
    });
    setToken(null);
    if (resp.ok) {
      status = "Signed out! Redirecting to /";
    } else {
      status = "Error. Already signed out? Redirecting to /";
    }
    window.location.href = "/";
  });
</script>

<svelte:head>
  <title>WI | Signing Out...</title>
</svelte:head>

<span class="block text-3xl">{status}</span>
