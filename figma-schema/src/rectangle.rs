use serde::{Deserialize, Serialize};

/// [Figma documentation](https://www.figma.com/developers/api#rectangle-type)
#[derive(Debug, Deserialize, Serialize)]
#[typeshare::typeshare]
pub struct Rectangle {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
}
