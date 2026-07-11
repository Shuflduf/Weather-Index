#nullable enable

using System.Collections.Generic;
using System.IO;
using BepInEx;
using Newtonsoft.Json;
using RiskOfOptions;
using RiskOfOptions.Options;
using UnityEngine;

namespace WeatherIndex
{
    class DataDumper
    {
        private static string pluginDir = Path.Combine(Paths.PluginPath, "WeatherIndex");

        public static void Init()
        {
            ModSettingsManager.AddOption(
                new GenericButtonOption(
                    "Dump Items",
                    "Debug",
                    "Dumps all item data into the plugin folder",
                    "Dump",
                    DataDumper.DumpItems
                )
            );
            ModSettingsManager.AddOption(
                new GenericButtonOption(
                    "Dump Bodies",
                    "Debug",
                    "Dumps all body data (survivors/enemies/etc) into the plugin folder",
                    "Dump",
                    DataDumper.DumpBodies
                )
            );
            ModSettingsManager.AddOption(
                new GenericButtonOption(
                    "Dump Endings",
                    "Debug",
                    "Dumps all game endings into the plugin folder",
                    "Dump",
                    DataDumper.DumpEndings
                )
            );
            ModSettingsManager.AddOption(
                new GenericButtonOption(
                    "Dump Difficulties",
                    "Debug",
                    "i frogot",
                    "Dump",
                    DataDumper.DumpDifficulties
                )
            );
            ModSettingsManager.AddOption(
                new GenericButtonOption(
                    "Dump Item Tiers",
                    "Debug",
                    "🐸🚀",
                    "Dump",
                    DataDumper.DumpItemTiers
                )
            );
            ModSettingsManager.AddOption(
                new GenericButtonOption(
                    "Dump Artifacts",
                    "Debug",
                    "artifacts",
                    "Dump",
                    DataDumper.DumpArtifacts
                )
            );
        }

        public static void DumpItems()
        {
            var outputDir = Path.Combine(pluginDir, "items");
            Directory.CreateDirectory(outputDir);
            var items = new List<object>();
            foreach (var idx in RoR2.ItemCatalog.allItems)
            {
                if (idx == RoR2.ItemIndex.None)
                    continue;

                var def = RoR2.ItemCatalog.GetItemDef(idx);
                if (def == null)
                    continue;

                string? filename = null;
                var sprite = def.pickupIconSprite;
                if (sprite == null || sprite.texture == null)
                    continue;
                Texture2D tex = sprite.texture;
                if (tex != null)
                {
                    filename = $"{def.name}.png";
                    writeTexture(Path.Combine(outputDir, filename), tex);
                }

                string displayName = RoR2.Language.GetString(def.nameToken);
                bool helper = string.IsNullOrEmpty(def.nameToken) || displayName == def.nameToken;

                items.Add(
                    new
                    {
                        id = (int)idx,
                        name = def.name,
                        nameToken = def.nameToken,
                        displayName = displayName,
                        tier = RoR2.ItemTierCatalog.GetItemTierDef(def.tier)?.name,
                        helper = helper,
                        icon = filename,
                    }
                );
            }
            var json = JsonConvert.SerializeObject(items);
            File.WriteAllText(Path.Combine(outputDir, "items.json"), json);
        }

        public static void DumpBodies()
        {
            var outputDir = Path.Combine(pluginDir, "bodies");
            Directory.CreateDirectory(outputDir);
            var bodies = new List<object>();
            foreach (var bodyPrefab in RoR2.BodyCatalog.allBodyPrefabs)
            {
                var body = bodyPrefab.GetComponent<RoR2.CharacterBody>();
                if (body == null)
                    continue;

                Texture2D tex = (Texture2D)body.portraitIcon;
                string? filename = null;
                if (tex != null)
                {
                    filename = $"{bodyPrefab.name}.png";
                    writeTexture(Path.Combine(outputDir, filename), tex);
                }
                var survivor =
                    RoR2.SurvivorCatalog.GetSurvivorIndexFromBodyIndex(body.bodyIndex)
                    != RoR2.SurvivorIndex.None;
                bodies.Add(
                    new
                    {
                        name = body.name,
                        nameToken = body.baseNameToken,
                        survivor = survivor,
                        displayName = RoR2.Language.GetString(body.baseNameToken),
                        icon = filename,
                    }
                );
            }
            var json = JsonConvert.SerializeObject(bodies);
            File.WriteAllText(Path.Combine(outputDir, "bodies.json"), json);
        }

        public static void DumpEndings()
        {
            var outputDir = Path.Combine(pluginDir, "endings");
            Directory.CreateDirectory(outputDir);
            var endings = new List<object>();
            foreach (var def in RoR2.GameEndingCatalog.gameEndingDefs)
            {
                if (def == null)
                    continue;

                var sprite = def.icon;
                if (sprite == null || sprite.texture == null)
                    continue;

                Texture2D tex = sprite.texture;
                if (tex == null)
                    continue;

                string filename = $"{def.cachedName}.png";
                writeEndingTexture(Path.Combine(outputDir, filename), tex, def.foregroundColor);

                endings.Add(
                    new
                    {
                        name = def.cachedName,
                        nameToken = def.endingTextToken,
                        endingMessage = RoR2.Language.GetString(def.endingTextToken),
                        isWin = def.isWin,
                        icon = filename,
                        colorFg = ToHex(def.foregroundColor),
                        colorBg = ToHex(def.backgroundColor),
                    }
                );
            }
            var json = JsonConvert.SerializeObject(endings);
            File.WriteAllText(Path.Combine(outputDir, "endings.json"), json);
        }

