use figma_schema::{self, File, Node, NodeType};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
struct StrokeAlign<'a> {
    value: &'a str,
    r#type: &'a str,
}

#[derive(Debug, Serialize)]
struct StrokeCap<'a> {
    value: &'a str,
    r#type: &'a str,
}

#[derive(Debug, Serialize)]
struct StrokeJoin<'a> {
    value: &'a str,
    r#type: &'a str,
}

#[derive(Debug, Serialize)]
struct StrokeMiterLimit<'a> {
    value: u8,
    r#type: &'a str,
    unit: &'a str,
}

#[derive(Debug, Serialize)]
struct StrokeWeight<'a> {
    value: f64,
    r#type: &'a str,
    unit: &'a str,
}

#[derive(Debug, Serialize)]
struct Stroke<'a> {
    value: String,
    r#type: &'a str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct BorderToken<'a> {
    category: &'a str,
    export_key: &'a str,
    stroke_align: StrokeAlign<'a>,
    stroke_cap: StrokeCap<'a>,
    stroke_join: StrokeJoin<'a>,
    stroke_miter_limit: StrokeMiterLimit<'a>,
    stroke_weight: StrokeWeight<'a>,
    stroke: Stroke<'a>,
}

pub fn as_border_token(node: &Node, _file: &File) -> Option<serde_json::Value> {
    if !matches!(
        node.r#type,
        NodeType::Frame
            | NodeType::Group
            | NodeType::Component
            | NodeType::ComponentSet
            | NodeType::Instance
    ) {
        return None;
    }
    let stroke = node.strokes().first()?;

    Some(json!(BorderToken {
        category: "border",
        export_key: "border",
        stroke_align: StrokeAlign {
            value: match node.stroke_align()? {
                figma_schema::StrokeAlign::Inside => "inside",
                figma_schema::StrokeAlign::Outside => "outside",
                figma_schema::StrokeAlign::Center => "center",
            },
            r#type: "string"
        },
        stroke_cap: StrokeCap {
            value: "none",
            r#type: "string"
        },
        stroke_join: StrokeJoin {
            value: "miter",
            r#type: "string"
        },
        stroke_miter_limit: StrokeMiterLimit {
            value: 4,
            r#type: "number",
            unit: "degree"
        },
        stroke_weight: StrokeWeight {
            value: node.stroke_weight()?,
            r#type: "number",
            unit: "pixel"
        },
        stroke: Stroke {
            value: stroke.color()?.to_rgb_string(),
            r#type: "color"
        }
    }))
}
