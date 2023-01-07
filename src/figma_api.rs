use std::collections::HashMap;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Color {
    #[serde(rename = "r")]
    red: f64,
    #[serde(rename = "g")]
    green: f64,
    #[serde(rename = "b")]
    blue: f64,
    #[serde(rename = "a")]
    alpha: f64,
}

impl Color {
    pub fn to_rgba_string(&self) -> String {
        format!(
            "rgba({}, {}, {}, {})",
            (self.red * 255.0).floor(),
            (self.green * 255.0).floor(),
            (self.blue * 255.0).floor(),
            self.alpha
        )
    }
}

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

#[derive(Debug, Deserialize, Serialize)]
pub struct Component {
    pub key: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StyleType {
    Fill,
    Text,
    Effect,
    Grid,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum StyleTypeMapKey {
    Fills,
    Grid,
    Effect,
    Strokes,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Style {
    pub key: String,
    pub name: String,
    pub description: String,
    pub remote: bool,
    pub style_type: StyleType,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Rectangle {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
}

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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StrokeAlign {
    Inside,
    Outside,
    Center,
}

fn default_true() -> bool {
    true
}

fn default_one() -> f64 {
    1.0
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: String,
    pub name: String,
    #[serde(default = "default_true")]
    pub visible: bool,
    #[serde(flatten)]
    pub node_type: NodeType,
}

impl Node {
    pub fn absolute_bounding_box(&self) -> Option<&Rectangle> {
        self.node_type.absolute_bounding_box()
    }

    pub fn corner_radius(&self) -> Option<f64> {
        self.node_type.corner_radius()
    }

    pub fn rectangle_corner_radii(&self) -> Option<[f64; 4]> {
        self.node_type.rectangle_corner_radii()
    }

    pub fn opacity(&self) -> Option<f64> {
        self.node_type.opacity()
    }

    pub fn frame_props(&self) -> Option<&NodeTypeFrame> {
        self.node_type.frame_props()
    }

    pub fn children(&self) -> &[Node] {
        self.node_type.children()
    }

    pub fn depth_first_stack_iter(&self) -> NodeDepthFirstStackIterator {
        NodeDepthFirstStackIterator {
            stack: vec![self],
            iter_stack: vec![self.children().iter()],
        }
    }

    pub fn component<'a>(&self, file: &'a File) -> Option<&'a Component> {
        file.components.get(&self.id)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeTypeFrame {
    pub children: Vec<Node>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fills: Vec<Paint>,
    pub strokes: Vec<Paint>,
    pub stroke_weight: f64,
    pub stroke_align: StrokeAlign,
    #[serde(skip_serializing_if = "Option::is_none")]
    corner_radius: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    rectangle_corner_radii: Option<[f64; 4]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_easing: Option<EasingType>,
    #[serde(default = "default_one")]
    pub opacity: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_bounding_box: Option<Rectangle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_render_bounds: Option<Rectangle>,
    #[serde(default)]
    pub padding_left: f64,
    #[serde(default)]
    pub padding_right: f64,
    #[serde(default)]
    pub padding_top: f64,
    #[serde(default)]
    pub padding_bottom: f64,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub styles: HashMap<StyleTypeMapKey, String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeTypeVector {
    #[serde(default = "default_one")]
    pub opacity: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_bounding_box: Option<Rectangle>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fills: Vec<Paint>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "type")]
pub enum NodeType {
    #[serde(rename_all = "camelCase")]
    Document { children: Vec<Node> },
    #[serde(rename_all = "camelCase")]
    Canvas {
        background_color: Color,
        children: Vec<Node>,
    },
    #[serde(rename_all = "camelCase")]
    Frame {
        #[serde(flatten)]
        base: NodeTypeFrame,
    },
    #[serde(rename_all = "camelCase")]
    Group {
        #[serde(flatten)]
        base: NodeTypeFrame,
    },
    #[serde(rename_all = "camelCase")]
    Vector {
        #[serde(flatten)]
        base: NodeTypeVector,
    },
    #[serde(rename_all = "camelCase")]
    BooleanOperation { children: Vec<Node> },
    #[serde(rename_all = "camelCase")]
    Star,
    #[serde(rename_all = "camelCase")]
    Line,
    #[serde(rename_all = "camelCase")]
    Ellipse,
    #[serde(rename_all = "camelCase")]
    RegularPolygon,
    #[serde(rename_all = "camelCase")]
    Rectangle {
        #[serde(flatten)]
        base: NodeTypeVector,
        #[serde(skip_serializing_if = "Option::is_none")]
        corner_radius: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        rectangle_corner_radii: Option<[f64; 4]>,
    },
    #[serde(rename_all = "camelCase")]
    Text {
        #[serde(flatten)]
        base: NodeTypeVector,
        characters: String,
    },
    #[serde(rename_all = "camelCase")]
    Slice,
    #[serde(rename_all = "camelCase")]
    Component {
        #[serde(flatten)]
        base: NodeTypeFrame,
    },
    #[serde(rename_all = "camelCase")]
    ComponentSet {
        #[serde(flatten)]
        base: NodeTypeFrame,
    },
    #[serde(rename_all = "camelCase")]
    Instance {
        #[serde(flatten)]
        base: NodeTypeFrame,
    },
    #[serde(rename_all = "camelCase")]
    Sticky { characters: String },
    #[serde(rename_all = "camelCase")]
    ShapeWithText { characters: String },
    #[serde(rename_all = "camelCase")]
    Connector { characters: String },
    #[serde(other)]
    Unknown,
}

pub struct NodeDepthFirstStackIterator<'a> {
    iter_stack: Vec<std::slice::Iter<'a, Node>>,
    stack: Vec<&'a Node>,
}

impl<'a> Iterator for NodeDepthFirstStackIterator<'a> {
    type Item = (&'a Node, Vec<&'a Node>);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut bottom_of_iter_stack = self.iter_stack.pop()?;
            let bottom_of_stack = self.stack.pop()?;
            if let Some(current) = bottom_of_iter_stack.next() {
                self.iter_stack.push(bottom_of_iter_stack);
                self.iter_stack.push(current.children().iter());
                self.stack.push(bottom_of_stack);
                self.stack.push(current);
                return Some((current, self.stack.clone()));
            }
        }
    }
}

impl NodeType {
    fn absolute_bounding_box(&self) -> Option<&Rectangle> {
        self.vector_props()
            .and_then(|v| v.absolute_bounding_box.as_ref())
            .or_else(|| {
                self.frame_props()
                    .and_then(|fp| fp.absolute_bounding_box.as_ref())
            })
    }