        public static void DumpDifficulties()
        {
            var outputDir = Path.Combine(pluginDir, "difficulties");
            Directory.CreateDirectory(outputDir);
            var difficulties = new List<object>();
            foreach (
                RoR2.DifficultyIndex idx in System.Enum.GetValues(typeof(RoR2.DifficultyIndex))
            )
            {
                if (idx == RoR2.DifficultyIndex.Invalid || idx == RoR2.DifficultyIndex.Count)
                    continue;

                var def = RoR2.DifficultyCatalog.GetDifficultyDef(idx);
                if (def == null)
                    continue;

                var sprite = def.GetIconSprite();
                if (sprite == null)
                    continue;

                string? filename = null;
                Texture2D tex = sprite.texture;
                if (tex != null)
                {
                    filename = $"{def.nameToken}.png";
                    writeTexture(Path.Combine(outputDir, filename), tex);
                }

                difficulties.Add(
                    new
                    {
                        nameToken = def.nameToken,
                        displayName = RoR2.Language.GetString(def.nameToken),
                        icon = filename,
                    }
                );
            }
            var json = JsonConvert.SerializeObject(difficulties);
            File.WriteAllText(Path.Combine(outputDir, "difficulties.json"), json);
        }

        public static void DumpItemTiers()
        {
            var outputDir = Path.Combine(pluginDir, "tiers");
            Directory.CreateDirectory(outputDir);
            var tiers = new List<object>();
            var tierOrder = new RoR2.ItemTier[]
            {
                RoR2.ItemTier.NoTier,
                RoR2.ItemTier.AssignedAtRuntime,
                RoR2.ItemTier.Lunar,
                RoR2.ItemTier.Tier1,
                RoR2.ItemTier.VoidTier1,
                RoR2.ItemTier.Tier2,
                RoR2.ItemTier.VoidTier2,
                RoR2.ItemTier.Tier3,
                RoR2.ItemTier.VoidTier3,
                RoR2.ItemTier.Boss,
                RoR2.ItemTier.VoidBoss,
                RoR2.ItemTier.FoodTier,
            };
            foreach (var def in RoR2.ItemTierCatalog.allItemTierDefs)
            {
                tiers.Add(
                    new { name = def.name, sort = System.Array.IndexOf(tierOrder, def.tier) }
                );
            }
            var json = JsonConvert.SerializeObject(tiers);
            File.WriteAllText(Path.Combine(outputDir, "tiers.json"), json);
        }

        public static void DumpArtifacts()
        {
            var outputDir = Path.Combine(pluginDir, "artifacts");
            Directory.CreateDirectory(outputDir);
            var artifacts = new List<object>();
            foreach (var def in RoR2.ArtifactCatalog.artifactDefs)
            {
                if (def == null)
                    continue;

                var sprite = def.smallIconSelectedSprite;
                if (sprite == null)
                    continue;

                string? filename = null;
                Texture2D tex = sprite.texture;
                if (tex != null)
                {
                    filename = $"{def.cachedName}.png";
                    writeTexture(Path.Combine(outputDir, filename), tex);
                }

                artifacts.Add(
                    new
                    {
                        name = def.cachedName,
                        nameToken = def.nameToken,
                        displayName = RoR2.Language.GetString(def.nameToken),
                        icon = filename,
                    }
                );
            }
            var json = JsonConvert.SerializeObject(artifacts);
            File.WriteAllText(Path.Combine(outputDir, "artifacts.json"), json);
        }

        private static void writeEndingTexture(string path, Texture2D tex, Color color)
        {
            var readable = new Texture2D(tex.width, tex.height, TextureFormat.RGBA32, false);
            RenderTexture current = RenderTexture.active;
            var rt = RenderTexture.GetTemporary(tex.width, tex.height);
            Graphics.Blit(tex, rt);
            RenderTexture.active = rt;
            readable.ReadPixels(new Rect(0, 0, rt.width, rt.height), 0, 0);
            readable.Apply();
            RenderTexture.active = current;
            RenderTexture.ReleaseTemporary(rt);

            var pixels = readable.GetPixels32();
            for (int i = 0; i < pixels.Length; i++)
            {
                float luminance = (pixels[i].r + pixels[i].g + pixels[i].b) / (3f * 255f);

                pixels[i] = new Color32(
                    (byte)(255),
                    (byte)(255),
                    (byte)(255),
                    (byte)(luminance * 255)
                );
            }
            readable.SetPixels32(pixels);
            readable.Apply();
            File.WriteAllBytes(path, readable.EncodeToPNG());
            Object.Destroy(readable);
        }

        private static void writeTexture(string path, Texture2D tex)
        {
            var readable = new Texture2D(tex.width, tex.height, TextureFormat.RGBA32, false);
            RenderTexture current = RenderTexture.active;
            var rt = RenderTexture.GetTemporary(tex.width, tex.height);
            Graphics.Blit(tex, rt);
            RenderTexture.active = rt;
            readable.ReadPixels(new Rect(0, 0, rt.width, rt.height), 0, 0);
            readable.Apply();
            RenderTexture.active = current;
            RenderTexture.ReleaseTemporary(rt);
            tex = readable;
            File.WriteAllBytes(path, tex.EncodeToPNG());
            Object.Destroy(tex);
        }

        private static string ToHex(Color c)
        {
            return $"#{(byte)(c.r * 255):X2}{(byte)(c.g * 255):X2}{(byte)(c.b * 255):X2}";
        }
    }
}
