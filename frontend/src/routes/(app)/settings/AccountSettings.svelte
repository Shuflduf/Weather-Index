<script lang="ts">
  import { auth } from "$lib";
  import { authedFetch } from "$lib/auth";
  import {
    SiDiscord,
    SiGithub,
    SiGoogle,
    SiHackclub,
  } from "@icons-pack/svelte-simple-icons";
  import { onMount } from "svelte";

  let providers: string[] = $state([]);

  let newPassword: string = $state("");
  let currentPassword: string = $state("");

  let changePasswordErrors: Record<string, string[]> = $state({});
  let changePasswordMessage: string = $state("");
  let changePasswordSuccess: boolean = $state(false);

  onMount(() => {
    fetchAccounts();
  });

  async function fetchAccounts() {
    authedFetch(auth("list-accounts"))
      .then((r) => r.json())
      .then((j: any[]) => {
        providers = j.map((a) => a.provider);
      });
  }

  async function connectOauth(provider: string) {
    let resp = await authedFetch(auth("link-social"), {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ provider: provider }),
    });
    let { url } = await resp.json();
    if (url) window.location.href = url;
  }

  async function disconnectOauth(provider: string) {
    await authedFetch(auth("unlink-account"), {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ providerId: provider }),
    });
    authedFetch(auth("list-accounts"))
      .then((r) => r.json())
      .then((j: any[]) => {
        providers = j.map((a) => a.provider);
      });
  }

  async function changePassword() {
    changePasswordErrors = {};
    changePasswordMessage = "";
    let resp = await authedFetch(auth("change-password"), {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ newPassword, currentPassword }),
    });
    let body = await resp.json();
    if ("errors" in body) {
      changePasswordErrors = body.errors;
    }
    if ("message" in body) {
      changePasswordMessage = body.message;
    }
    if ("user" in body) {
      changePasswordSuccess = true;
      changePasswordMessage = "Changed password successfully";
      newPassword = "";
      currentPassword = "";
    }
  }
</script>

<div>
  <h1 id="account" class="text-3xl tracking-tighter">Account</h1>
  <div class="mt-4 flex flex-col gap-2 font-mono">
    <details>
      <summary class="cursor-pointer text-xl">Change Password</summary>
      <div class="flex w-md flex-col border bg-bg-secondary p-4">
        <label
          for="currentPassword"
          class="flex flex-row items-center gap-1 font-sans text-lg tracking-tight"
        >
          Current Password
        </label>
        <input
          type="password"
          name="currentPassword"
          id="currentPassword"
          class="border bg-default p-2 font-mono transition-colors outline-none hover:bg-hover active:bg-active"
          bind:value={currentPassword}
        />
        {#if "current_password" in changePasswordErrors}
          {#each changePasswordErrors.current_password as err}
            <div class="text-error">
              {err}
            </div>
          {/each}
        {/if}

        <label
          for="newPassword"
          class="mt-4 flex flex-row items-center gap-1 font-sans text-lg tracking-tight"
        >
          New Password
        </label>
        <input
          type="password"
          name="newPassword"
          id="newPassword"
          class="border bg-default p-2 font-mono transition-colors outline-none hover:bg-hover active:bg-active"
          bind:value={newPassword}
        />
        {#if "new_password" in changePasswordErrors}
          {#each changePasswordErrors.new_password as err}
            <div class="text-error">
              {err}
            </div>
          {/each}
        {/if}
        <button
          type="button"
          onclick={changePassword}
          class="mt-4 flex cursor-pointer flex-row justify-center gap-1 border bg-default p-2 font-mono transition-colors hover:bg-hover active:bg-active"
        >
          Submit
        </button>
        <div class={changePasswordSuccess ? "text-success" : "text-error"}>
          {changePasswordMessage}
        </div>
      </div>
    </details>
    {#snippet providerStatus(name: string, display: string)}
      <span>{display}:</span>
      {#if providers.includes(name)}
        Connected
        <button
          onclick={() => disconnectOauth(name)}
          class="cursor-pointer border bg-default px-2 transition-colors hover:bg-red-800 active:bg-red-600"
        >
          Disconnect
        </button>
      {:else}
        <button
          onclick={() => connectOauth(name)}
          class="cursor-pointer border bg-default px-2 transition-colors hover:bg-hover active:bg-hover"
        >
          Connect
        </button>
      {/if}
    {/snippet}
    <div class="flex w-max flex-row gap-2 border bg-bg-secondary p-2">
      <SiGithub />
      {@render providerStatus("github", "GitHub")}
    </div>
    <div class="flex w-max flex-row gap-2 border bg-bg-secondary p-2">
      <SiDiscord />
      {@render providerStatus("discord", "Discord")}
    </div>
    <div class="flex w-max flex-row gap-2 border bg-bg-secondary p-2">
      <SiGoogle />
      {@render providerStatus("google", "Google")}
    </div>
    <div class="flex w-max flex-row gap-2 border bg-bg-secondary p-2">
      <SiHackclub />
      {@render providerStatus("hca", "Hack Club")}
    </div>
    <div class="flex w-max flex-row gap-2 border bg-bg-secondary p-2">
      <img src="slack.svg" alt="Slack" class="size-6" />
      {@render providerStatus("slack", "Slack")}
    </div>
  </div>
</div>
