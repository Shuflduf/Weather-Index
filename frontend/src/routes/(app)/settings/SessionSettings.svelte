<script lang="ts">
  import { auth } from "$lib";
  import { authedFetch } from "$lib/auth";
  import LoadingIndicator from "$lib/LoadingIndicator.svelte";
  import { onMount } from "svelte";

  let sessionPromise: Promise<
    {
      active: boolean;
      created_at: string;
      expires_at: string;
      id: string;
      token: string;
      updated_at: string;
      // user_agent: string;
    }[]
  > = $state(new Promise(() => {}));
  onMount(() => {
    sessionPromise = authedFetch(auth("list-sessions")).then((r) => r.json());
  });

  function revokeSession(token: string) {
    sessionPromise = new Promise(() => {});
    authedFetch(auth("revoke-session"), {
      method: "POST",
      body: JSON.stringify({ token }),
      headers: { "Content-Type": "application/json" },
    }).then((_) => {
      sessionPromise = authedFetch(auth("list-sessions")).then((r) => r.json());
    });
  }

  function revokeOthers() {
    sessionPromise = new Promise(() => {});
    authedFetch(auth("revoke-other-sessions"), { method: "POST" }).then((_) => {
      sessionPromise = authedFetch(auth("list-sessions")).then((r) => r.json());
    });
  }
</script>

<div>
  <h1 id="sessions" class="mb-4 text-3xl tracking-tighter">Sessions</h1>
  {#await sessionPromise}
    <LoadingIndicator indicator text="Loading sessions" />
  {:then sessions}
    <button
      class="mb-4 cursor-pointer border bg-default p-2 text-xl transition-colors hover:bg-red-800 active:bg-red-600"
      onclick={() => revokeOthers()}
    >
      Revoke all except current
    </button>
    {#each sessions as session}
      <div class="w-lg border bg-bg-secondary p-2">
        <div class="flex flex-row justify-between">
          <span class="text-secondary">
            {session.id}
          </span>
          <button
            onclick={() => revokeSession(session.token)}
            class="cursor-pointer border bg-default px-2 transition-colors hover:bg-red-800 active:bg-red-600"
          >
            Revoke
          </button>
        </div>
        <hr class="my-1" />
        <div class="flex flex-row justify-between">
          <div>
            <b>Created At:</b>
            <span
              title={new Date(session.created_at).toString()}
              class="cursor-help"
            >
              {new Date(session.created_at).toDateString()}
            </span>
          </div>
          <div>
            <b>Updated At:</b>
            <span
              title={new Date(session.updated_at).toString()}
              class="cursor-help"
            >
              {new Date(session.updated_at).toDateString()}
            </span>
          </div>
        </div>
      </div>
    {/each}
  {/await}
</div>
