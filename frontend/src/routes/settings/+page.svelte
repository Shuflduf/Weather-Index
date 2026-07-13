<script lang="ts">
  import { onMount } from "svelte";
  import ProfileSettings from "./ProfileSettings.svelte";
  import { ShieldUser, User } from "@lucide/svelte";
  import { SiDiscord, SiGithub } from "@icons-pack/svelte-simple-icons";

  let accounts: string[] = $state([]);

  onMount(() => {
    fetch("/auth/list-accounts")
      .then((r) => r.json())
      .then((j: any[]) => (accounts = j.map((a) => a.provider)));
  });
</script>

<div class="fixed flex flex-col">
  <a
    href="#profile"
    class="p-2 justify-center border bg-default hover:bg-hover active:bg-active flex flex-row gap-2 w-full"
  >
    <User />
    Profile
  </a>
  <a
    href="#account"
    class="p-2 justify-center border bg-default hover:bg-hover active:bg-active flex flex-row gap-2 w-full"
  >
    <ShieldUser />
    Account
  </a>
</div>

<div class="ml-40">
  <ProfileSettings />
  <hr class="my-8" />
  <div>
    <h1 id="account" class="text-3xl tracking-tighter">Account</h1>
    <div class="flex flex-row bg-bg-secondary border p-2 gap-2 w-max mt-4">
      <SiGithub />
      <span class="font-mono">Github:</span>
      <span>
        {accounts.includes("github") ? "Connected" : "Not Connected"}
      </span>
    </div>
    <div class="flex flex-row bg-bg-secondary border p-2 gap-2 w-max mt-4">
      <SiDiscord />
      <span class="font-mono">Discord:</span>
      <span>
        {accounts.includes("discord") ? "Connected" : "Not Connected"}
      </span>
    </div>
  </div>
</div>
