use figma_schema::{Node, NodeType};
use html_escape::{encode_style, encode_text};

use self::{
    css_properties::CssProperties,
    intermediate_node::{CSSVariable, CSSVariablesMap, HtmlFormatter, IntermediateNode},
};

use anyhow::{anyhow, Context, Result};
use horrorshow::html;
use lightningcss::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleAttribute};
use std::{io::Write, vec};

mod css_properties;
mod intermediate_node;

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

fn inline_css(
    node: &Node,
    parent: Option<&Node>,
    css_variables: &mut CSSVariablesMap,
) -> Result<Option<String>> {
    let css: Vec<(String, Option<String>)> = vec![
        ("align-items".into(), node.align_items()),
        ("align-self".into(), node.align_self(parent)),
        ("background".into(), node.background(css_variables)),
        ("border-radius".into(), node.border_radius()),
        ("bottom".into(), node.bottom(parent)),
        ("box-shadow".into(), node.box_shadow()),
        ("box-sizing".into(), node.box_sizing(parent)),
        ("color".into(), node.color(css_variables)),
        ("display".into(), node.display()),
        ("fill".into(), node.fill(css_variables)),
        ("flex-direction".into(), node.flex_direction()),
        ("flex-grow".into(), node.flex_grow()),
        ("font".into(), node.font(css_variables)),
        ("gap".into(), node.gap()),
        ("height".into(), node.height(parent)),
        ("justify-content".into(), node.justify_content()),
        ("left".into(), node.left(parent)),
        ("padding".into(), node.padding()),
        ("opacity".into(), CssProperties::opacity(node)),
        ("outline".into(), node.outline()),
        ("outline-offset".into(), node.outline_offset()),
        ("position".into(), node.position(parent)),
        ("right".into(), node.right(parent)),
        ("text-decoration-line".into(), node.text_decoration_line()),
        ("text-transform".into(), node.text_transform()),
        ("top".into(), node.top(parent)),
        ("white-space".into(), node.white_space()),
        ("width".into(), node.width(parent)),
    ];

    let css_string = create_inline_css(&css).context("Failed to generate instance CSS")?;
    if css_string.is_empty() {
        Ok(None)
    } else {
        Ok(Some(css_string))
    }
}

fn node_to_html(node: &Node, parent: Option<&Node>, css_variables: &mut CSSVariablesMap) -> String {
    let style = inline_css(node, parent, css_variables).unwrap_or_default();
    match node.r#type {
        NodeType::Vector | NodeType::BooleanOperation => {
            if CssProperties::opacity(node).as_deref() == Some("0") {
                "".into()
            } else {
                html! {
                    svg( style?=style.as_deref(), viewBox="-5 -81 100 100" ) {
                        text( font-size="90" ) { : "�" }
                    }
                }
                .to_string()
            }
        }
        NodeType::Text => {
            let characters = match node.characters.as_deref() {
                None => return "".into(),
                Some(c) => c,
            };

            let inner = html! {
                @ if !characters.contains('\n') {
                    : &characters
                } else {
                    @ for line in characters.split('\n') {
                        @ if line.is_empty() {
                          p(style=format!("margin: 0; height: {}px", node.style.as_ref().map(|s| s.line_height_px).unwrap_or(0.0)))
                        } else {
                            p(style="margin: 0;") {
                                : line
                            }
                        }
                    }
                }
            };
            let hyperlink = node.style.as_ref().and_then(|s| s.hyperlink.as_ref());

            html! {
              @ if let Some(hyperlink) = hyperlink.and_then(|h| h.url.as_deref().or(match h.node_id { Some(_) => Some("#"), None => None})) {
                    a( style?=style.as_deref(), href=hyperlink )  { : &inner }
                } else {
                     div( style?=style.as_deref() )  { : &inner }
                }
            }
            .to_string()
        }
        _ => {
            let child_nodes = node
                .enabled_children()
                .map(|child| node_to_html(child, Some(node), css_variables))
                .collect::<Vec<_>>();
            html! {
                div( style?=style.as_deref() ) {
                    @ for child_html in child_nodes.iter() {
                        : horrorshow::Raw(child_html)
                    }
                }
            }
            .to_string()
        }
    }
}

pub fn main(
    file: &figma_schema::File,
    stdout: &mut impl Write,
    _stderr: &mut impl Write,
    node_id: &str,
) -> Result<()> {
    let (body, _) = file
        .document
        .depth_first_stack_iter()
        .find(|(n, _)| n.id == node_id)
        .with_context(|| format!("Failed to find node with id {}", node_id))?;

    let mut css_variables: CSSVariablesMap = file
        .styles
        .iter()
        .map(|(key, style)| {
            (
                key.as_str(),
                CSSVariable {
                    name: format!(
                        "--{}",
                        style.name.replace(|c: char| !c.is_alphanumeric(), "-")
                    ),
                    value: None,
                },
            )
        })
        .collect();

    let intermediate_node = IntermediateNode::from_figma_node(body, None, &mut css_variables);

    let mut body_css_properties = vec![("margin".into(), Some("0".into()))];
    body_css_properties.extend(
        css_variables
            .into_values()
            .filter_map(|v| Some((v.name, Some(v.value?)))),
    );
    let body_css = create_inline_css(&body_css_properties)?;

    writeln!(
        stdout,
        r#"<!DOCTYPE html>
<html
  ><head
    ><meta charset="utf-8" /><title>{}</title
    ><style type="text/css">
{}
    </style></head
  ><body
    >{}</body
  ></html
>"#,
        encode_text(&body.name),
        encode_style(&body_css),
        HtmlFormatter {
            intermediate_node: &intermediate_node,
            nesting_depth: 2
        }
    )
    .context("Failed to write HTML to stdout")
}
