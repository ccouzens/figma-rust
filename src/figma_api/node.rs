use super::{default_one, Color, Component, EasingType, File, Paint, Rectangle};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum StyleTypeMapKey {
    Fills,
    Grid,
    Effect,
    Strokes,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StrokeAlign {
    Inside,
    Outside,
    Center,
}

/// [Figma documentation](https://www.figma.com/developers/api#node-types)
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: String,
    pub name: String,
    #[serde(default)]
    visible: Option<bool>,
    #[serde(flatten)]
    pub node_type: NodeType,
    #[serde(default)]
    children: Option<Vec<Node>>,
    #[serde(default)]
    fills: Option<Vec<Paint>>,
    #[serde(default)]
    strokes: Option<Vec<Paint>>,
    #[serde(default)]
    stroke_weight: Option<f64>,
    #[serde(default)]
    characters: Option<String>,
}

impl Node {
    pub fn visible(&self) -> bool {
        self.visible.unwrap_or(true)
    }

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
        self.children.as_deref().unwrap_or_default()
    }

    pub fn fills(&self) -> &[Paint] {
        self.fills.as_deref().unwrap_or_default()
    }

    pub fn strokes(&self) -> &[Paint] {
        self.strokes.as_deref().unwrap_or_default()
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

    pub fn stroke_weight(&self) -> Option<f64> {
        self.stroke_weight
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeTypeFrame {
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
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "type")]
pub enum NodeType {
    Document,
    #[serde(rename_all = "camelCase")]
    Canvas {
        background_color: Color,
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
    BooleanOperation,
    Star,
    Line,
    Ellipse,
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
    },
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
    Sticky,
    ShapeWithText,
    Connector,
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
}
