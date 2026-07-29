<script lang="ts">
  import { createApiReference } from "@scalar/api-reference";
  import "@scalar/api-reference/style.css";
  import { onMount } from "svelte";

  let openApiPromise: Promise<null> = $state(new Promise(() => {}));
  let app: HTMLElement | null = $state(null);

  onMount(() => {
    openApiPromise = fetch("/openapi-spec.yaml")
      .then((r) => r.text())
      .then((spec) => {
        createApiReference(app!, {
          content: spec,
        });
        return null;
      });
  });
</script>

<div bind:this={app}></div>
