#nullable enable

using System.Collections.Generic;

namespace WeatherIndex
{
    class StageTracker
    {
        public static List<string> stages = new List<string>();

        public static void Init()
        {
            On.RoR2.Run.OnStageStartGlobal += (orig, self, stage) =>
            {
                orig(self, stage);

                string stageName = RoR2.SceneCatalog.GetSceneDefForCurrentScene().cachedName;
                stages.Add(stageName);
            };
        }
    }
}
