use serde::{Deserialize, Serialize};

mod color;
mod component;
mod file;
mod node;
mod paint;
mod rectangle;
mod style;
pub use self::{
    color::Color,
    component::Component,
    file::File,
    node::{Node, NodeType, StrokeAlign},
    paint::Paint,
    rectangle::Rectangle,
    style::{Style, StyleType},
};

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

fn default_true() -> bool {
    true
}

fn default_one() -> f64 {
    1.0
}
