<script lang="ts">
  let {
    user,
    class: className,
  }: {
    user: { displayName: string | null; username: string; image: string };
    class: string | undefined;
  } = $props();
</script>

<div
  class="inline-flex flex-row justify-center items-center gap-2 w-max {className}"
>
  <img src={user.image} alt="profile" class="h-full border" />
  <a class="text-primary text-lg" href="/player/{user.username}">
    {user.displayName ?? user.username}
  </a>
</div>
