<script lang="ts">
  import { setToken, token } from "$lib/auth";
  import TableView from "$lib/TableView.svelte";
  import { onMount } from "svelte";
</script>

<svelte:head>
  <title>WI | Runs</title>
</svelte:head>

<TableView />
