<script lang="ts">
  import "./layout.css";
  import "geist-svelte/font/sans";
  import "geist-svelte/font/mono";
  import favicon from "$lib/assets/favicon.svg";
  import Navbar from "$lib/Navbar.svelte";
  import { dev } from "$app/environment";
  import { inject } from "@vercel/analytics";
  inject({ mode: dev ? "development" : "production" });

  let { children } = $props();
</script>

<svelte:head><link rel="icon" href={favicon} /></svelte:head>
<Navbar />

{@render children()}
