#nullable enable

using RoR2.UI;
using UnityEngine;

namespace WeatherIndex
{
    public class WIPopup
    {
        public static void ShowMessage(string message)
        {
            WeatherIndex.MainThread(() =>
            {
                var dialog = SimpleDialogBox.Create();
                dialog.headerToken = new SimpleDialogBox.TokenParamsPair
                {
                    token = "Weather Index",
                    formatParams = System.Array.Empty<Object>(),
                };
                dialog.descriptionToken = new SimpleDialogBox.TokenParamsPair
                {
                    token = message,
                    formatParams = System.Array.Empty<Object>(),
                };
                dialog.AddCancelButton("Proceed");
            });
        }
    }
}
