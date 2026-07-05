<script lang="ts">
  let {
    user,
  }: {
    user: { displayName: string | null; username: string; image: string };
  } = $props();
</script>

<div class="flex flex-row h-full justify-center items-center gap-2 w-max">
  <img src={user.image} alt="profile" class="h-16 rounded-full" />
  <span class="text-primary text-lg">
    {user.displayName ?? user.username}
  </span>
</div>
