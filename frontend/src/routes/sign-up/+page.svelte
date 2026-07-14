<script lang="ts">
  import { Mail, KeyRound, Heart, User, Pen } from "@lucide/svelte";
  import OAuthMethods from "$lib/OAuthMethods.svelte";

  let username: string = $state("");
  let displayUsername: string = $state("");
  let email: string = $state("");
  let password: string = $state("");

  async function signIn() {
    const resp = await fetch("/auth/sign-up/email", {
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
    window.location.href = "/";
    console.log(body);
  }
</script>

<div class="flex flex-col items-center">
  <h1 class="text-3xl tracking-tighter font-bold">Sign Up</h1>
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
      bind:value={email}
      class="p-2 bg-default hover:bg-hover active:bg-active transition-colors border outline-none font-mono"
    />
    <label
      for="password"
      class="text-xl tracking-tighter mt-4 flex flex-row gap-1 items-center font-mono"
    >
      <KeyRound /> Password
    </label>
    <input
      type="password"
      id="password"
      bind:value={password}
      class="p-2 bg-default hover:bg-hover active:bg-active transition-colors border outline-none font-mono"
    />
    <label
      for="username"
      class="text-xl tracking-tighter mt-4 flex flex-row gap-1 items-center font-mono"
    >
      <User /> Username
    </label>
    <div
      class=" bg-default hover:bg-hover active:bg-active border transition-colors flex flex-row"
    >
      <span class="p-2 pr-0">@</span>
      <input
        type="text"
        autocomplete="username"
        id="username"
        bind:value={username}
        class="outline-none font-mono w-full p-2 pl-0"
      />
    </div>
    <label
      for="displayUsername"
      class="text-xl tracking-tighter mt-4 flex flex-row gap-1 items-center font-mono"
    >
      <Pen /> Display Username
    </label>
    <input
      type="text"
      autocomplete="nickname"
      id="displayUsername"
      placeholder={username}
      bind:value={displayUsername}
      class="p-2 bg-default hover:bg-hover active:bg-active transition-colors border outline-none font-mono"
    />
    <button
      type="submit"
      onclick={signIn}
      class="bg-default hover:bg-hover active:bg-active transition-colors border p-2 mt-4 cursor-pointer font-mono flex flex-row gap-1 justify-center"
    >
      <Heart />

      Proceed
    </button>
  </div>
  <OAuthMethods />
</div>
