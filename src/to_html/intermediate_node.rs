use std::{borrow::Cow, fmt};

use figma_schema::{
    LayoutConstraintHorizontal, LayoutConstraintVertical, LayoutMode, LayoutPositioning,
    Node as FigmaNode, NodeType as FigmaNodeType, StrokeAlign,
};
use indexmap::IndexMap;
use serde::Serialize;

mod html_formatter;
pub use html_formatter::{format_css, HtmlFormatter};

use super::css_properties::{absolute_bounding_box, fills_color, CssProperties};

pub struct CSSVariable {
    pub name: String,
    pub value: Option<String>,
}

pub type CSSVariablesMap<'a> = IndexMap<&'a str, CSSVariable>;

#[derive(Debug, Serialize)]
pub enum AlignItems {
    FlexStart,
    Center,
    FlexEnd,
    Baseline,
}

#[derive(Debug, Serialize)]
pub enum AlignSelf {
    Stretch,
}

#[derive(Debug, Serialize)]
pub enum FlexDirection {
    Row,
    Column,
}

#[derive(Debug, Serialize)]
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
        css_variables: &mut CSSVariablesMap,
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

#[derive(Debug, Serialize)]
pub enum JustifyContent {
    FlexStart,
    Center,
    FlexEnd,
    SpaceBetween,
}

#[derive(Debug, Serialize)]
pub enum StrokeStyle {
    Solid,
    Dashed,
}

#[derive(Debug, Serialize)]
pub struct FlexContainer {
    pub align_items: AlignItems,
    pub direction: FlexDirection,
    pub gap: f64,
    pub justify_content: Option<JustifyContent>,
}

#[derive(Debug, Serialize)]
pub struct Location {
    pub padding: [f64; 4],
    pub align_self: Option<AlignSelf>,
    pub flex_grow: Option<f64>,
    pub inset: Option<[Inset; 4]>,
    pub height: Option<f64>,
    pub width: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct Appearance {
    pub opacity: Option<f64>,
}

#[derive(Debug, Serialize)]
pub struct FrameAppearance {
    pub background: Option<String>,
    pub border_radius: Option<[f64; 4]>,
    pub box_shadow: Option<String>,
    pub stroke: Option<Stroke>,
}

#[derive(Debug, Serialize)]
pub struct Stroke {
    pub weights: [f64; 4],
    pub style: StrokeStyle,
    pub offset: StrokeAlign,
}

#[derive(Debug, Serialize)]
pub struct ContentAppearance {
    pub color: Option<String>,
    pub fill: Option<String>,
    pub font: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Figma<'a> {
    pub name: &'a str,
    pub id: &'a str,
    pub r#type: FigmaNodeType,
}

#[derive(Debug, Serialize)]
pub enum IntermediateNodeType<'a> {
    Vector,
    Text { text: &'a str },
    Frame { children: Vec<IntermediateNode<'a>> },
}

#[derive(Debug, Serialize)]
pub struct IntermediateNode<'a> {
    pub figma: Figma<'a>,
    pub flex_container: Option<FlexContainer>,
    pub location: Location,
    pub appearance: Appearance,
    pub frame_appearance: FrameAppearance,
    pub content_appearance: ContentAppearance,
    pub node_type: IntermediateNodeType<'a>,
}

impl<'a> IntermediateNode<'a> {
    pub fn from_figma_node(
        node: &'a FigmaNode,
        parent: Option<&'a FigmaNode>,
        css_variables: &mut CSSVariablesMap,
    ) -> Self {
        IntermediateNode {
            figma: Figma {
                name: &node.name,
                id: &node.id,
                r#type: node.r#type,
            },
            flex_container: None,
            location: Location {
                padding: [
                    node.padding_top.unwrap_or(0.0),
                    node.padding_right.unwrap_or(0.0),
                    node.padding_bottom.unwrap_or(0.0),
                    node.padding_left.unwrap_or(0.0),
                ],
                align_self: None,
                flex_grow: None,
                inset: Inset::from_figma_node(node, parent, css_variables),
                height: None,
                width: None,
            },
            appearance: Appearance {
                opacity: node.opacity,
            },
            frame_appearance: FrameAppearance {
                background: node.background(css_variables),
                border_radius: None,
                box_shadow: None,
                stroke: None,
            },
            content_appearance: ContentAppearance {
                color: match node.r#type {
                    FigmaNodeType::Text => fills_color(node, css_variables),
                    _ => None,
                },
                fill: match node.r#type {
                    FigmaNodeType::Vector | FigmaNodeType::BooleanOperation => {
                        fills_color(node, css_variables)
                    }
                    _ => None,
                },
                font: node.font(css_variables),
            },
            node_type: match node.r#type {
                FigmaNodeType::Vector | FigmaNodeType::BooleanOperation => {
                    IntermediateNodeType::Vector
                }
                FigmaNodeType::Text => IntermediateNodeType::Text {
                    text: node.characters.as_deref().unwrap_or(""),
                },
                _ => IntermediateNodeType::Frame {
                    children: node
                        .enabled_children()
                        .map(|child| Self::from_figma_node(child, Some(node), css_variables))
                        .collect(),
                },
            },
        }
    }

    fn children(&self) -> Option<&[Self]> {
        match &self.node_type {
            IntermediateNodeType::Frame { children } => Some(&children),
            _ => None,
        }
    }

    pub fn naive_css_string(&self) -> String {
        let properties = &[
            (
                "background",
                self.frame_appearance
                    .background
                    .as_deref()
                    .map(Cow::Borrowed),
            ),
            (
                "color",
                self.content_appearance.color.as_deref().map(Cow::Borrowed),
            ),
            (
                "fill",
                self.content_appearance.fill.as_deref().map(Cow::Borrowed),
            ),
            (
                "font",
                self.content_appearance.font.as_deref().map(Cow::Borrowed),
            ),
            (
                "inset",
                self.location
                    .inset
                    .as_ref()
                    .map(|[top, right, bottom, left]| {
                        Cow::Owned(format!("{top} {right} {bottom} {left}"))
                    }),
            ),
            (
                "opacity",
                self.appearance.opacity.map(|o| Cow::Owned(format!("{o}"))),
            ),
            ("padding", {
                let p = self.location.padding;
                if p == [0.0, 0.0, 0.0, 0.0] {
                    None
                } else {
                    Some(Cow::Owned(format!(
                        "{}px {}px {}px {}px",
                        p[0], p[1], p[2], p[3]
                    )))
                }
            }),
            (
                "position",
                if self.location.inset.is_some() {
                    Some(Cow::Borrowed("absolute"))
                } else if self.children().is_some_and(|children| {
                    children.iter().any(|child| child.location.inset.is_some())
                }) {
                    Some(Cow::Borrowed("relative"))
                } else {
                    None
                },
            ),
        ];
        let mut output = String::new();
        for (name, value) in properties.iter() {
            if let Some(v) = value {
                output.push_str(name);
                output.push_str(": ");
                output.push_str(v);
                output.push(';');
            }
        }
        output
    }
}
