#nullable enable

using System.Collections.Generic;

namespace WeatherIndex
{
    struct ItemEvent
    {
        public int Id;
        public int Count;

        public ItemEvent(int id, int count)
        {
            Id = id;
            Count = count;
        }
    }

    class RunTracker
    {
        public static List<string> stages = new List<string>();
        public static List<ItemEvent> items = new List<ItemEvent>();

        public static void Init()
        {
            On.RoR2.Run.OnStageStartGlobal += (orig, self, stage) =>
            {
                orig(self, stage);

                string stageName = RoR2.SceneCatalog.GetSceneDefForCurrentScene().cachedName;
                stages.Add(stageName);
            };

            // On.RoR2.CharacterMaster.OnItemAddedClient += (orig, self, item) =>
            // {
            //     orig(self, item);

            //     foreach (var localUser in RoR2.NetworkUser.localPlayers)
            //     {
            //         if (localUser.master == self)
            //         {
            //             if (items[-1].Id == (int)item)
            //             {
            //                 items[-1] = new ItemEvent((int)item, items[-1].Count + 1);
            //             }
            //             else
            //             {
            //                 // items.Add
            //             }
            //             Log.Info(item);
            //             break;
            //         }
            //     }
            // };

            RoR2.Inventory.onInventoryChangedGlobal += (inv) =>
            {
                var master = inv.GetComponent<RoR2.CharacterMaster>();
                if (
                    master == null
                    || !RoR2.NetworkUser.localPlayers.Exists(nu => nu.master == master)
                )
                    return;

                Log.Info(inv);
                var stacks = inv.permanentItemStacks;
                Log.Info(stacks);
                Log.Info(stacks.GetTotalItemStacks());
                Log.Info(stacks.GetStackValue((RoR2.ItemIndex)206));
            };
        }
    }
}
