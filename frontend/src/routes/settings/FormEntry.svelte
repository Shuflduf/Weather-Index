<script lang="ts">
  let { children, id, label } = $props();
</script>

<div class="flex flex-row justify-between w-md items-center">
  <label for={id} class="font-mono">{label}</label>
  {@render children?.()}
</div>
