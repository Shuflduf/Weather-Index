<script lang="ts">
  import ProfileSettings from "./ProfileSettings.svelte";
  import AccountSettings from "./AccountSettings.svelte";
  import { ShieldUser, User } from "@lucide/svelte";
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
  <AccountSettings />
</div>
