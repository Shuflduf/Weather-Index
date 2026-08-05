<script lang="ts">
  let {
    children = undefined,
    order,
    idx,
    style: styleName = undefined,
  } = $props();
</script>

<div
  class="flex items-center border bg-bg-secondary p-4 transition-transform"
  style="order: {order}; grid-row: {idx + 2}; {styleName}"
  data-col-cell
>
  {@render children?.()}
</div>
