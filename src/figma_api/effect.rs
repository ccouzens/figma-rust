use serde::{Deserialize, Serialize};

use super::{Color, Vector};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[typeshare::typeshare]
pub enum EffectType {
    InnerShadow,
    DropShadow,
    LayerBlur,
    BackgroundBlur,
}

/// A visual effect such as a shadow or blur
///
/// [Figma documentation](https://www.figma.com/developers/api#effect-type)
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[typeshare::typeshare]
pub struct Effect {
    /// Type of effect
    pub r#type: EffectType,
    /// Is the effect active?
    pub visible: bool,
    /// The color of the shadow
    pub color: Color,
    /// How far the shadow is projected in the x and y directions
    pub offset: Vector,
    /// How far the shadow spreads
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spread: Option<f64>,
}

impl Effect {
    pub fn spread(&self) -> f64 {
        self.spread.unwrap_or(0.0)
    }
}
