use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
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
    pub fn frame_props(&self) -> Option<&NodeTypeFrame> {
        self.node_type.frame_props()
    }

    pub fn children(&self) -> &[Node] {
        self.node_type.children()
    }

    pub fn depth_first_iter(&self) -> impl Iterator<Item = &Self> {
        NodeTypeDepthFirstIterator {
            stack: vec![self.children().iter()],
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]

pub struct NodeTypeFrame {
    children: Vec<Node>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_easing: Option<EasingType>,
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
    Vector,
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
    Rectangle,
    #[serde(rename_all = "camelCase")]
    Text { characters: String },
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

struct NodeTypeDepthFirstIterator<'a> {
    stack: Vec<std::slice::Iter<'a, Node>>,
}

impl<'a> Iterator for NodeTypeDepthFirstIterator<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut bottom_of_stack = self.stack.pop()?;
            if let Some(current) = bottom_of_stack.next() {
                self.stack.push(bottom_of_stack);
                self.stack.push(current.children().iter());
                return Some(current);
            }
        }
    }
}

impl NodeType {
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
            NodeType::Frame {
                base: NodeTypeFrame { children, .. },
                ..
            } => children,
            NodeType::Group {
                base: NodeTypeFrame { children, .. },
                ..
            } => children,
            NodeType::Vector => &[],
            NodeType::BooleanOperation { children, .. } => children,
            NodeType::Star => &[],
            NodeType::Line => &[],
            NodeType::Ellipse => &[],
            NodeType::RegularPolygon => &[],
            NodeType::Rectangle => &[],
            NodeType::Text { .. } => &[],
            NodeType::Slice => &[],
            NodeType::Component {
                base: NodeTypeFrame { children, .. },
                ..
            } => children,
            NodeType::ComponentSet {
                base: NodeTypeFrame { children, .. },
                ..
            } => children,
            NodeType::Instance {
                base: NodeTypeFrame { children, .. },
                ..
            } => children,
            NodeType::Sticky { .. } => &[],
            NodeType::ShapeWithText { .. } => &[],
            NodeType::Connector { .. } => &[],
            NodeType::Unknown => &[],
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub document: Node,
    pub name: String,
    pub schema_version: u8,
    pub version: String,
}
