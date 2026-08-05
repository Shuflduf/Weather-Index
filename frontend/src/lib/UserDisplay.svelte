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
  class="inline-flex w-max flex-row items-center justify-center gap-2 {className}"
>
  <PFP src={user.image} class="h-full border" />
  <a class="text-lg text-primary" href="/player/{user.username}">
    {user.displayUsername && user.displayUsername.length > 0
      ? user.displayUsername
      : user.username}
  </a>
</div>
