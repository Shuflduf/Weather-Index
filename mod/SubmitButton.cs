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

            LanguageTextMeshController label =
                submitButtonObj.GetComponentInChildren<LanguageTextMeshController>();
            if (label != null)
            {
                label.token = "Submit";
            }
            UnityEngine.UI.Image image =
                submitButtonObj.GetComponentInChildren<UnityEngine.UI.Image>();
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
            SubmitRunResult result = await WIBridge.SubmitRun();
            switch (result)
            {
                case SubmitRunResult.Success:
                    WIPopup.ShowMessage("Run submitted succesfully!");
                    break;
                case SubmitRunResult.AlreadyUploaded:
                    WIPopup.ShowMessage("Run already submitted!");
                    break;
                case SubmitRunResult.NotLoggedIn:
                    WIPopup.ShowMessage(
                        "Not signed in. Sign in from the settings page and re-submit!"
                    );
                    break;
                case SubmitRunResult.NetworkError:
                    WIPopup.ShowMessage("Could not reach the server. Please try again later.");
                    break;
                case SubmitRunResult.ServerError:
                    WIPopup.ShowMessage("Server error. Please try again later.");
                    break;
            }
        }
    }
}
