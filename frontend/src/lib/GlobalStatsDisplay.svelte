<script lang="ts">
  import { defaultProperties } from "./properties";
  import { formatBig, formatSeconds, type RunReportWithUser } from "./RoR2";

  let { stats }: { stats: RunReportWithUser } = $props();
</script>

<div class="flex flex-row flex-wrap">
  {#each Object.entries(stats) as [stat, value]}
    {#snippet basicStat(name: string)}
      {#if stat == name}
        <span class="text-yellow-200">{formatBig(Number(value))}</span>
      {/if}
    {/snippet}
    <div
      class="min-w-80 p-4 bg-bg-secondary border flex flex-row justify-between flex-1"
    >
      <span>{defaultProperties[stat].name}</span>

      {#if stat == "timeAliveSeconds"}
        <span class="text-yellow-200">{formatSeconds(Number(value))}</span>
      {/if}
      {#if stat == "distanceTraveled"}
        <span>
          <span class="text-yellow-200">{formatBig(Number(value))}</span>
          m
        </span>
      {/if}
      {@render basicStat("stagesCompleted")}
      {@render basicStat("score")}
      {@render basicStat("itemsCollected")}
      {@render basicStat("dronesPurchased")}
      {@render basicStat("turretsPurchased")}
      {@render basicStat("kills")}
      {@render basicStat("eliteKills")}
      {@render basicStat("minionKills")}
      {@render basicStat("deaths")}
      {@render basicStat("damageDealt")}
      {@render basicStat("minionDamageDealt")}
      {@render basicStat("damageTaken")}
      {@render basicStat("highestDamageDealt")}
      {@render basicStat("healingRecieved")}
      {@render basicStat("highestLevel")}
      {@render basicStat("goldCollected")}
      {@render basicStat("purchases")}
      {@render basicStat("goldPurchases")}
      {@render basicStat("bloodPurchases")}
      {@render basicStat("lunarPurchases")}
    </div>
  {/each}
</div>
