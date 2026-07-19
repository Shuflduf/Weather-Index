<script lang="ts">
  import PFP from "./PFP.svelte";

  let {
    user,
    class: className,
  }: {
    user: { displayUsername: string | null; username: string; image: string };
    class: string | undefined;
  } = $props();
</script>

<div
  class="inline-flex flex-row justify-center items-center gap-2 w-max {className}"
>
  <PFP src={user.image} class="h-full border" />
  <a class="text-primary text-lg" href="/player/{user.username}">
    {user.displayUsername && user.displayUsername.length > 0
      ? user.displayUsername
      : user.username}
  </a>
</div>
