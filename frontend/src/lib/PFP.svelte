<script lang="ts">
  import { validUrl } from "$lib";

  let { src, class: className }: { src?: string; class: string } = $props();
</script>

<img
  src={validUrl(src) ? src : "/fallback_pfp.png"}
  alt="pfp"
  class={className}
  onerror={(ev: any) => (ev.target.src = "/fallback_pfp.png")}
/>
