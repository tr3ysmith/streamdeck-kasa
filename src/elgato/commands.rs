use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenUrlPayload {
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogMessagePayload {
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetTitlePayload {
    title: String,
    target: Target,
    #[serde(default)]
    state: Option<u64>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetImagePayload {
    image: String,
    target: Target,
    #[serde(default)]
    state: Option<u64>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Target {
    Software = 2,
    Hardware = 1,
    Both = 0
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ElgatoCommand {
    /// Save data persistently for the action's instance.
    SetSettings { context: Value, payload: Value },
    /// Request the persistent data for the action's instance.
    GetSettings { context: Value },
    /// Save data securely and globally for the plugin.
    SetGlobalSettings { context: Value, payload: Value },
    /// Request the global persistent data.
    GetGlobalSettings { context: Value },
    /// Open an URL in the default browser.
    OpenUrl { payload: OpenUrlPayload },
    /// Write a debug log to the logs file.
    LogMessage { payload: LogMessagePayload },
    /// Dynamically change the title of an instance of an action.
    SetTitle { context: Value, payload: SetTitlePayload },
    /// Dynamically change the image displayed by an instance of an action.
    SetImage { context: Value, payload: SetImagePayload },
    /// Dynamically change properties of items on the Stream Deck + touch display.
    SetFeedback,
    /// Dynamically change the current layout for the Stream Deck + touch display
    SetFeedbackLayout,
    /// Temporarily show an alert icon on they key or flash the touch display red (SD+) by an instance of an action.
    ShowAlert,
    /// Temporarily show an OK checkmark icon on the image displayed by an instance of an action.
    ShowOk,
    /// Change the state of the action's instance supporting multiple states.
    SetState,
    /// Switch to one of the pre-configured read-only profiles.
    SwitchToProfile,
    /// Send a payload to the Property Inspector.
    SendToPropertyInspector
}