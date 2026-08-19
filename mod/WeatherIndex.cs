#nullable enable

using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Net.Http;
using BepInEx;
using Newtonsoft.Json;
using RoR2;
using RoR2.Skills;
using RoR2.Stats;

namespace WeatherIndex
{
    [BepInPlugin(PluginGUID, PluginName, PluginVersion)]
    [BepInDependency("com.rune580.riskofoptions", BepInDependency.DependencyFlags.HardDependency)]
    public class WeatherIndex : BaseUnityPlugin
    {
        public const string PluginGUID = PluginAuthor + "." + PluginName;
        public const string PluginAuthor = "Shuflduf";
        public const string PluginName = "WeatherIndex";
        public const string PluginVersion = "1.0.2";

        internal static readonly ConcurrentQueue<Action> mainThreadQueue = new();
        internal static readonly HttpClient http = new();
        internal static string? lastRun;
        internal static bool uploadedRun = true;
        internal static string pluginDir = System.IO.Path.GetDirectoryName(
            typeof(WeatherIndex).Assembly.Location
        );

        public void Awake()
        {
            Log.Init(Logger);

            Run.onRunStartGlobal += (Run run) =>
            {
                RunTracker.Reset();
                uploadedRun = false;
            };

            Run.onClientGameOverGlobal += (Run run, RunReport report) =>
            {
                RunReport.PlayerInfo? player = report.playerInfos?[0];

                StatSheet stats = player!.statSheet;
                Dictionary<int, int> itemCounts = getItemCounts(player.itemStacks);
                List<string> artifacts = new List<string>();
                foreach (ArtifactDef def in RoR2.ArtifactCatalog.artifactDefs)
                {
                    if (report.ruleBook.GenerateArtifactMask().HasArtifact(def.artifactIndex))
                    {
                        artifacts.Add(def.cachedName);
                    }
                }

                List<int> skillList = new List<int>();
                Loadout.BodyLoadoutManager loadout = player.master.loadout.bodyLoadoutManager;
                GenericSkill[] slots = BodyCatalog.GetBodyPrefabSkillSlots(player.bodyIndex);
                for (int i = 0; i < slots.Length; i++)
                {
                    uint variant = loadout.GetSkillVariant(player.bodyIndex, i);
                    SkillDef def = slots[i].skillFamily.variants[variant].skillDef;
                    skillList.Add(def.skillIndex);
                }

                // player.master.inventory.abili
                object info = new
                {
                    // run info
                    survivor = player.bodyName,
                    skills = skillList,
                    ending = report.gameEnding.cachedName,
                    startTime = report.runStartTimeUtc,
                    difficulty = DifficultyCatalog
                        .GetDifficultyDef(report.ruleBook.FindDifficulty())
                        .nameToken,
                    timeAliveSeconds = (ulong)stats.GetStatValueAsDouble(StatDef.totalTimeAlive),
                    artifacts = artifacts,
                    stagesCompleted = stats.GetStatValueULong(StatDef.totalStagesCompleted),
                    stageHistory = RunTracker.stages,

                    // items
                    items = itemCounts,
                    equipment = player.equipment.Length > 0
                        ? player.equipment[0]
                        : EquipmentIndex.None,
                    itemsCollected = stats.GetStatValueULong(StatDef.totalItemsCollected),
                    itemHistory = RunTracker.items,
                    equipmentHistory = RunTracker.equipments,

                    // drones
                    dronesPurchased = stats.GetStatValueULong(StatDef.totalDronesPurchased),
                    turretsPurchased = stats.GetStatValueULong(StatDef.totalTurretsPurchased),

                    // combat
                    kills = stats.GetStatValueULong(StatDef.totalKills),
                    eliteKills = stats.GetStatValueULong(StatDef.totalEliteKills),
                    minionKills = stats.GetStatValueULong(StatDef.totalMinionKills),
                    deaths = stats.GetStatValueULong(StatDef.totalDeaths),

                    // damage
                    damageDealt = stats.GetStatValueULong(StatDef.totalDamageDealt),
                    minionDamageDealt = stats.GetStatValueULong(StatDef.totalMinionDamageDealt),
                    damageTaken = stats.GetStatValueULong(StatDef.totalDamageTaken),
                    highestDamageDealt = stats.GetStatValueULong(StatDef.highestDamageDealt),

                    // healing
                    healingRecieved = stats.GetStatValueULong(StatDef.totalHealthHealed),

                    // progression
                    highestLevel = stats.GetStatValueULong(StatDef.highestLevel),
                    goldCollected = stats.GetStatValueULong(StatDef.goldCollected),
                    purchases = stats.GetStatValueULong(StatDef.totalPurchases),
                    goldPurchases = stats.GetStatValueULong(StatDef.totalGoldPurchases),
                    bloodPurchases = stats.GetStatValueULong(StatDef.totalBloodPurchases),
                    lunarPurchases = stats.GetStatValueULong(StatDef.totalLunarPurchases),

                    // movement
                    distanceTraveled = (ulong)
                        stats.GetStatValueAsDouble(StatDef.totalDistanceTraveled),
                };

                string json = JsonConvert.SerializeObject(
                    info,
                    Formatting.None,
                    new JsonSerializerSettings
                    {
                        ReferenceLoopHandling = ReferenceLoopHandling.Ignore,
                        NullValueHandling = NullValueHandling.Ignore,
                    }
                );
                Log.Debug(json);
                lastRun = json;
            };

            On.RoR2.UI.GameEndReportPanelController.Awake += (orig, self) =>
            {
                orig(self);
                self.gameObject.AddComponent<SubmitButton>().Init(self);
            };

            WIConfig.Init(this);
            RunTracker.Init();
            Debug.Init();
            DataDumper.Init();

            WIBridge.RefreshStatus(false);
        }

        internal static void MainThread(Action action)
        {
            mainThreadQueue.Enqueue(action);
        }

        private Dictionary<int, int> getItemCounts(int[] itemStacks)
        {
            Dictionary<int, int> itemCounts = new();
            for (int i = 0; i < itemStacks.Length; i++)
            {
                if (itemStacks[i] > 0)
                {
                    itemCounts[i] = itemStacks[i];
                }
            }
            return itemCounts;
        }

        private void Update()
        {
            while (mainThreadQueue.TryDequeue(out Action action))
            {
                action();
            }

            Debug.SkipKeybind();
        }
    }
}
