mod color;
mod component;
mod easing_type;
mod file;
mod node;
mod paint;
mod rectangle;
mod style;
pub use self::{
    color::Color,
    component::Component,
    easing_type::EasingType,
    file::File,
    node::{Node, NodeType, StrokeAlign},
    paint::Paint,
    rectangle::Rectangle,
    style::{Style, StyleType},
};

fn default_true() -> bool {
    true
}

fn default_one() -> f64 {
    1.0
}
