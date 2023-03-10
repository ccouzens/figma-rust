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
    // serde_json::to_writer_pretty(stdout, &node).context("Failed to write node")?;

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
            }
        }
    )
    .context("Failed to write HTML to stdout")?;

    Ok(())
}
