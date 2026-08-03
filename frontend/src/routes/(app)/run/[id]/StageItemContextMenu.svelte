<script lang="ts">
  import {
    EQUIPMENTS,
    formatSeconds,
    ITEMS,
    type StageInteractable,
  } from "$lib/RoR2";

  let shown: boolean = $state(false);
  let pos: [number, number] = $state([0, 0]);
  let info: StageInteractable | null = $state(null);
  let isEquipment: boolean = $state(false);
  let popup: any = $state();

  export function show(e: MouseEvent, newInfo: StageInteractable) {
    pos = [e.clientX, e.clientY];
    info = newInfo;
    isEquipment = info.name == "EQUIPMENTBARREL_NAME";
    shown = true;
  }
  export function update(e: MouseEvent) {
    pos = [
      Math.min(
        e.clientX,
        window.innerWidth - popup.getBoundingClientRect().width,
      ),
      e.clientY,
    ];
  }
  export function hide() {
    shown = false;
  }
</script>

{#if shown}
  <div
    class="border bg-bg-secondary p-4 fixed z-10 pointer-events-none"
    style="left: {pos[0]}px; top: {pos[1]}px;"
    bind:this={popup}
  >
    <b>
      {#if isEquipment}
        {EQUIPMENTS[info!.item].displayName}
      {:else}
        {ITEMS[info!.item].displayName}
      {/if}
    </b>
    <p>
      {info?.name}
    </p>
    {#if info!.time}
      {formatSeconds(info!.time)}
    {/if}
  </div>
{/if}
