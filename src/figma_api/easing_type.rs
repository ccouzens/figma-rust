use serde::{Deserialize, Serialize};

/// [Figma documentation](https://www.figma.com/developers/api#easingtype-type)
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EasingType {
    Linear,
    EaseIn,
    EaseOut,
    EaseInAndOut,
    EaseInBack,
    EaseOutBack,
    EaseInAndOutBack,
    CustomBezier,
    Gentle,
    Quick,
    Bouncy,
    Slow,
    CustomSpring,
}
