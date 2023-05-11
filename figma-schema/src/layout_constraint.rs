use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[typeshare::typeshare]
pub enum LayoutConstraintVertical {
    /// Node is laid out relative to top of the containing frame
    Top,
    /// Node is laid out relative to bottom of the containing frame
    Bottom,
    /// Node is vertically centered relative to containing frame
    Center,
    /// Both top and bottom of node are constrained relative to containing frame (node stretches with frame)
    TopBottom,
    /// Node scales vertically with containing frame
    Scale,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[typeshare::typeshare]
pub enum LayoutConstraintHorizontal {
    /// Node is laid out relative to left of the containing frame
    Left,
    /// Node is laid out relative to right of the containing frame
    Right,
    /// Node is horizontally centered relative to containing frame
    Center,
    /// Both left and right of node are constrained relative to containing frame (node stretches with frame)
    LeftRight,
    /// Node scales horizontally with containing frame
    Scale,
}

/// Layout constraint relative to containing Frame
#[derive(Debug, Deserialize, Serialize)]
#[typeshare::typeshare]
pub struct LayoutConstraint {
    pub vertical: LayoutConstraintVertical,
    pub horizontal: LayoutConstraintHorizontal,
}
