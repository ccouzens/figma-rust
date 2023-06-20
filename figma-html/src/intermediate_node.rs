use std::borrow::Cow;

use figma_schema::{
    AxisSizingMode, CounterAxisAlignItems, LayoutAlign, LayoutConstraint,
    LayoutConstraintHorizontal, LayoutConstraintVertical, LayoutMode, Node as FigmaNode,
    NodeType as FigmaNodeType, PrimaryAxisAlignItems, StrokeAlign, StrokeWeights, TextAutoResize,
    TextCase, TextDecoration, TypeStyle,
};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

mod html_formatter;
mod inset;
mod size;
pub use html_formatter::{format_css, HtmlFormatter};
pub use inset::Inset;
pub use size::Size;

use super::css_properties::{absolute_bounding_box, fills_color, stroke_color, CssProperties};

#[derive(Debug, Serialize, Deserialize)]
pub struct CSSVariable {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

pub type CSSVariablesMap<'a> = IndexMap<Cow<'a, str>, CSSVariable>;

#[derive(Debug, Serialize, Deserialize)]
pub enum AlignItems {
    Stretch,
    FlexStart,
    Center,
    FlexEnd,
    Baseline,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AlignSelf {
    Stretch,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum FlexDirection {
    Row,
    Column,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum JustifyContent {
    FlexStart,
    Center,
    FlexEnd,
    SpaceBetween,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StrokeStyle {
    Solid,
    Dashed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FlexContainer {
    pub align_items: AlignItems,
    pub direction: FlexDirection,
    pub gap: Size,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub justify_content: Option<JustifyContent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub padding: [Size; 4],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align_self: Option<AlignSelf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flex_grow: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inset: Option<[Inset; 4]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Size>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Size>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Appearance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
    pub preserve_whitespace: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_tranform: Option<TextCase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_decoration_line: Option<TextDecoration>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FrameAppearance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_radius: Option<[Size; 4]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub box_shadow: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke: Option<Stroke>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stroke {
    pub weights: [f64; 4],
    pub style: StrokeStyle,
    pub offset: StrokeAlign,
    pub color: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Figma<'a> {
    pub name: Cow<'a, str>,
    pub id: Cow<'a, str>,
    pub r#type: FigmaNodeType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IntermediateNodeType<'a> {
    Vector,
    Text { text: Cow<'a, str> },
    Frame { children: Vec<IntermediateNode<'a>> },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntermediateNode<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub figma: Option<Figma<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flex_container: Option<FlexContainer>,
    pub location: Location,
    pub appearance: Appearance,
    pub frame_appearance: FrameAppearance,
    pub node_type: IntermediateNodeType<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<Cow<'a, str>>,
}

impl<'a> IntermediateNode<'a> {
    pub fn from_figma_node(
        node: &'a FigmaNode,
        parent: Option<&'a FigmaNode>,
        css_variables: &mut CSSVariablesMap,
    ) -> Self {
        IntermediateNode {
            figma: Some(Figma {
                name: Cow::Borrowed(&node.name),
                id: Cow::Borrowed(&node.id),
                r#type: node.r#type,
            }),
            flex_container: {
                let align_items = match node.counter_axis_align_items {
                    None => AlignItems::Stretch,
                    Some(CounterAxisAlignItems::Min) => AlignItems::FlexStart,
                    Some(CounterAxisAlignItems::Center) => AlignItems::Center,
                    Some(CounterAxisAlignItems::Max) => AlignItems::FlexEnd,
                    Some(CounterAxisAlignItems::Baseline) => AlignItems::Baseline,
                };
                let gap = Size::Pixels(node.item_spacing.unwrap_or(0.0));
                let justify_content = match node.primary_axis_align_items {
                    None => None,
                    Some(PrimaryAxisAlignItems::Min) => Some(JustifyContent::FlexStart),
                    Some(PrimaryAxisAlignItems::Center) => Some(JustifyContent::Center),
                    Some(PrimaryAxisAlignItems::Max) => Some(JustifyContent::FlexEnd),
                    Some(PrimaryAxisAlignItems::SpaceBetween) => Some(JustifyContent::SpaceBetween),
                };
                match node.layout_mode {
                    Some(LayoutMode::Horizontal) => Some(FlexContainer {
                        align_items,
                        direction: FlexDirection::Row,
                        gap,
                        justify_content,
                    }),
                    Some(LayoutMode::Vertical) => Some(FlexContainer {
                        align_items,
                        direction: FlexDirection::Column,
                        gap,
                        justify_content,
                    }),
                    _ => None,
                }
            },
            location: Location {
                padding: [
                    Size::Pixels(node.padding_top.unwrap_or(0.0)),
                    Size::Pixels(node.padding_right.unwrap_or(0.0)),
                    Size::Pixels(node.padding_bottom.unwrap_or(0.0)),
                    Size::Pixels(node.padding_left.unwrap_or(0.0)),
                ],
                align_self: match (
                    parent.and_then(|p| p.layout_mode.as_ref()),
                    node.layout_align.as_ref(),
                ) {
                    (
                        Some(LayoutMode::Horizontal | LayoutMode::Vertical),
                        Some(LayoutAlign::Stretch),
                    ) => Some(AlignSelf::Stretch),
                    _ => None,
                },
                flex_grow: match (
                    parent.and_then(|p| p.layout_mode.as_ref()),
                    node.layout_grow,
                ) {
                    (Some(LayoutMode::Horizontal | LayoutMode::Vertical), Some(grow))
                        if grow != 0.0 =>
                    {
                        Some(grow)
                    }
                    _ => None,
                },
                inset: Inset::from_figma_node(node, parent),
                height: match (parent, node) {
                    (
                        Some(FigmaNode {
                            layout_mode: Some(LayoutMode::Horizontal),
                            ..
                        }),
                        FigmaNode {
                            layout_align: Some(LayoutAlign::Stretch),
                            ..
                        },
                    )
                    | (
                        _,
                        FigmaNode {
                            characters: Some(_),
                            ..
                        }
                        | FigmaNode {
                            constraints:
                                Some(LayoutConstraint {
                                    vertical: LayoutConstraintVertical::TopBottom,
                                    ..
                                }),
                            ..
                        },
                    ) => None,
                    (
                        Some(FigmaNode {
                            layout_mode: Some(LayoutMode::Vertical),
                            ..
                        }),
                        FigmaNode {
                            layout_grow: Some(layout_grow),
                            ..
                        },
                    ) if *layout_grow == 1.0 => None,
                    (
                        _,
                        FigmaNode {
                            layout_mode: Some(LayoutMode::Vertical),
                            primary_axis_sizing_mode,
                            ..
                        },
                    ) if primary_axis_sizing_mode != &Some(AxisSizingMode::Fixed) => None,
                    (
                        _,
                        FigmaNode {
                            layout_mode: Some(LayoutMode::Horizontal),
                            counter_axis_sizing_mode,
                            ..
                        },
                    ) if counter_axis_sizing_mode != &Some(AxisSizingMode::Fixed) => None,
                    _ => absolute_bounding_box(node)
                        .and_then(|b| b.height)
                        .map(Size::Pixels),
                },
                width: match (parent, node) {
                    (
                        Some(FigmaNode {
                            layout_mode: Some(LayoutMode::Vertical),
                            ..
                        }),
                        FigmaNode {
                            layout_align: Some(LayoutAlign::Stretch),
                            ..
                        },
                    )
                    | (
                        _,
                        FigmaNode {
                            style:
                                Some(TypeStyle {
                                    text_auto_resize: Some(TextAutoResize::WidthAndHeight),
                                    ..
                                }),
                            ..
                        }
                        | FigmaNode {
                            constraints:
                                Some(LayoutConstraint {
                                    horizontal: LayoutConstraintHorizontal::LeftRight,
                                    ..
                                }),
                            ..
                        },
                    ) => None,
                    (
                        Some(FigmaNode {
                            layout_mode: Some(LayoutMode::Horizontal),
                            ..
                        }),
                        FigmaNode {
                            layout_grow: Some(layout_grow),
                            ..
                        },
                    ) if *layout_grow == 1.0 => None,
                    (
                        _,
                        FigmaNode {
                            layout_mode: Some(LayoutMode::Horizontal),
                            primary_axis_sizing_mode,
                            ..
                        },
                    ) if primary_axis_sizing_mode != &Some(AxisSizingMode::Fixed) => None,
                    (
                        _,
                        FigmaNode {
                            layout_mode: Some(LayoutMode::Vertical),
                            counter_axis_sizing_mode,
                            ..
                        },
                    ) if counter_axis_sizing_mode != &Some(AxisSizingMode::Fixed) => None,
                    _ => absolute_bounding_box(node)
                        .and_then(|b| b.width)
                        .map(Size::Pixels),
                },
            },
            appearance: Appearance {
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
                opacity: node.opacity,
                text_decoration_line: node.style.as_ref().and_then(|s| s.text_decoration),
                text_tranform: node.style.as_ref().and_then(|s| s.text_case),
                preserve_whitespace: node
                    .characters
                    .as_deref()
                    .map(|text| {
                        text.contains('\n') || {
                            // detect if the first or last characters are whitespace, or if there is double whitespace
                            let mut last_char_was_whitespace = true;
                            for c in text.chars() {
                                if c.is_ascii_whitespace() {
                                    if last_char_was_whitespace {
                                        break;
                                    }
                                    last_char_was_whitespace = true
                                } else {
                                    last_char_was_whitespace = false
                                }
                            }
                            last_char_was_whitespace
                        }
                    })
                    .unwrap_or(false),
            },
            frame_appearance: FrameAppearance {
                background: node.background(css_variables),
                border_radius: node
                    .rectangle_corner_radii()
                    .map(|[top, right, bottom, left]| {
                        [
                            Size::Pixels(top),
                            Size::Pixels(right),
                            Size::Pixels(bottom),
                            Size::Pixels(left),
                        ]
                    }),
                box_shadow: node.box_shadow(),
                stroke: {
                    let style =
                        if node.stroke_dashes.as_ref().map(|sd| sd.is_empty()) == Some(false) {
                            StrokeStyle::Dashed
                        } else {
                            StrokeStyle::Solid
                        };
                    match (
                        stroke_color(node),
                        &node.individual_stroke_weights,
                        node.stroke_weight,
                        node.stroke_align,
                    ) {
                        (
                            Some(color),
                            Some(StrokeWeights {
                                top,
                                right,
                                bottom,
                                left,
                            }),
                            _,
                            Some(offset),
                        ) => Some(Stroke {
                            weights: [*top, *right, *bottom, *left],
                            style,
                            offset,
                            color,
                        }),
                        (Some(color), _, Some(w), Some(offset)) => Some(Stroke {
                            weights: [w, w, w, w],
                            style,
                            offset,
                            color,
                        }),
                        _ => None,
                    }
                },
            },
            node_type: match node.r#type {
                FigmaNodeType::Vector | FigmaNodeType::BooleanOperation => {
                    IntermediateNodeType::Vector
                }
                FigmaNodeType::Text => IntermediateNodeType::Text {
                    text: Cow::Borrowed(node.characters.as_deref().unwrap_or("")),
                },
                _ => IntermediateNodeType::Frame {
                    children: node
                        .enabled_children()
                        .map(|child| Self::from_figma_node(child, Some(node), css_variables))
                        .collect(),
                },
            },
            href: node
                .style
                .as_ref()
                .and_then(|s| s.hyperlink.as_ref())
                .and_then(|h| h.url.as_deref().or_else(|| h.node_id.as_ref().map(|_| "#")))
                .map(Cow::Borrowed),
        }
    }

    fn children(&self) -> Option<&[Self]> {
        match &self.node_type {
            IntermediateNodeType::Frame { children } => Some(children),
            _ => None,
        }
    }

    pub fn naive_css_string(&self) -> String {
        let properties = &[
            (
                "align-items",
                self.flex_container
                    .as_ref()
                    .and_then(|c| match c.align_items {
                        AlignItems::Stretch => None,
                        AlignItems::FlexStart => Some(Cow::Borrowed("flex-start")),
                        AlignItems::Center => Some(Cow::Borrowed("center")),
                        AlignItems::FlexEnd => Some(Cow::Borrowed("flex-end")),
                        AlignItems::Baseline => Some(Cow::Borrowed("baseline")),
                    }),
            ),
            (
                "align-self",
                match self.location.align_self {
                    Some(AlignSelf::Stretch) => Some(Cow::Borrowed("stretch")),
                    _ => None,
                },
            ),
            (
                "background",
                self.frame_appearance
                    .background
                    .as_deref()
                    .map(Cow::Borrowed),
            ),
            (
                "border-radius",
                self.frame_appearance
                    .border_radius
                    .as_ref()
                    .map(|[nw, ne, se, sw]| Cow::Owned(format!("{nw} {ne} {se} {sw}"))),
            ),
            (
                "box-shadow",
                self.frame_appearance
                    .box_shadow
                    .as_deref()
                    .map(Cow::Borrowed),
            ),
            ("box-sizing", {
                let Location {
                    width,
                    height,
                    padding: [top, right, bottom, left],
                    ..
                } = &self.location;
                if (top != &Size::Pixels(0.0) || bottom != &Size::Pixels(0.0)) && height.is_some()
                    || (right != &Size::Pixels(0.0) || left != &Size::Pixels(0.0))
                        && width.is_some()
                {
                    Some(Cow::Borrowed("border-box"))
                } else {
                    None
                }
            }),
            ("color", self.appearance.color.as_deref().map(Cow::Borrowed)),
            (
                "display",
                self.flex_container.as_ref().map(|_| Cow::Borrowed("flex")),
            ),
            (
                "flex-direction",
                self.flex_container.as_ref().map(|c| {
                    Cow::Borrowed(match c.direction {
                        FlexDirection::Row => "row",
                        FlexDirection::Column => "column",
                    })
                }),
            ),
            ("fill", self.appearance.fill.as_deref().map(Cow::Borrowed)),
            (
                "flex-grow",
                self.location.flex_grow.map(|g| Cow::Owned(format!("{g}"))),
            ),
            ("font", self.appearance.font.as_deref().map(Cow::Borrowed)),
            (
                "gap",
                self.flex_container.as_ref().and_then(|c| {
                    if c.gap == Size::Pixels(0.0) {
                        None
                    } else {
                        Some(Cow::Owned(format!("{}", c.gap)))
                    }
                }),
            ),
            (
                "height",
                self.location
                    .height
                    .as_ref()
                    .map(|h| Cow::Owned(format!("{h}"))),
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
                "justify-content",
                self.flex_container.as_ref().and_then(|c| {
                    c.justify_content.as_ref().map(|j| {
                        Cow::Borrowed(match j {
                            JustifyContent::FlexStart => "flex-start",
                            JustifyContent::Center => "center",
                            JustifyContent::FlexEnd => "flex-end",
                            JustifyContent::SpaceBetween => "space-between",
                        })
                    })
                }),
            ),
            (
                "opacity",
                self.appearance.opacity.map(|o| Cow::Owned(format!("{o}"))),
            ),
            (
                "outline",
                self.frame_appearance.stroke.as_ref().and_then(|s| {
                    // Let top border represent the weight of all the borders
                    let width = s.weights[0];
                    if width == 0.0 {
                        return None;
                    }
                    let style = match s.style {
                        StrokeStyle::Solid => "solid",
                        StrokeStyle::Dashed => "dashed",
                    };
                    let color = &s.color;
                    Some(Cow::Owned(format!("{width}px {style} {color}")))
                }),
            ),
            (
                "outline-offset",
                self.frame_appearance.stroke.as_ref().and_then(|s| {
                    // Let top border represent the weight of all the borders
                    let width = s.weights[0];
                    match s.offset {
                        StrokeAlign::Inside => Some(Cow::Owned(format!("-{width}px"))),
                        StrokeAlign::Outside => None,
                        StrokeAlign::Center => Some(Cow::Owned(format!("-{}px", width / 2.0))),
                    }
                }),
            ),
            ("padding", {
                let p = &self.location.padding;
                if p == &[
                    Size::Pixels(0.0),
                    Size::Pixels(0.0),
                    Size::Pixels(0.0),
                    Size::Pixels(0.0),
                ] {
                    None
                } else {
                    Some(Cow::Owned(format!("{} {} {} {}", p[0], p[1], p[2], p[3])))
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
            (
                "text-decoration-line",
                self.appearance.text_decoration_line.map(|t| {
                    Cow::Borrowed(match t {
                        TextDecoration::Strikethrough => "line-through",
                        TextDecoration::Underline => "underline",
                    })
                }),
            ),
            (
                "text-transform",
                self.appearance.text_tranform.and_then(|t| match t {
                    TextCase::Upper => Some(Cow::Borrowed("uppercase")),
                    TextCase::Lower => Some(Cow::Borrowed("lowercase")),
                    TextCase::Title => Some(Cow::Borrowed("capitalize")),
                    TextCase::SmallCaps => None,
                    TextCase::SmallCapsForced => None,
                }),
            ),
            (
                "white-space",
                self.appearance
                    .preserve_whitespace
                    .then_some(Cow::Borrowed("pre-wrap")),
            ),
            (
                "width",
                self.location
                    .width
                    .as_ref()
                    .map(|w| Cow::Owned(format!("{w}"))),
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
