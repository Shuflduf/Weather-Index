#nullable enable

using System;
using System.Collections.Generic;
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

    internal class EquipmentEvent
    {
        public EquipmentIndex? id;
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
        internal static List<EquipmentEvent> equipments = new List<EquipmentEvent>();

        private static ItemList oldItems = new ItemList();
        private static EquipmentIndex oldEquip = EquipmentIndex.None;
        private static Dictionary<int, StageInteractable>? currentStage;
        private static string? currentStageName;

        public static void Reset()
        {
            stages = new List<StageInfo>();
            items = new List<ItemEvent>();
            equipments = new List<EquipmentEvent>();
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
                                si.time = timestamp();
                                currentStage[id] = si;
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
                CharacterMaster master = inv.GetComponent<CharacterMaster>();
                if (
                    master == null
                    || !RoR2.NetworkUser.localPlayers.Exists(nu => nu.master == master)
                )
                    return;

                ItemCollection stacks = inv.permanentItemStacks;
                Dictionary<ItemIndex, int> newItems = itemList(stacks);
                EquipmentIndex equip = inv.GetEquipmentIndex();
                if (equip != oldEquip)
                {
                    equipments.Add(new EquipmentEvent() { id = equip, time = timestamp() });
                    oldEquip = equip;
                }

                List<ItemEvent> diffs = itemDifference(oldItems, newItems);
                addItemEvents(diffs);

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
                if (items.Count > 0)
                {
                    ItemEvent last = items[items.Count - 1];
                    if (last.id == diff.id && last.time == diff.time)
                    {
                        int merged = (last.count ?? 0) + (diff.count ?? 0);
                        if (merged == 0)
                        {
                            items.RemoveAt(items.Count - 1);
                        }
                        else
                        {
                            items[items.Count - 1] = new ItemEvent()
                            {
                                id = last.id,
                                count = merged,
                                time = last.time,
                            };
                        }
                        continue;
                    }
                }
                items.Add(diff);
            }
        }

        static ItemList itemList(ItemCollection stacks)
        {
            Dictionary<ItemIndex, int> list = new Dictionary<ItemIndex, int>();
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
                            time = timestamp(),
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
                            time = timestamp(),
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
                            time = timestamp(),
                        }
                    );
                }
            }
            return diffs;
        }

        static int timestamp()
        {
            return (int)Math.Floor(Run.TimeStamp.tNow);
        }
    }
}
