<script lang="ts">
  import "./layout.css";
  import "geist-svelte/font/sans";
  import "geist-svelte/font/mono";
  import favicon from "$lib/assets/favicon.svg";

  let { children } = $props();
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>
{@render children()}
