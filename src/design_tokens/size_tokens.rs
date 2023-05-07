use figma_schema::{self, File, Node};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SizeToken<'a> {
    category: &'a str,
    export_key: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    comment: Option<&'a str>,
    value: f64,
    r#type: &'a str,
    unit: &'a str,
}

pub fn as_size_token(node: &Node, file: &File) -> Option<serde_json::Value> {
    if !matches!(
        node.r#type,
        figma_schema::NodeType::Component { .. }
            | figma_schema::NodeType::Rectangle { .. }
            | figma_schema::NodeType::Frame { .. }
    ) {
        return None;
    }
    let width = node.absolute_bounding_box()?.width?;
    let component = node.component(file);

    Some(json!(SizeToken {
        category: "size",
        export_key: "size",
        comment: component.and_then(|c| if c.description.is_empty() {
            None
        } else {
            Some(c.description.as_str())
        }),
        value: width,
        r#type: "number",
        unit: "pixel"
    }))
}
