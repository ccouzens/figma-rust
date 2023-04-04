use serde::{Deserialize, Serialize};

/// Animation easing curves
///
/// [Figma documentation](https://www.figma.com/developers/api#easingtype-type)
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[typeshare::typeshare]
pub enum EasingType {
    /// Ease in with an animation curve similar to CSS ease-in
    EaseIn,
    /// Ease out with an animation curve similar to CSS ease-out
    EaseOut,
    /// Ease in and then out with an animation curve similar to CSS ease-in-out
    EaseInAndOut,
    /// No easing, similar to CSS linear
    Linear,
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
