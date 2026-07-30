<script lang="ts">
  import { createApiReference } from "@scalar/api-reference";
  import "@scalar/api-reference/style.css";
  import { onMount } from "svelte";

  let app: HTMLElement | null = $state(null);

  onMount(() => {
    fetch("/openapi-spec.yaml")
      .then((r) => r.text())
      .then((spec) => {
        createApiReference(app!, {
          content: spec,
          defaultOpenAllTags: true,
          agent: { disabled: true },
          mcp: { disabled: true },
          hideClientButton: true,
        });
      });

    return () => {
      document.body.classList.remove("dark-mode", "light-mode");
    };
  });
</script>

<div bind:this={app}></div>
