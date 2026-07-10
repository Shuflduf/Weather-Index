<script lang="ts">
  import { onMount } from "svelte";

  let status: string = $state("Signing out...");

  onMount(async () => {
    const resp = await fetch("/auth/sign-out", { method: "POST" });
    if (resp.ok) {
      status = "Signed out! Redirecting to /";
    } else {
      status = "Error. Already signed out? Redirecting to /";
    }
    window.location.href = "/";
  });
</script>

<span class="text-3xl block">{status}</span>
