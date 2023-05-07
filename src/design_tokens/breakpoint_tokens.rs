use figma_schema::{File, Node};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct BreakpointToken<'a> {
    category: &'a str,
    export_key: &'a str,
    value: f64,
    r#type: &'a str,
    unit: &'a str,
}

pub fn as_breakpoint_token(node: &Node, _file: &File) -> Option<serde_json::Value> {
    let width = node.absolute_bounding_box()?.width?;

    Some(json!(BreakpointToken {
        category: "breakpoint",
        export_key: "breakpoint",
        value: width,
        r#type: "number",
        unit: "pixel"
    }))
}
