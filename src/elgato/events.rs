use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElgatorEvent {
    action: String,
    event: String,
    context: Value,
    device: String,
    payload: ElgatoEventPayload
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ElgatoEventPayload {
    /// When the user presses a key, the plugin will receive the keyDown event.
    KeyDown(ElgatoKeyPayload),
    /// When the user releases a key, the plugin will receive the keyUp event.
    KeyUp(ElgatoKeyPayload),
    /// When the user touches the display, the plugin will receive the touchTap event (SD+).
    TouchTap,
    /// When the user presses or releases the encoder, the plugin will receive the dialPress event (SD+).
    DialPress,
    /// When the user rotates the encoder, the plugin will receive the dialRotate event (SD+).
    DialRotate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElgatoCoordinates {
    column: u64,
    row: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElgatoKeyPayload {
    settings: Value,
    coordinates: ElgatoCoordinates,
    state: Option<u64>,
    user_desired_state: Option<u64>,
    is_in_multi_action: bool
}