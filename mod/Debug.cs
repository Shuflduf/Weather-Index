#nullable enable

using RoR2;

namespace WeatherIndex
{
    class Debug
    {
        public static bool enabled = false;

        public static void Init()
        {
            On.EntityStates.GameOver.RoR2MainEndingPlayCutscene.FixedUpdate += (orig, self) =>
            {
                orig(self);
                if (enabled)
                {
                    self.outer.SetNextStateToMain();
                }
            };

            On.EntityStates.GameOver.ShowCredits.OnEnter += (orig, self) =>
            {
                orig(self);
                if (enabled)
                {
                    self.outer.SetNextState(new EntityStates.GameOver.ShowReport());
                }
            };
        }

        internal static void SkipKeybind()
        {
            if (Run.instance && WIConfig.endRunKeybind!.Value.IsDown() && enabled)
            {
                Run.instance.BeginGameOver(RoR2Content.GameEndings.MainEnding);
            }
        }
    }
}
