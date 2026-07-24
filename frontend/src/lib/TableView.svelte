<script lang="ts">
  import type { Property } from "$lib";
  import Table from "../routes/Table.svelte";
  import { defaultProperties } from "./properties";
  import type { RunReportWithUser } from "./RoR2";

  let {
    runs,
    openContextMenu = null,
  }: {
    runs: RunReportWithUser[];
    openContextMenu?: ((e: MouseEvent, id: string) => null) | null;
  } = $props();

  let properties: Record<string, Property> = $state(
    structuredClone(defaultProperties),
  );

  export function loadMore() {}
</script>

<Table {properties} {runs} {openContextMenu} />
