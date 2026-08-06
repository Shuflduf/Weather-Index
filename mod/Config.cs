#nullable enable

using System.IO;
using BepInEx;
using BepInEx.Configuration;
using RiskOfOptions;
using RiskOfOptions.OptionConfigs;
using RiskOfOptions.Options;
using UnityEngine;

namespace WeatherIndex
{
    public class WIConfig
    {
        internal static ConfigEntry<KeyboardShortcut>? endRunKeybind;
        internal static ConfigEntry<string>? accessToken;
        internal static ConfigEntry<string>? backendURL;
        internal static ConfigEntry<string>? connectionStatus;

        private static Sprite loadIcon()
        {
            string path = Path.Combine(WeatherIndex.pluginDir, "icon_full.png");
            byte[] bytes = File.ReadAllBytes(path);
            Texture2D tex = new Texture2D(2, 2);
            ImageConversion.LoadImage(tex, bytes);
            return Sprite.Create(
                tex,
                new Rect(0, 0, tex.width, tex.height),
                new Vector2(0.5f, 0.5f)
            );
        }

        internal static void Init(BaseUnityPlugin plugin)
        {
            ModSettingsManager.SetModIcon(loadIcon());

            endRunKeybind = plugin.Config.Bind<KeyboardShortcut>(
                "Debug",
                "End Run",
                new KeyboardShortcut(KeyCode.F10),
                "fucking"
            );

            accessToken = plugin.Config.Bind<string>(
                "Account",
                "Access Token",
                "",
                "Weather Index access token"
            );

            backendURL = plugin.Config.Bind<string>(
                "Debug",
                "Backend URL",
                "https://wi-api.shuflduf.xyz",
                "Weather Index backend URL"
            );

            connectionStatus = plugin.Config.Bind<string>(
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
                    WIBridge.StartConnection
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

            ModSettingsManager.AddOption(
                new StringInputFieldOption(
                    backendURL,
                    new InputFieldConfig
                    {
                        name = "Backend URL",
                        category = "Debug",
                        description = "URL of Weather Index server.",
                    }
                )
            );
        }
    }
}
