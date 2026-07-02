using BepInEx;
using RoR2;
using UnityEngine;
using BepInEx.Configuration;
using RoR2.Stats;
using Newtonsoft.Json;
using System.Net.Http;
using System.Text;
using RiskOfOptions;
using RiskOfOptions.Options;

namespace WeatherIndex
{
    [BepInPlugin(PluginGUID, PluginName, PluginVersion)]

    [BepInDependency("com.rune580.riskofoptions")]

    public class WeatherIndex : BaseUnityPlugin
    {
        // The Plugin GUID should be a unique ID for this plugin,
        // which is human readable (as it is used in places like the config).
        // If we see this PluginGUID as it is on thunderstore,
        // we will deprecate this mod.
        // Change the PluginAuthor and the PluginName !
        public const string PluginGUID = PluginAuthor + "." + PluginName;
        public const string PluginAuthor = "Shuflduf";
        public const string PluginName = "WeatherIndex";
        public const string PluginVersion = "1.0.0";

        private static ConfigEntry<KeyboardShortcut> endRunKeybind;

        public void Awake()
        {
            Log.Init(Logger);

            Run.onClientGameOverGlobal += (Run run, RunReport report) =>
            {
                var player = report.playerInfos?[0];
                var stats = player.statSheet;
                var info = new
                {
                    // run info
                    survivor = player.bodyName,
                    ending = report.gameEnding.cachedName,
                    startTime = report.runStartTimeUtc,
                    difficulty = DifficultyCatalog.GetDifficultyDef(report.ruleBook.FindDifficulty()).nameToken,
                    timeAliveSeconds = (ulong)stats.GetStatValueAsDouble(StatDef.totalTimeAlive),
                    stagesCompleted = stats.GetStatValueULong(StatDef.totalStagesCompleted),

                    // items
                    itemsCollected = stats.GetStatValueULong(StatDef.totalItemsCollected),

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
                    goldSpent = stats.GetStatValueULong(StatDef.totalGoldPurchases),
                    lunarCoinsSpent = stats.GetStatValueULong(StatDef.totalLunarPurchases),
                    purchases = stats.GetStatValueULong(StatDef.totalPurchases),
                    bloodPurchases = stats.GetStatValueULong(StatDef.totalBloodPurchases),

                    // movement
                    distanceTraveledMetres = (ulong)stats.GetStatValueAsDouble(StatDef.totalDistanceTraveled),
                };
                var json = JsonConvert.SerializeObject(info, Formatting.Indented, new JsonSerializerSettings
                {
                    ReferenceLoopHandling = ReferenceLoopHandling.Ignore,
                    NullValueHandling = NullValueHandling.Ignore,
                });
                Log.Info(info);
                this.PostRunReport(json);
            };

            On.EntityStates.GameOver.RoR2MainEndingPlayCutscene.FixedUpdate += (orig, self) =>
                {
                    orig(self);
                    self.outer.SetNextStateToMain();
                };

            On.EntityStates.GameOver.ShowCredits.OnEnter += (orig, self) =>
                {
                    orig(self);
                    self.outer.SetNextState(new EntityStates.GameOver.ShowReport());
                };

            endRunKeybind = Config.Bind<KeyboardShortcut>("Debug", "End Run", new KeyboardShortcut(KeyCode.F10), "fucking");

            ModSettingsManager.AddOption(new GenericButtonOption(
                "Link Account", "General",
                "Connects your Weather Index account to Risk of Rain 2. \n\nWill open your browser for authentication.",
                "Connect", OnConnectClick
            ));
        }

        private async void PostRunReport(string json)
        {
            using var client = new HttpClient();
            var content = new StringContent(json, Encoding.UTF8, "application/json");
            var response = await client.PostAsync("http://localhost:3000/new-run", content);
        }

        private void OnConnectClick()
        {
            Log.Info("shit");
        }

        private void Update()
        {
            if (Run.instance && endRunKeybind.Value.IsDown())
            {
                Run.instance.BeginGameOver(RoR2Content.GameEndings.MainEnding);
            }
        }
    }
}
