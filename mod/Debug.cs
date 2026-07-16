#nullable enable

namespace WeatherIndex
{
    class Debug
    {
        public static bool enabled = true;

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
    }
}
