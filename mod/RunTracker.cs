#nullable enable

using System;
using System.Collections.Generic;
using Newtonsoft.Json;
using RoR2;

namespace WeatherIndex
{
    using ItemList = Dictionary<ItemIndex, int>;

    internal class ItemEvent
    {
        public ItemIndex? id;
        public int? count;
        public int? time;
    }

    internal class StageInteractable
    {
        public string? name;
        public int? time;
        public int? item;
    }

    internal class StageInfo
    {
        public string name = "";
        public List<StageInteractable> interactables = new List<StageInteractable>();
    }

    class RunTracker
    {
        internal static List<StageInfo> stages = new List<StageInfo>();
        internal static List<ItemEvent> items = new List<ItemEvent>();

        private static ItemList oldItems = new ItemList();
        private static Dictionary<int, StageInteractable>? currentStage;
        private static string? currentStageName;

        public static void Reset()
        {
            stages = new List<StageInfo>();
            items = new List<ItemEvent>();
            oldItems = new ItemList();
        }

        public static void Init()
        {
            On.RoR2.Run.OnClientGameOver += (orig, self, runReport) =>
            {
                addCurrentStage();

                orig(self, runReport);
            };
            On.RoR2.Run.AdvanceStage += (orig, self, stage) =>
            {
                orig(self, stage);

                addCurrentStage();
            };
            On.RoR2.Run.OnStageStartGlobal += (orig, self, stage) =>
            {
                orig(self, stage);

                currentStage = new Dictionary<int, StageInteractable>();

                PurchaseInteraction[] interactions = InstanceTracker
                    .GetInstancesList<PurchaseInteraction>()
                    .ToArray();
                for (int i = 0; i < interactions.Length; i++)
                {
                    int id = i;
                    PurchaseInteraction interaction = interactions[i];

                    interaction.onDetailedPurchaseServer.AddListener(
                        (ctx, res) =>
                        {
                            if (currentStage.TryGetValue(id, out StageInteractable si))
                            {
                                int timestamp = (int)Math.Floor(Run.TimeStamp.tNow);
                                si.time = timestamp;
                                currentStage[id] = si;

                                Log.Info(JsonConvert.SerializeObject(currentStage));
                            }
                        }
                    );
                    if (interaction.TryGetComponent<ChestBehavior>(out ChestBehavior chest))
                    {
                        PickupDef pickup = PickupCatalog.GetPickupDef(
                            chest.currentPickup.pickupIndex
                        );

                        StageInteractable si = new StageInteractable();
                        si.name = interaction.displayNameToken;
                        si.item =
                            pickup.itemIndex != ItemIndex.None
                                ? (int)pickup.itemIndex
                                : (int)pickup.equipmentIndex;

                        currentStage.Add(id, si);
                    }
                }
                currentStageName = RoR2.SceneCatalog.GetSceneDefForCurrentScene().cachedName;
                // StageInfo info = new StageInfo();
                // info.name = stageName;
                // info.interactables = stages.Add(stageName);
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

        static void addCurrentStage()
        {
            StageInfo info = new StageInfo();
            info.name = currentStageName!;
            info.interactables = new List<StageInteractable>(currentStage!.Count);
            foreach (StageInteractable si in currentStage!.Values)
            {
                info.interactables.Add(si);
            }
            stages.Add(info);
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
                    items[items.Count - 1] = new ItemEvent()
                    {
                        id = last.id,
                        count = last.count + diff.count,
                        time = last.time,
                    };
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
                    diffs.Add(
                        new ItemEvent()
                        {
                            id = item,
                            count = -oldItems[item],
                            time = timestamp,
                        }
                    );
                }
                else if (!oldItems.ContainsKey(item) && newItems.ContainsKey(item))
                {
                    diffs.Add(
                        new ItemEvent()
                        {
                            id = item,
                            count = newItems[item],
                            time = timestamp,
                        }
                    );
                }
                else if (oldItems[item] == newItems[item])
                {
                    continue;
                }
                else
                {
                    diffs.Add(
                        new ItemEvent()
                        {
                            id = item,
                            count = newItems[item] - oldItems[item],
                            time = timestamp,
                        }
                    );
                }
            }
            return diffs;
        }
    }
}
