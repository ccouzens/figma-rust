use crate::figma_api::{File, Node};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct OpacityToken<'a> {
    category: &'a str,
    export_key: &'a str,
    r#type: &'a str,
    value: f64,
}

pub fn as_opacity_token(node: &Node, _file: &File) -> Option<serde_json::Value> {
    Some(json!(OpacityToken {
        category: "opacity",
        export_key: "opacity",
        r#type: "number",
        value: node.opacity()
    }))
}
