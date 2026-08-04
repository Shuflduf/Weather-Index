<script lang="ts">
  import { onMount } from "svelte";
  import { page } from "$app/state";
  import { auth } from "$lib";
  import { authedFetch } from "$lib/auth";
  import LoadingIndicator from "$lib/LoadingIndicator.svelte";

  let currentUserPromise: Promise<{
    session: any;
    user: { username: string };
  }> = $state(new Promise(() => {}));
  let userCode = $state("");
  let status: "PENDING" | "APPROVED" | "ERROR" | "DENIED" = $state("PENDING");

  onMount(() => {
    const params = page.url.searchParams;
    if (params.has("user_code")) {
      userCode = params.get("user_code")!;
    }
    currentUserPromise = authedFetch(auth("get-session")).then((r) => r.json());
  });

  async function approve() {
    status = "PENDING";
    const resp = await authedFetch(auth("device/approve"), {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ userCode }),
    });
    if (resp.ok) {
      status = "APPROVED";
    } else {
      status = "ERROR";
      console.error(await resp.text());
    }
  }
  async function deny() {
    status = "DENIED";
    const resp = await authedFetch(auth("device/deny"), {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ userCode }),
    });
    if (resp.ok) {
      status = "DENIED";
    } else {
      status = "ERROR";
      console.error(await resp.text());
    }
  }
</script>

{#await currentUserPromise}
  <LoadingIndicator text="Loading user" indicator />
{:then currentUser}
  <div class="flex justify-center items-center flex-col">
    <span>
      Connecting as <b>@{currentUser.user.username}</b>
    </span>
    <span class="text-bold text-3xl tracking-tighter">
      {status}
    </span>
    <div class="mt-4">
      <button
        class="bg-default active:bg-active transition p-2 cursor-pointer hover:bg-hover border"
        onclick={approve}
      >
        Connect
      </button>
      <button
        class="bg-default active:bg-active transition p-2 cursor-pointer hover:bg-hover border"
        onclick={deny}
      >
        Cancel
      </button>
    </div>
  </div>
{/await}
