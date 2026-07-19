#nullable enable

using System.Collections.Generic;
using System.Net.Http;
using System.Net.Http.Headers;
using System.Text;
using System.Threading.Tasks;
using BepInEx;
using BepInEx.Configuration;
using Newtonsoft.Json;
using RiskOfOptions;
using RiskOfOptions.Options;
using RoR2;
using RoR2.Stats;
using UnityEngine;

namespace WeatherIndex
{
    [BepInPlugin(PluginGUID, PluginName, PluginVersion)]
    [BepInDependency("com.rune580.riskofoptions")]
    public class WeatherIndex : BaseUnityPlugin
    {
        public const string PluginGUID = PluginAuthor + "." + PluginName;
        public const string PluginAuthor = "Shuflduf";
        public const string PluginName = "WeatherIndex";
        public const string PluginVersion = "1.0.0";

        private static readonly HttpClient http = new();
        private static string backendURL = "http://localhost:3000";
        private static ConfigEntry<KeyboardShortcut>? endRunKeybind;
        private static ConfigEntry<string>? accessToken;

        public void Awake()
        {
            Log.Init(Logger);

            Run.onRunStartGlobal += (Run run) =>
            {
                RunTracker.items = new List<ItemEvent>();
                RunTracker.stages = new List<string>();
            };

            Run.onClientGameOverGlobal += (Run run, RunReport report) =>
            {
                var player = report.playerInfos?[0];
                var stats = player!.statSheet;
                var itemCounts = getItemCounts(player.itemStacks);
                List<string> artifacts = new List<string>();
                foreach (var def in RoR2.ArtifactCatalog.artifactDefs)
                {
                    if (report.ruleBook.GenerateArtifactMask().HasArtifact(def.artifactIndex))
                    {
                        artifacts.Add(def.cachedName);
                    }
                }

                var info = new
                {
                    // run info
                    survivor = player.bodyName,
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
                    purchases = stats.GetStatValueULong(StatDef.totalPurchases),
                    goldPurchases = stats.GetStatValueULong(StatDef.totalGoldPurchases),
                    bloodPurchases = stats.GetStatValueULong(StatDef.totalBloodPurchases),
                    lunarPurchases = stats.GetStatValueULong(StatDef.totalLunarPurchases),

                    // movement
                    distanceTraveledMetres = (ulong)
                        stats.GetStatValueAsDouble(StatDef.totalDistanceTraveled),
                };

                var json = JsonConvert.SerializeObject(
                    info,
                    Formatting.None,
                    new JsonSerializerSettings
                    {
                        ReferenceLoopHandling = ReferenceLoopHandling.Ignore,
                        NullValueHandling = NullValueHandling.Ignore,
                    }
                );
                Log.Info(json);
                this.PostRunReport(json);
            };
            endRunKeybind = Config.Bind<KeyboardShortcut>(
                "Debug",
                "End Run",
                new KeyboardShortcut(KeyCode.F10),
                "fucking"
            );

            accessToken = Config.Bind<string>(
                "Account",
                "Access Token",
                "",
                "Weather Index access token"
            );

            ModSettingsManager.AddOption(
                new GenericButtonOption(
                    "Link Account",
                    "General",
                    "Connects your Weather Index account to Risk of Rain 2. \n\nWill open your browser for authentication.",
                    "Connect",
                    OnConnectClick
                )
            );

            RunTracker.Init();
            Debug.Init();
            DataDumper.Init();
        }

        private async void PostRunReport(string json)
        {
            var content = new StringContent(json, Encoding.UTF8, "application/json");
            var request = new HttpRequestMessage(HttpMethod.Post, $"{backendURL}/new-run")
            {
                Content = content,
            };
            if (!string.IsNullOrEmpty(accessToken?.Value))
            {
                request.Headers.Authorization = new AuthenticationHeaderValue(
                    "Bearer",
                    accessToken.Value
                );
            }
            var response = await http.SendAsync(request);
            Log.Info(await response.Content.ReadAsStringAsync());
        }

        private async void OnConnectClick()
        {
            var resp = await http.PostAsync(
                $"{backendURL}/auth/device/code",
                new StringContent(
                    JsonConvert.SerializeObject(new { client_id = "weather-index-mod" }),
                    Encoding.UTF8,
                    "application/json"
                )
            );
            var body = JsonConvert.DeserializeAnonymousType(
                await resp.Content.ReadAsStringAsync(),
                new
                {
                    device_code = "",
                    user_code = "",
                    verification_uri = "",
                    interval = 5,
                    expires_in = 1800,
                }
            );

            var deviceUri = $"{body.verification_uri}?user_code={body.user_code}";
            // Log.Info(deviceUri);

            Application.OpenURL(deviceUri);

            _ = Task.Run(async () =>
            {
                while (true)
                {
                    await Task.Delay(body.interval * 1000);
                    var poll = await http.PostAsync(
                        $"{backendURL}/auth/device/token",
                        new StringContent(
                            JsonConvert.SerializeObject(
                                new
                                {
                                    grant_type = "urn:ietf:params:oauth:grant-type:device_code",
                                    device_code = body.device_code,
                                    client_id = "weather-index-mod",
                                }
                            ),
                            Encoding.UTF8,
                            "application/json"
                        )
                    );

                    if (!poll.IsSuccessStatusCode)
                        continue;

                    var tokenBody = JsonConvert.DeserializeAnonymousType(
                        await poll.Content.ReadAsStringAsync(),
                        new { access_token = "" }
                    );
                    accessToken!.Value = tokenBody.access_token;
                    // Log.Info(await poll.Content.ReadAsStringAsync());
                    break;
                }
            });
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
            if (Run.instance && endRunKeybind!.Value.IsDown())
            {
                Run.instance.BeginGameOver(RoR2Content.GameEndings.MainEnding);
            }
        }
    }
}
