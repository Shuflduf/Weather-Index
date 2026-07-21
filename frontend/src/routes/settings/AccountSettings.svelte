<script lang="ts">
  import { auth } from "$lib";
  import {
    SiDiscord,
    SiGithub,
    SiGoogle,
  } from "@icons-pack/svelte-simple-icons";
  import { onMount } from "svelte";

  let accounts: any[] = $state([]);
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
    fetch(auth("list-accounts"))
      .then((r) => r.json())
      .then((j: any[]) => {
        accounts = j;
        providers = j.map((a) => a.provider);
      });
  }

  async function connectOauth(provider: string) {
    let resp = await fetch(auth("link-social"), {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ provider: provider }),
    });
    let { url } = await resp.json();
    if (url) window.location.href = url;
  }

  async function changePassword() {
    changePasswordErrors = {};
    changePasswordMessage = "";
    let resp = await fetch(auth("change-password"), {
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
  <div class="flex flex-col gap-2 mt-4 font-mono">
    <details>
      <summary class="text-xl cursor-pointer">Change Password</summary>
      <form class="flex flex-col w-md border p-4 bg-bg-secondary">
        <label
          for="currentPassword"
          class="text-lg tracking-tight flex flex-row gap-1 items-center font-sans"
        >
          Current Password
        </label>
        <input
          type="password"
          name="currentPassword"
          id="currentPassword"
          class="p-2 bg-default hover:bg-hover active:bg-active transition-colors border outline-none font-mono"
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
          class="text-lg tracking-tight mt-4 flex flex-row gap-1 items-center font-sans"
        >
          New Password
        </label>
        <input
          type="password"
          name="newPassword"
          id="newPassword"
          class="p-2 bg-default hover:bg-hover active:bg-active transition-colors border outline-none font-mono"
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
          type="submit"
          onclick={changePassword}
          class="bg-default hover:bg-hover active:bg-active transition-colors border p-2 mt-4 cursor-pointer font-mono flex flex-row gap-1 justify-center"
        >
          Submit
        </button>
        <div class={changePasswordSuccess ? "text-success" : "text-error"}>
          {changePasswordMessage}
        </div>
      </form>
    </details>
    <div class="flex flex-row bg-bg-secondary border p-2 gap-2 w-max">
      <SiGithub />
      <span>Github:</span>
      {#if providers.includes("github")}
        Connected
      {:else}
        <button
          onclick={() => connectOauth("github")}
          class="px-2 bg-default hover:bg-hover active:bg-hover border cursor-pointer"
        >
          Connect
        </button>
      {/if}
    </div>
    <div class="flex flex-row bg-bg-secondary border p-2 gap-2 w-max">
      <SiDiscord />
      <span>Discord:</span>
      {#if providers.includes("discord")}
        Connected
      {:else}
        <button
          onclick={() => connectOauth("discord")}
          class="px-2 bg-default hover:bg-hover active:bg-hover border cursor-pointer"
        >
          Connect
        </button>
      {/if}
    </div>
    <div class="flex flex-row bg-bg-secondary border p-2 gap-2 w-max">
      <SiGoogle />
      <span>Google:</span>
      {#if providers.includes("google")}
        Connected
      {:else}
        <button
          onclick={() => connectOauth("google")}
          class="px-2 bg-default hover:bg-hover active:bg-hover border cursor-pointer"
        >
          Connect
        </button>
      {/if}
    </div>
  </div>
</div>
