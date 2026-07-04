#nullable enable

using System.Collections.Generic;
using System.IO;
using BepInEx;
using Newtonsoft.Json;
using UnityEngine;

namespace WeatherIndex
{
    class ItemDumper
    {
        public static void Dump()
        {
            var outputDir = Path.Combine(Paths.PluginPath, "WeatherIndex", "items");
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
                    var readable = new Texture2D(
                        tex.width,
                        tex.height,
                        TextureFormat.RGBA32,
                        false
                    );
                    RenderTexture current = RenderTexture.active;
                    var rt = RenderTexture.GetTemporary(tex.width, tex.height);
                    Graphics.Blit(tex, rt);
                    RenderTexture.active = rt;
                    readable.ReadPixels(new Rect(0, 0, rt.width, rt.height), 0, 0);
                    readable.Apply();
                    RenderTexture.active = current;
                    RenderTexture.ReleaseTemporary(rt);
                    tex = readable;
                    File.WriteAllBytes(Path.Combine(outputDir, filename), tex.EncodeToPNG());
                    Object.Destroy(tex);
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
    }
}
