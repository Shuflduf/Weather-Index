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
            btn.interactable = !string.IsNullOrEmpty(WeatherIndex.accessToken?.Value);

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

        private void OnSubmitClicked()
        {
            WeatherIndex.PostRunReport();
            // TODO: reflect error messages and stuff
            ShowPopup("Run submitted succesfully!");
        }

        private void ShowPopup(string message)
        {
            if (panel == null)
                return;

            var eventSystem = panel.GetComponent<MPEventSystemProvider>()?.eventSystem;
            if (eventSystem == null)
                return;

            var dialog = SimpleDialogBox.Create(eventSystem);
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
            dialog.AddCancelButton("Close");
        }
    }
}
