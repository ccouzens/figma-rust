use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct GlobalProperties {
    id: String,
    name: String,
}

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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Node {
    #[serde(flatten)]
    global_properties: GlobalProperties,
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
    #[serde(other)]
    Unknown,
}

impl NodeType {
    fn children(&self) -> &[Node] {
        match self {
            NodeType::Document { children } => &children,
            NodeType::Canvas { children, .. } => &children,
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
