use super::figma_api;
use anyhow::{anyhow, Context, Result};
use horrorshow::{helper::doctype, html};
use lightningcss::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet};
use std::io::Write;

fn create_css(selector: &str, properties: &[(&str, Option<&str>)]) -> Result<String> {
    let mut style_sheet_text = format!("{selector} {{");
    for (property, value) in properties {
        if let Some(value) = value {
            style_sheet_text.push_str(&format!("{property}: {value};"));
        }
    }
    style_sheet_text.push('}');
    let mut stylesheet = StyleSheet::parse(&style_sheet_text, ParserOptions::default())
        .map_err(|err| anyhow!("Failed to parse CSS for {selector}\n{err}"))?;

    stylesheet
        .minify(MinifyOptions::default())
        .with_context(|| format!("Failed to minify CSS for {selector}"))?;

    Ok(stylesheet
        .to_css(PrinterOptions::default())
        .with_context(|| format!("Failed to print CSS for {selector}"))?
        .code)
}

struct RenderProps<'a> {
    body_css: &'a str,
    component_title: &'a str,
    examples: &'a [ExampleRenderProps],
}

struct ExampleRenderProps {
    inline_css: String,
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
        .with_context(|| format!("Failed to find node with id {}", node_id))?
        .0;

    let absolute_bounding_box = node
        .absolute_bounding_box()
        .context("Failed to load bounding box")?;

    let node_offset_top = absolute_bounding_box.y.context("Failed to load y offset")?;
    let node_offset_left = absolute_bounding_box.x.context("Failed to load x offset")?;

    let body_css = create_css(
        "body",
        &[
            ("box-sizing", Some("border-box")),
            ("position", Some("relative")),
            (
                "width",
                absolute_bounding_box
                    .width
                    .map(|width| format!("{width}px"))
                    .as_deref(),
            ),
            (
                "height",
                absolute_bounding_box
                    .height
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
        ],
    )?;

    let mut example_render_props = Vec::new();

    for component_node in node.children().iter() {
        if let (Some(component_offset_top), Some(component_offset_left)) = (
            component_node.absolute_bounding_box().and_then(|bb| bb.y),
            component_node.absolute_bounding_box().and_then(|bb| bb.x),
        ) {
            example_render_props.push(ExampleRenderProps {
                inline_css: format!(
                    "position: absolute; top: {top}px; left: {left}px;",
                    top = component_offset_top - node_offset_top,
                    left = component_offset_left - node_offset_left
                ),
            });
        }
    }

    let render_props = RenderProps {
        body_css: &body_css,
        component_title: &node.name,
        examples: &example_render_props,
    };

    writeln!(
        stdout,
        "{}",
        html! {
            : doctype::HTML;
            html {
                head {
                    meta(charset="utf-8");
                    title : format!("{} component", render_props.component_title);
                    style(type="text/css"): render_props.body_css;
                }
                body {
                    @ for example_render_prop in render_props.examples.iter() {
                        div(style=&example_render_prop.inline_css): "button"
                    }
                }
            }
        }
    )
    .context("Failed to write HTML to stdout")?;

    Ok(())
}
