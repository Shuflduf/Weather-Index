<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/state";

  let userCode = $state("");
  let status = $state("pending");

  onMount(() => {
    const params = page.url.searchParams;
    if (params.has("user_code")) {
      userCode = params.get("user_code")!;
    }
  });

  async function approve() {
    status = "pending";
    const resp = await fetch("/auth/device/approve", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ userCode }),
    });
    if (resp.ok) {
      status = "approved";
    } else {
      status = "error";
      console.error(await resp.text());
    }
  }
  async function deny() {
    status = "denied";
    const resp = await fetch("/auth/device/deny", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ userCode }),
    });
    if (resp.ok) {
      status = "denied";
    } else {
      status = "error";
      console.error(await resp.text());
    }
  }
</script>

{status}
<button
  class="bg-default active:bg-active transition p-2 cursor-pointer hover:bg-hover border"
  onclick={approve}
>
  AGREE
</button>
<button
  class="bg-default active:bg-active transition p-2 cursor-pointer hover:bg-hover border"
  onclick={deny}
>
  Cancel
</button>
