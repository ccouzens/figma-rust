use super::{Color, Component, EasingType, File, Paint, Rectangle};
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
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
    #[serde(flatten)]
    pub node_type: NodeType,
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<Node>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fills: Option<Vec<Paint>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strokes: Option<Vec<Paint>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stroke_weight: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    characters: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    background_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    absolute_bounding_box: Option<Rectangle>,
}

impl Node {
    pub fn visible(&self) -> bool {
        self.visible.unwrap_or(true)
    }

    pub fn absolute_bounding_box(&self) -> Option<&Rectangle> {
        self.absolute_bounding_box.as_ref()
    }

    pub fn corner_radius(&self) -> Option<f64> {
        self.node_type.corner_radius()
    }

    pub fn rectangle_corner_radii(&self) -> Option<[f64; 4]> {
        self.node_type.rectangle_corner_radii()
    }

    pub fn opacity(&self) -> f64 {
        self.opacity.unwrap_or(1.0)
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
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "type")]
pub enum NodeType {
    Document,
    #[serde(rename_all = "camelCase")]
    Canvas,
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
    Vector,
    BooleanOperation,
    Star,
    Line,
    Ellipse,
    RegularPolygon,
    #[serde(rename_all = "camelCase")]
    Rectangle {
        #[serde(skip_serializing_if = "Option::is_none")]
        corner_radius: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        rectangle_corner_radii: Option<[f64; 4]>,
    },
    Text,
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
