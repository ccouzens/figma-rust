use crate::figma_api::{File, Node};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
struct Direction<'a> {
    value: f64,
    r#type: &'a str,
    unit: &'a str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SpacingToken<'a> {
    category: &'a str,
    export_key: &'a str,
    top: Direction<'a>,
    right: Direction<'a>,
    bottom: Direction<'a>,
    left: Direction<'a>,
}

pub fn as_spacing_token(node: &Node, _file: &File) -> Option<serde_json::Value> {
    Some(json!(SpacingToken {
        category: "spacing",
        export_key: "spacing",
        top: Direction {
            value: node.padding_top(),
            r#type: "number",
            unit: "pixel"
        },
        right: Direction {
            value: node.padding_right(),
            r#type: "number",
            unit: "pixel"
        },
        bottom: Direction {
            value: node.padding_bottom(),
            r#type: "number",
            unit: "pixel"
        },
        left: Direction {
            value: node.padding_left(),
            r#type: "number",
            unit: "pixel"
        },
    }))
}
