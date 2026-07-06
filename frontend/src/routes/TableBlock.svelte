<script lang="ts">
  let {
    children = undefined,
    order,
    idx,
    style: styleName = undefined,
  } = $props();
</script>

<div
  class="border p-4 bg-bg-secondary flex items-center"
  style="order: {order}; grid-row: {idx + 2}; {styleName}"
>
  {@render children?.()}
</div>
