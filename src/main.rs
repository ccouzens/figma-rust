use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Color {
    #[serde(rename = "r")]
    red: f64,
    #[serde(rename = "g")]
    green: f64,
    #[serde(rename = "b")]
    blue: f64,
    #[serde(rename = "a")]
    alpha: f64,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Node {
    id: String,
    name: String,
    #[serde(default = "default_true")]
    visible: bool,
    #[serde(flatten)]
    node_type: NodeType,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "type")]
enum NodeType {
    #[serde(rename_all = "camelCase")]
    Document { children: Vec<Node> },
    #[serde(rename_all = "camelCase")]
    Canvas {
        background_color: Color,
        children: Vec<Node>,
    },
    #[serde(rename_all = "camelCase")]
    Frame { children: Vec<Node> },
    #[serde(rename_all = "camelCase")]
    Group { children: Vec<Node> },
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
    Component { children: Vec<Node> },
    #[serde(rename_all = "camelCase")]
    ComponentSet { children: Vec<Node> },
    #[serde(rename_all = "camelCase")]
    Instance { children: Vec<Node> },
    #[serde(rename_all = "camelCase")]
    Sticky { characters: String },
    #[serde(rename_all = "camelCase")]
    ShapeWithText { characters: String },
    #[serde(rename_all = "camelCase")]
    Connector { characters: String },
    #[serde(other)]
    Unknown,
}

impl NodeType {
    fn children(&self) -> &[Node] {
        match self {
            NodeType::Document { children, .. } => &children,
            NodeType::Canvas { children, .. } => &children,
            NodeType::Frame { children, .. } => &children,
            NodeType::Group { children, .. } => &children,
            NodeType::Vector => &[],
            NodeType::BooleanOperation { children, .. } => &children,
            NodeType::Star => &[],
            NodeType::Line => &[],
            NodeType::Ellipse => &[],
            NodeType::RegularPolygon => &[],
            NodeType::Rectangle => &[],
            NodeType::Text { .. } => &[],
            NodeType::Slice => &[],
            NodeType::Component { children, .. } => &children,
            NodeType::ComponentSet { children, .. } => &children,
            NodeType::Instance { children, .. } => &children,
            NodeType::Sticky { .. } => &[],
            NodeType::ShapeWithText { .. } => &[],
            NodeType::Connector { .. } => &[],
            NodeType::Unknown => &[],
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct File {
    document: Node,
    name: String,
    schema_version: u8,
    version: String,
}

fn main() {
    let f: File = serde_json::from_reader(std::io::stdin()).unwrap();
    println!("{}", serde_json::to_string_pretty(&f).unwrap());
}
