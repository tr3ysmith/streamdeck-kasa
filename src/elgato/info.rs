use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Debug, Serialize, Deserialize)]
pub struct ElgatoInfo {
    application: ElgatoApplicationInfo,
    plugin: ElgatoPluginInfo,
    #[serde(rename = "devicePixelRatio")]
    device_pixel_ratio: f64,
    colors: HashMap<String, String>,
    devices: Vec<ElgatoDeviceInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElgatoApplicationInfo {
    font: Option<String>,
    language: String,
    platform: ElgatoPlatform,
    version: String,
    #[serde(rename = "platformVersion")]
    platform_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ElgatoPlatform {
    #[serde(rename = "mac")]
    MacOS,
    #[serde(rename = "windows")]
    Windows
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElgatoPluginInfo {
    version: String,
    uuid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElgatoDeviceInfo {
    id: String,
    #[serde(rename = "type")]
    device_type: ElgatoDeviceType,
    size: ElgatoDeviceSize,
    name: String
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
pub enum ElgatoDeviceType {
    StreamDeck = 0,
    StreamDeckMini = 1,
    StreamDeckXL = 2,
    StreamDeckMobile = 3,
    CorsairGKeys = 4,
    StreamDeckPedal = 5,
    CorsairVoyager = 6,
    StreamDeckPlus = 7
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElgatoDeviceSize {
    columns: u64,
    rows: u64
}