#nullable enable

using System.Collections.Generic;
using System.IO;
using BepInEx;
using Newtonsoft.Json;
using UnityEngine;

namespace WeatherIndex
{
    class DataDumper
    {
        private static string pluginDir = Path.Combine(Paths.PluginPath, "WeatherIndex");

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

                items.Add(
                    new
                    {
                        id = (int)idx,
                        name = def.name,
                        nameToken = def.nameToken,
                        displayName = RoR2.Language.GetString(def.nameToken),
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

                bodies.Add(
                    new
                    {
                        name = body.name,
                        nameToken = body.baseNameToken,
                        displayName = RoR2.Language.GetString(body.baseNameToken),
                        icon = filename,
                    }
                );
            }
            var json = JsonConvert.SerializeObject(bodies);
            File.WriteAllText(Path.Combine(outputDir, "bodies.json"), json);
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
    }
}
