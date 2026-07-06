<script lang="ts">
  let {
    children = undefined,
    order,
    idx,
    style: styleName = undefined,
  } = $props();
</script>

<div
  class="border p-4 bg-bg-secondary flex items-center transition-transform"
  style="order: {order}; grid-row: {idx + 2}; {styleName}"
  data-col-cell
>
  {@render children?.()}
</div>
