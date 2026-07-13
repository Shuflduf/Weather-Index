<script lang="ts">
  import { Mail, KeyRound, Heart } from "@lucide/svelte";

  let email: string = $state("");
  let password: string = $state("");

  async function signIn() {
    const resp = await fetch("/auth/sign-in/email", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        email,
        password,
      }),
    });
    const body = await resp.json();
    console.log(body);
  }
</script>

<div class="flex flex-col items-center">
  <h1 class="text-3xl tracking-tighter font-bold">Sign In</h1>
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
    <button
      type="submit"
      onclick={signIn}
      class="bg-default hover:bg-hover active:bg-active transition-colors border p-2 mt-4 cursor-pointer font-mono flex flex-row gap-1 justify-center"
    >
      <Heart />

      Proceed
    </button>
  </div>
</div>
