use super::{BlendMode, Color, Vector};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[typeshare::typeshare]
pub enum PaintType {
    Solid,
    GradientLinear,
    GradientRadial,
    GradientAngular,
    GradientDiamond,
    Image,
}

/// A solid color, gradient, or image texture that can be applied as fills or strokes
///
/// [Figma documentation](https://www.figma.com/developers/api#paint-type)
#[derive(Debug, Deserialize, Serialize)]
#[typeshare::typeshare]
pub struct Paint {
    pub r#type: PaintType,
    /// Is the paint enabled?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    /// Overall opacity of paint (colors within the paint can also have opacity values which would blend with this)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
    /// Solid color of the paint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    /// How this node blends with nodes behind it in the scene
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blend_mode: Option<BlendMode>,
    /// This field contains three vectors, each of which are a position in normalized object space (normalized object space is if the top left corner of the bounding box of the object is (0, 0) and the bottom right is (1,1)). The first position corresponds to the start of the gradient (value 0 for the purposes of calculating gradient stops), the second position is the end of the gradient (value 1), and the third handle position determines the width of the gradient. See image examples below:
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gradient_handle_positions: Option<[Vector; 3]>,
}

impl Paint {
    pub fn visible(&self) -> bool {
        self.visible.unwrap_or(true)
    }

    pub fn opacity(&self) -> f64 {
        self.opacity.unwrap_or(1.0)
    }

    pub fn color(&self) -> Option<&Color> {
        self.color.as_ref()
    }
}
