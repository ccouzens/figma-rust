use super::figma_api;
use anyhow::{Context, Result};
use horrorshow::{helper::doctype, html};
use std::io::Write;

fn create_css(properties: &[(&str, Option<&str>)]) -> String {
    let mut output = String::new();
    for (property, value) in properties {
        if let Some(value) = value {
            output.push_str(&format!("{property}: {value};"));
        }
    }
    output
}

pub fn main(
    file: &figma_api::File,
    stdout: &mut impl Write,
    _stderr: &mut impl Write,
    node_id: &str,
) -> Result<()> {
    let node = file
        .document
        .depth_first_stack_iter()
        .find(|(n, _)| n.id == node_id)
        .context(format!("Failed to find node with id {}", node_id))?
        .0;

    let body_css = create_css(&[
        ("box-sizing", Some("border-box")),
        ("position", Some("relative")),
        (
            "width",
            node.absolute_bounding_box()
                .and_then(|bb| bb.width)
                .map(|width| format!("{width}px"))
                .as_deref(),
        ),
        (
            "height",
            node.absolute_bounding_box()
                .and_then(|bb| bb.height)
                .map(|height| format!("{height}px"))
                .as_deref(),
        ),
        (
            "background-color",
            node.fills()
                .get(0)
                .and_then(|fill| fill.color())
                .map(|color| color.to_rgb_string())
                .as_deref(),
        ),
        ("border-width", Some("1px")),
        ("border-style", Some("dashed")),
        (
            "border-color",
            node.strokes()
                .get(0)
                .and_then(|stroke| stroke.color())
                .map(|color| color.to_rgb_string())
                .as_deref(),
        ),
        (
            "border-radius",
            node.corner_radius()
                .map(|radius| format!("{radius}px"))
                .as_deref(),
        ),
    ]);

    writeln!(
        stdout,
        "{}",
        html! {
            : doctype::HTML;
            html {
                head {
                    meta(charset="utf-8");
                    title : format!("{} component", node.name);
                    style(type="text/css");
                }
                body(style=&body_css) {}
            }
        }
    )
    .context("Failed to write HTML to stdout")?;

    Ok(())
}
