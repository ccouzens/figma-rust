use std::fmt;

use figma_schema::{
    LayoutConstraintHorizontal, LayoutConstraintVertical, LayoutMode, LayoutPositioning,
    Node as FigmaNode,
};
use serde::{Serialize, Deserialize};

use crate::css_properties::absolute_bounding_box;

#[derive(Debug, Serialize, Deserialize)]
pub enum Inset {
    Auto,
    /// To be used like so: calc(100% * dy / dx + c px)
    Linear {
        dy: f64,
        dx: f64,
        c: f64,
    },
}

impl Inset {
    pub fn from_figma_node<'a>(
        node: &'a FigmaNode,
        parent: Option<&'a FigmaNode>,
    ) -> Option<[Inset; 4]> {
        let parent = parent?;
        if matches!(
            parent.layout_mode,
            Some(LayoutMode::Horizontal) | Some(LayoutMode::Vertical)
        ) && !matches!(node.layout_positioning, Some(LayoutPositioning::Absolute))
        {
            return None;
        }
        let parent_rectangle = absolute_bounding_box(parent)?;
        let node_rectangle = absolute_bounding_box(node)?;
        let top_distance = node_rectangle.y? - parent_rectangle.y?;
        let right_distance = parent_rectangle.x? + parent_rectangle.width?
            - node_rectangle.x?
            - node_rectangle.width?;
        let bottom_distance = parent_rectangle.y? + parent_rectangle.height?
            - node_rectangle.y?
            - node_rectangle.height?;
        let left_distance = node_rectangle.x? - parent_rectangle.x?;
        let node_constraints = node.constraints.as_ref()?;
        Some([
            match node_constraints.vertical {
                LayoutConstraintVertical::Top | LayoutConstraintVertical::TopBottom => {
                    Self::Linear {
                        dy: 0.0,
                        dx: 1.0,
                        c: top_distance,
                    }
                }
                LayoutConstraintVertical::Bottom => Self::Auto,
                LayoutConstraintVertical::Center => Self::Linear {
                    dy: 1.0,
                    dx: 2.0,
                    c: top_distance - parent_rectangle.height? / 2.0,
                },
                LayoutConstraintVertical::Scale => Self::Linear {
                    dy: top_distance,
                    dx: parent_rectangle.height?,
                    c: 0.0,
                },
            },
            match node_constraints.horizontal {
                LayoutConstraintHorizontal::Left => Self::Auto,
                LayoutConstraintHorizontal::Right | LayoutConstraintHorizontal::LeftRight => {
                    Self::Linear {
                        dy: 0.0,
                        dx: 1.0,
                        c: -right_distance,
                    }
                }
                LayoutConstraintHorizontal::Center => Self::Auto,
                LayoutConstraintHorizontal::Scale => Self::Linear {
                    dy: right_distance,
                    dx: parent_rectangle.width?,
                    c: 0.0,
                },
            },
            match node_constraints.vertical {
                LayoutConstraintVertical::Top => Self::Auto,
                LayoutConstraintVertical::Bottom | LayoutConstraintVertical::TopBottom => {
                    Self::Linear {
                        dy: 0.0,
                        dx: 1.0,
                        c: -bottom_distance,
                    }
                }
                LayoutConstraintVertical::Center => Self::Auto,
                LayoutConstraintVertical::Scale => Self::Linear {
                    dy: bottom_distance,
                    dx: parent_rectangle.height?,
                    c: 0.0,
                },
            },
            match node_constraints.horizontal {
                LayoutConstraintHorizontal::Left | LayoutConstraintHorizontal::LeftRight => {
                    Self::Linear {
                        dy: 0.0,
                        dx: 1.0,
                        c: left_distance,
                    }
                }
                LayoutConstraintHorizontal::Right => Self::Auto,
                LayoutConstraintHorizontal::Center => Self::Linear {
                    dy: 1.0,
                    dx: 2.0,
                    c: left_distance - parent_rectangle.width? / 2.0,
                },
                LayoutConstraintHorizontal::Scale => Self::Linear {
                    dy: left_distance,
                    dx: parent_rectangle.width?,
                    c: 0.0,
                },
            },
        ])
    }
}

impl fmt::Display for Inset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Inset::Auto => write!(f, "auto"),
            Inset::Linear { dy, dx, c } => write!(f, "calc(100% * {dy} / {dx} + {c}px)"),
        }
    }
}
