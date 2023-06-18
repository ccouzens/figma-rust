use std::{io::Write, borrow::{Cow, Borrow}};

use figma_schema::Node;
use html_escape::{encode_style, encode_text};

use self::intermediate_node::{
    format_css, CSSVariable, CSSVariablesMap, HtmlFormatter, IntermediateNode,
};

mod css_properties;
pub mod intermediate_node;

pub fn file_collect_css_variables(file: &figma_schema::File) -> CSSVariablesMap {
    file.styles
        .iter()
        .map(|(key, style)| {
            (
                Cow::Borrowed(key.as_str()),
                CSSVariable {
                    name: format!(
                        "--{}",
                        style.name.replace(|c: char| !c.is_alphanumeric(), "-")
                    ),
                    value: None,
                },
            )
        })
        .collect()
}

pub fn find_figma_node_by_id<'a>(
    file: &'a figma_schema::File,
    node_id: &str,
) -> Option<&'a figma_schema::Node> {
    let (node, _) = file
        .document
        .depth_first_stack_iter()
        .find(|(n, _)| n.id == node_id)?;
    Some(node)
}

pub fn figma_node_to_intermediate_node<'a>(
    node: &'a Node,
    css_variables: &mut CSSVariablesMap,
) -> IntermediateNode<'a> {
    IntermediateNode::from_figma_node(node, None, css_variables)
}

pub fn intermediate_node_to_html_writer(
    writer: &mut impl Write,
    node: &IntermediateNode,
    css_variables: &CSSVariablesMap,
) -> Result<(), std::io::Error> {
    let mut naive_css = "margin: 0;".to_string();
    for v in css_variables.values() {
        if let Some(value) = v.value.as_deref() {
            naive_css.push_str(&v.name);
            naive_css.push_str(": ");
            naive_css.push_str(value);
            naive_css.push(';');
        }
    }

    let body_css = format_css(3, &naive_css).unwrap_or_default();

    writeln!(
        writer,
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
        encode_text(&node.figma.as_ref().map(|f| f.name.borrow()).unwrap_or("")),
        encode_style(&body_css),
        HtmlFormatter {
            intermediate_node: node,
            nesting_depth: 2
        }
    )
}
