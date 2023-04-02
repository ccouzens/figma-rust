use serde::{Deserialize, Serialize};

/// [Figma documentation](https://www.figma.com/developers/api#color-type)
#[derive(Debug, Deserialize, Serialize)]
pub struct Color {
    #[serde(rename = "r")]
    red: f64,
    #[serde(rename = "g")]
    green: f64,
    #[serde(rename = "b")]
    blue: f64,
    #[serde(rename = "a")]
    alpha: f64,
}

impl Color {
    pub fn to_rgb_string(&self) -> String {
        format!(
            "rgb({}, {}, {}, {})",
            (self.red * 255.0).floor(),
            (self.green * 255.0).floor(),
            (self.blue * 255.0).floor(),
            self.alpha
        )
    }
}