    fn corner_radius(&self) -> Option<f64> {
        match self {
            NodeType::Rectangle { corner_radius, .. } => *corner_radius,
            _ => self.frame_props().and_then(|fp| fp.corner_radius),
        }
    }

    fn rectangle_corner_radii(&self) -> Option<[f64; 4]> {
        match self {
            NodeType::Rectangle {
                rectangle_corner_radii: Some(rectangle_corner_radii),
                ..
            } => Some(*rectangle_corner_radii),
            NodeType::Rectangle {
                corner_radius: Some(r),
                ..
            } => Some([*r, *r, *r, *r]),
            _ => match self.frame_props() {
                Some(NodeTypeFrame {
                    rectangle_corner_radii: Some(rectangle_corner_radii),
                    ..
                }) => Some(*rectangle_corner_radii),
                Some(NodeTypeFrame {
                    corner_radius: Some(r),
                    ..
                }) => Some([*r, *r, *r, *r]),
                _ => None,
            },
        }
    }

    fn opacity(&self) -> Option<f64> {
        self.vector_props()
            .map(|v| v.opacity)
            .or_else(|| self.frame_props().map(|fp| fp.opacity))
    }

    fn vector_props(&self) -> Option<&NodeTypeVector> {
        match self {
            NodeType::Vector { base, .. } | NodeType::Rectangle { base, .. } => Some(base),
            _ => None,
        }
    }

    fn frame_props(&self) -> Option<&NodeTypeFrame> {
        match self {
            NodeType::Frame { base, .. }
            | NodeType::Group { base, .. }
            | NodeType::Component { base, .. }
            | NodeType::ComponentSet { base, .. }
            | NodeType::Instance { base, .. } => Some(base),
            _ => None,
        }
    }

    fn children(&self) -> &[Node] {
        match self {
            NodeType::Document { children, .. } => children,
            NodeType::Canvas { children, .. } => children,
            NodeType::BooleanOperation { children, .. } => children,
            _ => match self.frame_props() {
                Some(NodeTypeFrame { children, .. }) => children,
                None => &[],
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub document: Node,
    pub components: IndexMap<String, Component>,
    pub styles: IndexMap<String, Style>,
    pub name: String,
    pub schema_version: u8,
    pub version: String,
}
