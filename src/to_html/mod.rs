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
    let css: Vec<(String, Option<String>)> = vec![
        ("align-items".into(), node.align_items()),
        ("background".into(), node.background()),
        ("border-radius".into(), node.border_radius()),
        ("box-shadow".into(), node.box_shadow()),
        ("box-sizing".into(), node.box_sizing()),
        ("color".into(), node.color()),
        ("display".into(), node.display()),
        ("fill".into(), node.fill()),
        ("flex-direction".into(), node.flex_direction()),
        ("flex-grow".into(), node.flex_grow()),
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
        ("text-decoration-line".into(), node.text_decoration_line()),
        ("text-transform".into(), node.text_transform()),
        ("top".into(), node.top(parent)),
        ("width".into(), node.width()),
    ];

    let css_string = create_inline_css(&css).context("Failed to generate instance CSS")?;
    if css_string.is_empty() {
        Ok(None)
    } else {
        Ok(Some(css_string))
    }
}

fn node_to_html(node: &Node, parent: Option<&Node>) -> String {
    match node.r#type {
        NodeType::Vector | NodeType::BooleanOperation => html! {
            svg(
                style?=inline_css(node, parent).unwrap_or_default(),
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
                style?=inline_css(node, parent).unwrap_or_default(),
                data-figma-name=&node.name,
                data-figma-id=&node.id
            ) {
                @ for child in node.enabled_children() {
                    : horrorshow::Raw(node_to_html(child, Some(node)))
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
    node: &'a Node,
}

pub fn main(
    file: &figma_api::File,
    stdout: &mut impl Write,
    _stderr: &mut impl Write,
    node_id: &str,
) -> Result<()> {
    let (body, _) = file
        .document
        .depth_first_stack_iter()
        .find(|(n, _)| n.id == node_id)
        .with_context(|| format!("Failed to find node with id {}", node_id))?;

    let global_css = create_css(&[("body".into(), vec![("margin".into(), Some("0".into()))])])?;

    let render_props = RenderProps {
        body_css: &global_css,
        component_title: &body.name,
        node: body,
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
                    : horrorshow::Raw(node_to_html(render_props.node, None))
                }
            }
        }
    )
    .context("Failed to write HTML to stdout")?;

    Ok(())
}
