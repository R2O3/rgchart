use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct EditorLayer {
    #[serde(rename = "Name")]
    pub name: String,
    
    #[serde(rename = "ColorRgb")]
    pub color_rgb: RgbColor,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RgbColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}