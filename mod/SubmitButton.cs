#nullable enable

using RoR2.UI;
using UnityEngine;

namespace WeatherIndex
{
    public class SubmitButton : MonoBehaviour
    {
        private GameEndReportPanelController? panel;

        public void Init(GameEndReportPanelController panelController)
        {
            panel = panelController;
            CreateButton();
        }

        private void CreateButton()
        {
            if (panel == null)
                return;

            MPButton continueButton = panel.continueButton;
            if (continueButton == null)
                return;

            GameObject submitButtonObj = Instantiate(
                continueButton.gameObject,
                continueButton.transform.parent
            );
            submitButtonObj.transform.SetAsFirstSibling();
            submitButtonObj.name = "WeatherIndexSubmitButton";

            HGButton btn = submitButtonObj.GetComponent<HGButton>();
            btn.onClick.RemoveAllListeners();
            btn.onClick.AddListener(OnSubmitClicked);
            // btn.interactable = !string.IsNullOrEmpty(WeatherIndex.accessToken?.Value);

            var label = submitButtonObj.GetComponentInChildren<LanguageTextMeshController>();
            if (label != null)
            {
                label.token = "Submit";
            }
            var image = submitButtonObj.GetComponentInChildren<UnityEngine.UI.Image>();
            if (image != null)
            {
                image.color = new Color(0.5f, 0.9f, 1.0f, 1.0f);
            }

            Transform glyph = submitButtonObj.transform.Find("GenericGlyph");
            if (glyph != null)
            {
                glyph.gameObject.SetActive(false);
            }
        }

        private async void OnSubmitClicked()
        {
            var result = await WeatherIndex.SubmitRun();
            switch (result)
            {
                case WeatherIndex.SubmitRunResult.Success:
                    WIPopup.ShowMessage("Run submitted succesfully!");
                    break;
                case WeatherIndex.SubmitRunResult.AlreadyUploaded:
                    WIPopup.ShowMessage("Run already submitted!");
                    break;
                case WeatherIndex.SubmitRunResult.NotLoggedIn:
                    WIPopup.ShowMessage(
                        "Not signed in. Sign in from the settings page and re-submit!"
                    );
                    break;
                case WeatherIndex.SubmitRunResult.NetworkError:
                    WIPopup.ShowMessage("Could not reach the server. Please try again later.");
                    break;
                case WeatherIndex.SubmitRunResult.ServerError:
                    WIPopup.ShowMessage("Server error. Please try again later.");
                    break;
            }
        }
    }
}
