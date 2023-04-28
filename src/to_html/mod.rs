use crate::figma_api::{Node, NodeType};

use self::css_properties::CssProperties;

use super::figma_api;
use anyhow::{anyhow, Context, Result};
use horrorshow::{helper::doctype, html};
use lightningcss::stylesheet::{
    MinifyOptions, ParserOptions, PrinterOptions, StyleAttribute, StyleSheet,
};
use std::{io::Write, vec};

mod css_properties;

type CSSRulePairs = (String, Option<String>);

fn create_inline_css(properties: &[CSSRulePairs]) -> Result<String> {
    use std::fmt::Write;

    let mut style_sheet_text = String::new();
    for (property, value) in properties {
        if let Some(value) = value {
            write!(&mut style_sheet_text, "{property}: {value};")
                .context("Failed to write property to string")?;
        }
    }

    let mut style_attribute = StyleAttribute::parse(&style_sheet_text, ParserOptions::default())
        .map_err(|err| anyhow!("Failed to parse CSS\n{err}"))?;

    style_attribute.minify(MinifyOptions::default());

    Ok(style_attribute
        .to_css(PrinterOptions::default())
        .context("Failed to print CSS")?
        .code)
}

fn create_css(selectors: &[(String, Vec<CSSRulePairs>)]) -> Result<String> {
    use std::fmt::Write;

    let mut style_sheet_text = String::new();

    for (selector, properties) in selectors.iter() {
        write!(&mut style_sheet_text, "{selector} {{").context("Failed to write selctor")?;
        for (property, value) in properties {
            if let Some(value) = value {
                write!(&mut style_sheet_text, "{property}: {value};")
                    .context("Failed to write property to string")?;
            }
        }
        style_sheet_text.push('}');
    }
    let mut stylesheet = StyleSheet::parse(&style_sheet_text, ParserOptions::default())
        .map_err(|err| anyhow!("Failed to parse CSS\n{err}"))?;

    stylesheet
        .minify(MinifyOptions::default())
        .context("Failed to minify CSS")?;

    Ok(stylesheet
        .to_css(PrinterOptions::default())
        .context("Failed to print CSS")?
        .code)
}

fn inline_css(node: &Node, parent: Option<&Node>) -> Result<Option<String>> {
    let mut css: Vec<(String, Option<String>)> = vec![
        ("align-items".into(), node.align_items()),
        ("background".into(), node.background()),
        ("border-radius".into(), node.border_radius()),
        ("box-shadow".into(), node.box_shadow()),
        ("color".into(), node.color()),
        ("display".into(), node.display()),
        ("fill".into(), node.fill()),
        ("flex-direction".into(), node.flex_direction()),
        ("font-family".into(), node.font_family()),
        ("font-size".into(), node.font_size()),
        ("font-weight".into(), node.font_weight()),
        ("gap".into(), node.gap()),
        ("height".into(), node.height()),
        ("justify-content".into(), node.justify_content()),
        ("left".into(), node.left(parent)),
        ("line-height".into(), node.line_height()),
        ("padding".into(), node.padding()),
        ("opacity".into(), CssProperties::opacity(node)),
        ("outline".into(), node.outline()),
        ("outline-offset".into(), node.outline_offset()),
        ("position".into(), node.position(parent)),
        ("text-transform".into(), node.text_transform()),
        ("top".into(), node.top(parent)),
        ("width".into(), node.width()),
    ];

    if let (Some(component_height), Some(component_width)) = (
        node.absolute_bounding_box().and_then(|bb| bb.height),
        node.absolute_bounding_box().and_then(|bb| bb.width),
    ) {
        if node.r#type == NodeType::Component {
            css.extend_from_slice(&[
                ("max-width".into(), Some(format!("{component_width}px"))),
                ("max-height".into(), Some(format!("{component_height}px"))),
            ]);
        }
    }

    let css_string = create_inline_css(&css).context("Failed to generate instance CSS")?;
    if css_string.is_empty() {
        Ok(None)
    } else {
        Ok(Some(css_string))
    }
}

fn node_to_html(node: &Node, parent: &Node) -> String {
    match node.r#type {
        NodeType::Vector => html! {
            svg(
                style?=inline_css(node, Some(parent)).unwrap_or_default(),
                data-figma-name=&node.name,
                data-figma-id=&node.id,
                viewBox="0 0 100 100"
            ) {
                text(
                    y=".9em",
                    font-size="90"
                ) {
                    : "�"
                }
            }
        }
        .to_string(),
        _ => html! {
            div(
                style?=inline_css(node, Some(parent)).unwrap_or_default(),
                data-figma-name=&node.name,
                data-figma-id=&node.id
            ) {
                @ for child in node.enabled_children() {
                    : horrorshow::Raw(node_to_html(child, node))
                }
                : &node.characters.as_deref().unwrap_or_default();
            }
        }
        .to_string(),
    }
}

struct RenderProps<'a> {
    body_css: &'a str,
    component_title: &'a str,
    nodes: &'a [Node],
}

pub fn main(
    file: &figma_api::File,
    stdout: &mut impl Write,
    _stderr: &mut impl Write,
    node_id: &str,
) -> Result<()> {
    let (body, parents) = file
        .document
        .depth_first_stack_iter()
        .find(|(n, _)| n.id == node_id)
        .with_context(|| format!("Failed to find node with id {}", node_id))?;

    let canvas = parents
        .iter()
        .find(|n| matches!(n.r#type, NodeType::Canvas))
        .context("Failed to find canvas parent")?;

    let absolute_bounding_box = body
        .absolute_bounding_box()
        .context("Failed to load bounding box")?;

    let global_css = create_css(&[
        (
            "body".into(),
            vec![
                ("margin".into(), Some("0".into())),
                ("position".into(), Some("relative".into())),
                (
                    "width".into(),
                    absolute_bounding_box
                        .width
                        .map(|width| format!("{width}px")),
                ),
                (
                    "height".into(),
                    absolute_bounding_box
                        .height
                        .map(|height| format!("{height}px")),
                ),
                ("background".into(), body.background()),
                ("border-radius".into(), body.border_radius()),
                ("outline".into(), body.outline()),
                ("outline-offset".into(), body.outline_offset()),
            ],
        ),
        (
            "html".into(),
            vec![("background".into(), canvas.background())],
        ),
    ])?;

    let render_props = RenderProps {
        body_css: &global_css,
        component_title: &body.name,
        nodes: body.children(),
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
                    style(type="text/css"): horrorshow::Raw(render_props.body_css);
                }
                body {
                    @ for node in render_props.nodes.iter() {
                        : horrorshow::Raw(node_to_html(node, body))
                    }
                }
            }
        }
    )
    .context("Failed to write HTML to stdout")?;

    Ok(())
}
