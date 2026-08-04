#nullable enable

using System.Net;
using System.Net.Http;
using System.Net.Http.Headers;
using System.Text;
using System.Threading.Tasks;
using Newtonsoft.Json;
using UnityEngine;

namespace WeatherIndex
{
    internal enum SubmitRunResult
    {
        Success,
        NotLoggedIn,
        ServerError,
        NetworkError,
        AlreadyUploaded,
    }

    public class WIBridge
    {
        private static bool connecting = false;

        internal static async Task<SubmitRunResult> SubmitRun()
        {
            if (WeatherIndex.uploadedRun == true)
                return SubmitRunResult.AlreadyUploaded;

            WeatherIndex.uploadedRun = true;

            if (string.IsNullOrEmpty(WIConfig.accessToken?.Value))
            {
                WeatherIndex.uploadedRun = false;
                return SubmitRunResult.NotLoggedIn;
            }

            try
            {
                string url = $"{WIConfig.backendURL?.Value}/runs/new";
                var content = new StringContent(
                    WeatherIndex.lastRun,
                    Encoding.UTF8,
                    "application/json"
                );
                var request = new HttpRequestMessage(HttpMethod.Post, url) { Content = content };
                if (!string.IsNullOrEmpty(WIConfig.accessToken?.Value))
                {
                    request.Headers.Authorization = new AuthenticationHeaderValue(
                        "Bearer",
                        WIConfig.accessToken.Value
                    );
                }
                var response = await WeatherIndex.http.SendAsync(request);
                Log.Info(await response.Content.ReadAsStringAsync());
                if (response.IsSuccessStatusCode)
                    return SubmitRunResult.Success;
                else
                {
                    WeatherIndex.uploadedRun = false;
                    return SubmitRunResult.ServerError;
                }
            }
            catch (System.Exception e)
            {
                Log.Error(e);
                WeatherIndex.uploadedRun = false;
                return SubmitRunResult.NetworkError;
            }
        }

        internal static async void RefreshStatus(bool popupEnabled = true)
        {
            WIConfig.connectionStatus?.Value = "LOADING";
            string url = $"{WIConfig.backendURL?.Value}/auth/get-session";
            var request = new HttpRequestMessage(HttpMethod.Get, url);
            if (!string.IsNullOrEmpty(WIConfig.accessToken?.Value))
            {
                request.Headers.Authorization = new AuthenticationHeaderValue(
                    "Bearer",
                    WIConfig.accessToken.Value
                );
            }
            var response = await WeatherIndex.http.SendAsync(request);
            switch (response.StatusCode)
            {
                case HttpStatusCode.Unauthorized:
                    if (popupEnabled)
                        WIPopup.ShowMessage("Not connected. Please try again.");
                    WIConfig.connectionStatus?.Value = "NOT CONNECTED";
                    break;
                case HttpStatusCode.OK:
                    var json = await response.Content.ReadAsStringAsync();
                    var data = JsonConvert.DeserializeAnonymousType(
                        json,
                        new { user = new { username = "" } }
                    );
                    if (popupEnabled)
                        WIPopup.ShowMessage($"Connected succesfully as @{data.user.username}!");
                    WIConfig.connectionStatus?.Value = $"CONNECTED AS @{data.user.username}";
                    break;
                default:
                    string message = await response.Content.ReadAsStringAsync();
                    if (popupEnabled)
                        WIPopup.ShowMessage($"An error occured: {message}");
                    WIConfig.connectionStatus?.Value = "ERROR";
                    break;
            }
        }

        internal static async void StartConnection()
        {
            if (connecting)
                return;
            connecting = true;
            WIConfig.connectionStatus?.Value = "CONNECTING";

            string url = $"{WIConfig.backendURL?.Value}/auth/device/code";
            var resp = await WeatherIndex.http.PostAsync(
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
                    var poll = await WeatherIndex.http.PostAsync(
                        $"{WIConfig.backendURL?.Value}/auth/device/token",
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
                    WIConfig.accessToken?.Value = tokenBody.access_token;
                    connecting = false;
                    RefreshStatus(true);
                    break;
                }
            });
        }
    }
}
