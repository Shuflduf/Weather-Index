<script lang="ts">
  let { src, class: className }: { src?: string; class: string } = $props();
</script>

<img src={src ? src : "/fallback_pfp.png"} alt="pfp" class={className} />
