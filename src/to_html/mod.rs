use super::figma_api;
use anyhow::{Context, Result};
use horrorshow::{helper::doctype, html};
use std::io::Write;

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

    let node_background_color = node
        .fills()
        .get(0)
        .context("Expected to have a background fill")?
        .color()
        .context("Expected to have a background color")?;

    let node_bounding_box = node
        .absolute_bounding_box()
        .context("Expected to have a render box")?;

    let body_css = format!(
        "
    box-sizing: border-box;
    position: relative;
    width: {width}px;
    height: {height}px;
    background: {background_color};
    border: 1px dashed {border_color};
    border-radius: {border_radius}px;
   ",
        width = node_bounding_box
            .width
            .context("Expected to have a width")?,
        height = node_bounding_box
            .height
            .context("Expected to have a height")?,
        background_color = node_background_color.to_rgb_string(),
        border_color = node
            .strokes()
            .get(0)
            .context("Expected to have a border stroke")?
            .color()
            .context("Expected to have a border color")?
            .to_rgb_string(),
        border_radius = node.corner_radius().context("Expected a border radius")?
    );

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
                body(style=format!("
            box-sizing: border-box;
            position: relative;
            width: 473px;
            height: 345px;
            background: {};
            border: 1px dashed #7b61ff;
            border-radius: 5px;
          ", node_background_color.to_rgb_string())) {}
                body(style=&body_css) {}
            }
        }
    )
    .context("Failed to write HTML to stdout")?;

    Ok(())
}
