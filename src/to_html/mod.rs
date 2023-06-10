use html_escape::{encode_style, encode_text};

use self::intermediate_node::{
    format_css, CSSVariable, CSSVariablesMap, HtmlFormatter, IntermediateNode,
};

use anyhow::{Context, Result};
use std::io::Write;

mod css_properties;
mod intermediate_node;

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
    let mut naive_css = "margin: 0;".to_string();
    for v in css_variables.into_values() {
        if let Some(value) = v.value {
            naive_css.push_str(&v.name);
            naive_css.push_str(": ");
            naive_css.push_str(&value);
            naive_css.push(';');
        }
    }

    let body_css = format_css(3, &naive_css)?;

    writeln!(
        stdout,
        r#"<!DOCTYPE html>
<html
  ><head
    ><meta charset="utf-8" /><title>{}</title
    ><style type="text/css">
      body {{{}}}
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
