use super::{default_one, default_true, Color};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaintTypeGradient {}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "type")]
pub enum PaintType {
    #[serde(rename_all = "camelCase")]
    Solid {
        color: Color,
    },
    GradientLinear {
        #[serde(flatten)]
        base: PaintTypeGradient,
    },
    GradientRadial {
        #[serde(flatten)]
        base: PaintTypeGradient,
    },
    GradientAngular {
        #[serde(flatten)]
        base: PaintTypeGradient,
    },
    GradientDiamond {
        #[serde(flatten)]
        base: PaintTypeGradient,
    },
    Image,
}

/// [Figma documentation](https://www.figma.com/developers/api#paint-type)
#[derive(Debug, Deserialize, Serialize)]
pub struct Paint {
    #[serde(default = "default_true")]
    pub visible: bool,
    #[serde(default = "default_one")]
    pub opacity: f64,
    #[serde(flatten)]
    pub paint_type: PaintType,
}

impl Paint {
    pub fn color(&self) -> Option<&Color> {
        match self.paint_type {
            PaintType::Solid { ref color } => Some(color),
            _ => None,
        }
    }
}
