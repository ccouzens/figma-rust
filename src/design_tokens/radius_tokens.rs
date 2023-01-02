use crate::figma_api::{File, Node};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct FloatValueToken<'a> {
    value: f64,
    r#type: &'a str,
    unit: Option<&'a str>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct RadiusType<'a> {
    value: &'a str,
    r#type: &'a str,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Radii<'a> {
    top_left: FloatValueToken<'a>,
    top_right: FloatValueToken<'a>,
    bottom_right: FloatValueToken<'a>,
    bottom_left: FloatValueToken<'a>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct RadiusToken<'a> {
    category: &'a str,
    export_key: &'a str,
    radius: Option<FloatValueToken<'a>>,
    radius_type: RadiusType<'a>,
    radii: Radii<'a>,
}

pub fn as_radius_token(node: &Node, _file: &File) -> Option<serde_json::Value> {
    let corner_radius = node.corner_radius();
    let corner_radii = node.rectangle_corner_radii()?;

    Some(json!(RadiusToken {
        category: "radius",
        export_key: "radius",
        radius_type: RadiusType {
            value: if corner_radius.is_some() {
                "single"
            } else {
                "mixed"
            },
            r#type: "string"
        },
        radii: Radii {
            top_left: FloatValueToken {
                value: corner_radii[0],
                r#type: "number",
                unit: Some("pixel")
            },
            top_right: FloatValueToken {
                value: corner_radii[1],
                r#type: "number",
                unit: Some("pixel")
            },
            bottom_right: FloatValueToken {
                value: corner_radii[2],
                r#type: "number",
                unit: Some("pixel")
            },
            bottom_left: FloatValueToken {
                value: corner_radii[3],
                r#type: "number",
                unit: Some("pixel")
            },
        },
        radius: corner_radius.map(|cr| FloatValueToken {
            value: cr,
            r#type: "number",
            unit: Some("pixel")
        })
    }))
}
