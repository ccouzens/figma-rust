use serde::{Deserialize, Serialize};

/// [Figma documentation](https://www.figma.com/developers/api#color-type)
#[derive(Debug, Deserialize, Serialize, Clone)]
#[typeshare::typeshare]
pub struct Color {
    #[serde(rename = "r")]
    pub red: f64,
    #[serde(rename = "g")]
    pub green: f64,
    #[serde(rename = "b")]
    pub blue: f64,
    #[serde(rename = "a")]
    pub alpha: f64,
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

    pub fn to_option_rgb_string(&self) -> Option<String> {
        if self.alpha == 0.0 {
            None
        } else {
            Some(self.to_rgb_string())
        }
    }
}
