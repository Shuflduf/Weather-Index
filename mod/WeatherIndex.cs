#nullable enable

using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Net;
using System.Net.Http;
using System.Net.Http.Headers;
using System.Text;
using System.Threading.Tasks;
using BepInEx;
using BepInEx.Configuration;
using Newtonsoft.Json;
using RiskOfOptions;
using RiskOfOptions.OptionConfigs;
using RiskOfOptions.Options;
using RoR2;
using RoR2.Stats;
using UnityEngine;

namespace WeatherIndex
{
    [BepInPlugin(PluginGUID, PluginName, PluginVersion)]
    [BepInDependency("com.rune580.riskofoptions", BepInDependency.DependencyFlags.HardDependency)]
    public class WeatherIndex : BaseUnityPlugin
    {
        public const string PluginGUID = PluginAuthor + "." + PluginName;
        public const string PluginAuthor = "Shuflduf";
        public const string PluginName = "WeatherIndex";
        public const string PluginVersion = "1.0.0";

        private static readonly ConcurrentQueue<Action> mainThreadQueue = new();
        private static readonly HttpClient http = new();
        private static ConfigEntry<KeyboardShortcut>? endRunKeybind;
        internal static ConfigEntry<string>? accessToken;
        private static ConfigEntry<string>? backendURL;
        private static ConfigEntry<string>? connectionStatus;
        private static bool connecting = false;
        private static string? lastRun;
        private static bool uploadedRun = true;

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
                    itemHistory = RunTracker.items,

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
                lastRun = json;
                // this.PostRunReport(json);
            };

            On.RoR2.UI.GameEndReportPanelController.Awake += (orig, self) =>
            {
                orig(self);
                self.gameObject.AddComponent<SubmitButton>().Init(self);
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

            backendURL = Config.Bind<string>(
                "Debug",
                "Backend URL",
                "https://wi-backend.shuflduf.hackclub.app",
                "Weather Index backend URL"
            );

            connectionStatus = Config.Bind<string>(
                "Account",
                "Status",
                "NOT CONNECTED",
                "Status of Weather Index connection"
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

            ModSettingsManager.AddOption(
                new StringInputFieldOption(
                    connectionStatus!,
                    new InputFieldConfig
                    {
                        name = "Status",
                        category = "General",
                        description =
                            "Status of Weather Index connection.\n\nPossible values: NOT CONNECTED, CONNECTED AS [username], CONNECTING, LOADING, ERROR\n\n Automatically updated when this page is loaded. Exit settings and re-open this page for the proper updated value.",
                    }
                )
            );

            RunTracker.Init();
            Debug.Init();
            DataDumper.Init();

            RefreshStatus(false);
        }

        internal enum SubmitRunResult
        {
            Success,
            NotLoggedIn,
            ServerError,
            NetworkError,
            AlreadyUploaded,
        }

        internal static async Task<SubmitRunResult> SubmitRun()
        {
            if (uploadedRun == true)
                return SubmitRunResult.AlreadyUploaded;

            uploadedRun = true;

            if (string.IsNullOrEmpty(accessToken?.Value))
            {
                uploadedRun = false;
                return SubmitRunResult.NotLoggedIn;
            }

            try
            {
                string url = $"{backendURL?.Value}/api/runs/new";
                var content = new StringContent(lastRun, Encoding.UTF8, "application/json");
                var request = new HttpRequestMessage(HttpMethod.Post, url) { Content = content };
                if (!string.IsNullOrEmpty(accessToken?.Value))
                {
                    request.Headers.Authorization = new AuthenticationHeaderValue(
                        "Bearer",
                        accessToken.Value
                    );
                }
                var response = await http.SendAsync(request);
                Log.Info(await response.Content.ReadAsStringAsync());
                if (response.IsSuccessStatusCode)
                    return SubmitRunResult.Success;
                else
                {
                    uploadedRun = false;
                    return SubmitRunResult.ServerError;
                }
            }
            catch (System.Exception e)
            {
                Log.Error(e);
                uploadedRun = false;
                return SubmitRunResult.NetworkError;
            }
        }

        internal static void MainThread(Action action)
        {
            mainThreadQueue.Enqueue(action);
        }

        private async void RefreshStatus(bool popupEnabled = true)
        {
            connectionStatus?.Value = "LOADING";
            string url = $"{backendURL?.Value}/auth/get-session";
            var request = new HttpRequestMessage(HttpMethod.Get, url);
            if (!string.IsNullOrEmpty(accessToken?.Value))
            {
                request.Headers.Authorization = new AuthenticationHeaderValue(
                    "Bearer",
                    accessToken.Value
                );
            }
            var response = await http.SendAsync(request);
            switch (response.StatusCode)
            {
                case HttpStatusCode.Unauthorized:
                    if (popupEnabled)
                        WIPopup.ShowMessage("Not connected. Please try again.");
                    connectionStatus?.Value = "NOT CONNECTED";
                    break;
                case HttpStatusCode.OK:
                    var json = await response.Content.ReadAsStringAsync();
                    var data = JsonConvert.DeserializeAnonymousType(
                        json,
                        new { user = new { username = "" } }
                    );
                    if (popupEnabled)
                        WIPopup.ShowMessage($"Connected succesfully as @{data.user.username}!");
                    connectionStatus?.Value = $"CONNECTED AS @{data.user.username}";
                    break;
                default:
                    string message = await response.Content.ReadAsStringAsync();
                    if (popupEnabled)
                        WIPopup.ShowMessage($"An error occured: {message}");
                    connectionStatus?.Value = "ERROR";
                    break;
            }
        }

        private async void OnConnectClick()
        {
            if (connecting)
                return;
            connecting = true;
            connectionStatus?.Value = "CONNECTING";

            string url = $"{backendURL?.Value}/auth/device/code";
            var resp = await http.PostAsync(
                url,
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

            Application.OpenURL(deviceUri);

            _ = Task.Run(async () =>
            {
                while (true)
                {
                    await Task.Delay(body.interval * 1000);
                    var poll = await http.PostAsync(
                        $"{backendURL?.Value}/auth/device/token",
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
                    Log.Info(await poll.Content.ReadAsStringAsync());

                    if (!poll.IsSuccessStatusCode)
                        continue;

                    var tokenBody = JsonConvert.DeserializeAnonymousType(
                        await poll.Content.ReadAsStringAsync(),
                        new { access_token = "" }
                    );
                    Log.Info(JsonConvert.SerializeObject(tokenBody));
                    accessToken?.Value = tokenBody.access_token;
                    connecting = false;
                    RefreshStatus(true);
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
            while (mainThreadQueue.TryDequeue(out var action))
            {
                action();
            }

            if (Run.instance && endRunKeybind!.Value.IsDown())
            {
                Run.instance.BeginGameOver(RoR2Content.GameEndings.MainEnding);
            }
        }
    }
}
