use serde_json::json;

use crate::{figma_api::Node, node_match_prefix};

pub fn as_size_token(node: &Node) -> Option<serde_json::Value> {
    if !node_match_prefix(&["size", "sizes"], node) {
        return None;
    }
    let frame_props = node.frame_props()?;
    let width = frame_props.absolute_bounding_box.as_ref()?.width?;

    Some(json!({
        "category": "size",
        "exportKey": "size",
        "value": width,
        "type": "number",
        "unit": "pixel"
    }))
}
