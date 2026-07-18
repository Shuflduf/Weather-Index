#nullable enable

using System.Collections.Generic;
using Newtonsoft.Json;
using RoR2;

namespace WeatherIndex
{
    using ItemList = Dictionary<ItemIndex, int>;

    struct ItemEvent
    {
        public ItemIndex id;
        public int count;

        public ItemEvent(ItemIndex Id, int Count)
        {
            id = Id;
            count = Count;
        }
    }

    class RunTracker
    {
        public static List<string> stages = new List<string>();
        public static List<ItemEvent> items = new List<ItemEvent>();
        static ItemList oldItems = new ItemList();

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

                var stacks = inv.permanentItemStacks;
                var newItems = itemList(stacks);

                ItemEvent? diff = itemDifference(oldItems, newItems);
                if (diff is not null)
                {
                    items.Add(diff.Value);
                }
                Log.Info(JsonConvert.SerializeObject(items));

                oldItems = newItems;
            };
        }

        static ItemList itemList(ItemCollection stacks)
        {
            var list = new Dictionary<ItemIndex, int>();
            for (int i = 0; i < ItemCatalog.itemCount; i++)
            {
                ItemIndex item = (ItemIndex)i;
                int count = stacks.GetStackValue(item);
                if (count == 0)
                    continue;

                list[item] = count;
            }
            return list;
        }

        static ItemEvent? itemDifference(ItemList oldItems, ItemList newItems)
        {
            for (int i = 0; i < ItemCatalog.itemCount; i++)
            {
                ItemIndex item = (ItemIndex)i;
                if (!oldItems.ContainsKey(item) && !newItems.ContainsKey(item))
                {
                    continue;
                }
                else if (oldItems.ContainsKey(item) && !newItems.ContainsKey(item))
                {
                    return new ItemEvent(item, -oldItems[item]);
                }
                else if (!oldItems.ContainsKey(item) && newItems.ContainsKey(item))
                {
                    return new ItemEvent(item, newItems[item]);
                }
                else if (oldItems[item] == newItems[item])
                {
                    continue;
                }
                else
                {
                    return new ItemEvent(item, newItems[item] - oldItems[item]);
                }
            }
            return null;
        }
    }
}
