#nullable enable

using System;
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
        public int time;

        public ItemEvent(ItemIndex Id, int Count, int Time)
        {
            id = Id;
            count = Count;
            time = Time;
        }
    }

    class RunTracker
    {
        public static List<string> stages = new List<string>();
        public static List<ItemEvent> items = new List<ItemEvent>();
        static ItemList oldItems = new ItemList();

        public static void Reset()
        {
            stages = new List<string>();
            items = new List<ItemEvent>();
            oldItems = new ItemList();
        }

        public static void Init()
        {
            On.RoR2.Run.OnStageStartGlobal += (orig, self, stage) =>
            {
                orig(self, stage);

                string stageName = RoR2.SceneCatalog.GetSceneDefForCurrentScene().cachedName;
                stages.Add(stageName);
            };

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

                List<ItemEvent> diffs = itemDifference(oldItems, newItems);
                addItemEvents(diffs);
                Log.Info(JsonConvert.SerializeObject(items));

                oldItems = newItems;
            };
        }

        static void addItemEvents(List<ItemEvent> diffs)
        {
            if (items.Count == 0)
            {
                items.AddRange(diffs);
                return;
            }

            foreach (ItemEvent diff in diffs)
            {
                ItemEvent last = items[items.Count - 1];
                if (last.id == diff.id)
                {
                    items[items.Count - 1] = new ItemEvent(
                        last.id,
                        last.count + diff.count,
                        last.time
                    );
                }
                else
                {
                    items.Add(diff);
                }
            }
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

        static List<ItemEvent> itemDifference(ItemList oldItems, ItemList newItems)
        {
            int timestamp = (int)Math.Floor(Run.TimeStamp.tNow);
            List<ItemEvent> diffs = new List<ItemEvent>();
            for (int i = 0; i < ItemCatalog.itemCount; i++)
            {
                ItemIndex item = (ItemIndex)i;
                if (!oldItems.ContainsKey(item) && !newItems.ContainsKey(item))
                {
                    continue;
                }
                else if (oldItems.ContainsKey(item) && !newItems.ContainsKey(item))
                {
                    diffs.Add(new ItemEvent(item, -oldItems[item], timestamp));
                }
                else if (!oldItems.ContainsKey(item) && newItems.ContainsKey(item))
                {
                    diffs.Add(new ItemEvent(item, newItems[item], timestamp));
                }
                else if (oldItems[item] == newItems[item])
                {
                    continue;
                }
                else
                {
                    diffs.Add(new ItemEvent(item, newItems[item] - oldItems[item], timestamp));
                }
            }
            return diffs;
        }
    }
}
