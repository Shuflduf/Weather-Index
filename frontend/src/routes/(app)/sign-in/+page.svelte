<script lang="ts">
  import { Mail, KeyRound, Heart } from "@lucide/svelte";
  import OAuthMethods from "$lib/OAuthMethods.svelte";
  import { auth } from "$lib";
  import { setToken } from "$lib/auth";

  let email: string = $state("");
  let password: string = $state("");
  let errors: Record<string, string> = $state({});
  let errorMessage: string = $state("");

  async function signIn() {
    const resp = await fetch(auth("sign-in/email"), {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      credentials: "include",
      body: JSON.stringify({
        email,
        password,
      }),
    });
    const body = await resp.json();
    if ("errors" in body) {
      errors = body.errors;
    }
    if ("message" in body) {
      errorMessage = body.message;
    }
    if ("user" in body && "token" in body) {
      window.location.href = body.token ? `/?token=${body.token}` : "/";
    }
  }
</script>

<div class="flex flex-col items-center">
  <h1 class="text-3xl tracking-tighter font-bold">Sign In</h1>
  <a href="/sign-up" class="underline">Sign Up here</a>
  <div class="flex flex-col w-80">
    <label
      for="email"
      class="text-xl tracking-tighter mt-4 flex flex-row gap-1 items-center font-mono"
    >
      <Mail /> Email
    </label>
    <input
      type="email"
      id="email"
      autocomplete="email"
      bind:value={email}
      class="p-2 bg-default hover:bg-hover active:bg-active transition-colors border outline-none font-mono"
    />
    {#if "email" in errors}
      {#each errors.email as err}
        <div class="text-error">
          {err}
        </div>
      {/each}
    {/if}
    <label
      for="password"
      class="text-xl tracking-tighter mt-4 flex flex-row gap-1 items-center font-mono"
    >
      <KeyRound /> Password
    </label>
    <input
      type="password"
      autocomplete="new-password"
      id="password"
      bind:value={password}
      class="p-2 bg-default hover:bg-hover active:bg-active transition-colors border outline-none font-mono"
    />
    {#if "password" in errors}
      {#each errors.password as err}
        <div class="text-error">
          {err}
        </div>
      {/each}
    {/if}
    <button
      type="button"
      onclick={signIn}
      class="bg-default hover:bg-hover active:bg-active transition-colors border p-2 mt-4 cursor-pointer font-mono flex flex-row gap-1 justify-center"
    >
      <Heart />

      Proceed
    </button>
    <div class="text-error">{errorMessage}</div>
  </div>
  <OAuthMethods />
</div>
