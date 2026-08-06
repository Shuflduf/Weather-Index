<script lang="ts">
  import ProfileSettings from "./ProfileSettings.svelte";
  import AccountSettings from "./AccountSettings.svelte";
  import { Logs, ShieldUser, User } from "@lucide/svelte";
  import SessionSettings from "./SessionSettings.svelte";
</script>

<svelte:head>
  <title>WI | Settings</title>
</svelte:head>

<div class="fixed flex flex-col">
  <a
    href="#profile"
    class="flex w-full flex-row justify-center gap-2 border bg-default p-2 hover:bg-hover active:bg-active"
  >
    <User />
    Profile
  </a>
  <a
    href="#account"
    class="flex w-full flex-row justify-center gap-2 border bg-default p-2 hover:bg-hover active:bg-active"
  >
    <ShieldUser />
    Account
  </a>
  <a
    href="#sessions"
    class="flex w-full flex-row justify-center gap-2 border bg-default p-2 hover:bg-hover active:bg-active"
  >
    <Logs />
    Sessions
  </a>
</div>

<div class="ml-40">
  <ProfileSettings />
  <hr class="my-8" />
  <AccountSettings />
  <hr class="my-8" />
  <SessionSettings />
</div>
