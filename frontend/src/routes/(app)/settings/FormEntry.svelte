<script lang="ts">
  let { children, id, label } = $props();
</script>

<div class="flex w-md flex-row items-center justify-between">
  <label for={id} class="font-mono">{label}</label>
  {@render children?.()}
</div>
