<script lang="ts">
  import { Mail, KeyRound, Heart, User, Pen } from "@lucide/svelte";
  import OAuthMethods from "$lib/OAuthMethods.svelte";
  import { auth } from "$lib";

  let username: string = $state("");
  let displayUsername: string = $state("");
  let email: string = $state("");
  let password: string = $state("");

  let errors: Record<string, string[]> = $state({});
  let errorMessage: string = $state("");

  async function signIn() {
    const resp = await fetch(auth("sign-up/email"), {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        name: username,
        username,
        displayUsername,
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

<svelte:head>
  <title>WI | Sign Up</title>
</svelte:head>

<div class="flex flex-col items-center">
  <h1 class="text-3xl font-bold tracking-tighter">Sign Up</h1>
  <a href="/sign-in" class="underline">Sign In here</a>
  <div class="flex w-80 flex-col">
    <label
      for="email"
      class="mt-4 flex flex-row items-center gap-1 font-mono text-xl tracking-tighter"
    >
      <Mail /> Email
    </label>
    <input
      type="email"
      id="email"
      autocomplete="email"
      bind:value={email}
      class="border bg-default p-2 font-mono transition-colors outline-none hover:bg-hover active:bg-active"
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
      class="mt-4 flex flex-row items-center gap-1 font-mono text-xl tracking-tighter"
    >
      <KeyRound /> Password
    </label>
    <input
      type="password"
      id="password"
      autocomplete="new-password"
      bind:value={password}
      class="border bg-default p-2 font-mono transition-colors outline-none hover:bg-hover active:bg-active"
    />
    {#if "password" in errors}
      {#each errors.password as err}
        <div class="text-error">
          {err}
        </div>
      {/each}
    {/if}

    <label
      for="username"
      class="mt-4 flex flex-row items-center gap-1 font-mono text-xl tracking-tighter"
    >
      <User /> Username
    </label>
    <div
      class=" flex flex-row border bg-default transition-colors hover:bg-hover active:bg-active"
    >
      <span class="p-2 pr-0">@</span>
      <input
        type="text"
        autocomplete="username"
        id="username"
        bind:value={username}
        class="w-full p-2 pl-0 font-mono outline-none"
      />
    </div>
    {#if "name" in errors}
      {#each errors.name as err}
        <div class="text-error">
          {err.replaceAll("Name", "Username")}
        </div>
      {/each}
    {/if}

    <label
      for="displayUsername"
      class="mt-4 flex flex-row items-center gap-1 font-mono text-xl tracking-tighter"
    >
      <Pen /> Display Username
    </label>
    <input
      type="text"
      autocomplete="nickname"
      id="displayUsername"
      placeholder={username}
      bind:value={displayUsername}
      class="border bg-default p-2 font-mono transition-colors outline-none hover:bg-hover active:bg-active"
    />
    <button
      type="button"
      onclick={signIn}
      class="mt-4 flex cursor-pointer flex-row justify-center gap-1 border bg-default p-2 font-mono transition-colors hover:bg-hover active:bg-active"
    >
      <Heart />

      Proceed
    </button>
    <div class="text-error">
      {errorMessage}
    </div>
  </div>
  <OAuthMethods />
</div>
