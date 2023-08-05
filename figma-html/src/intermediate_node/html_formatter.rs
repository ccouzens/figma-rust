use anyhow::{anyhow, Context, Result};
use lightningcss::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleAttribute};
use std::{
    borrow::Borrow,
    fmt::{Display, Write},
};

use super::{IntermediateNode, IntermediateNodeType};

pub struct HtmlFormatter<'a> {
    pub intermediate_node: &'a IntermediateNode<'a>,
    pub nesting_depth: u16,
}

use html_escape::{encode_double_quoted_attribute, encode_text};

fn indent(f: &mut impl Write, level: u16) -> std::fmt::Result {
    for _ in 0..level {
        write!(f, "  ")?
    }

    Ok(())
}

fn open_start_tag(f: &mut impl Write, _level: u16, name: &str) -> std::fmt::Result {
    writeln!(f, "<{name}")
}

fn close_start_tag(f: &mut impl Write, level: u16) -> std::fmt::Result {
    indent(f, level + 1)?;
    write!(f, ">")
}

fn close_self_closing_tag(f: &mut impl Write, level: u16) -> std::fmt::Result {
    indent(f, level)?;
    write!(f, "/>")
}

fn attribute(f: &mut impl Write, level: u16, name: &str, value: &str) -> std::fmt::Result {
    indent(f, level + 1)?;
    writeln!(f, "{name}=\"{}\"", encode_double_quoted_attribute(value))
}

fn text(f: &mut impl Write, _level: u16, value: &str) -> std::fmt::Result {
    write!(f, "{}", encode_text(value))
}

fn end_tag(f: &mut impl Write, level: u16, name: &str) -> std::fmt::Result {
    writeln!(f, "</{name}")?;
    indent(f, level)?;
    write!(f, ">")
}

pub fn format_css(level: u16, naive_css: &str) -> Result<String> {
    let mut style_attribute = StyleAttribute::parse(naive_css, ParserOptions::default())
        .map_err(|err| anyhow!("Failed to parse CSS\n{err}"))?;

    style_attribute.minify(MinifyOptions::default());

    let mut output = String::new();

    for declaration in style_attribute.declarations.declarations.iter() {
        output.push('\n');
        indent(&mut output, level + 1)?;
        output.push_str(
            &declaration
                .to_css_string(false, PrinterOptions::default())
                .context("Failed to write CSS property")?,
        );
        output.push(';');
    }
    output.push('\n');
    indent(&mut output, level)?;

    Ok(output)
}

fn common_attributes(
    f: &mut impl Write,
    level: u16,
    intermediate_node: &IntermediateNode<'_>,
) -> std::fmt::Result {
    if let Some(figma) = intermediate_node.figma.as_ref() {
        attribute(f, level, "data-figma-name", figma.name.borrow())?;
        attribute(f, level, "data-figma-id", figma.id.borrow())?;
    }
    if let Some(href) = intermediate_node.href.as_deref() {
        attribute(f, level, "href", href.borrow())?;
    }
    let css = format_css(level + 1, &intermediate_node.naive_css_string()).unwrap();
    if !css.trim().is_empty() {
        attribute(f, level, "style", &css)?;
    }
    Ok(())
}

impl<'a> Display for HtmlFormatter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let container_type = if self.intermediate_node.href.is_some() {
            "a"
        } else {
            "div"
        };
        match &self.intermediate_node.node_type {
            IntermediateNodeType::Vector => {
                open_start_tag(f, self.nesting_depth, "svg")?;
                common_attributes(f, self.nesting_depth, self.intermediate_node)?;
                attribute(f, self.nesting_depth, "viewBox", "0 0 1 1")?;
                close_start_tag(f, self.nesting_depth)?;
                open_start_tag(f, self.nesting_depth + 1, "rect")?;
                attribute(f, self.nesting_depth + 1, "x", "-0.2")?;
                attribute(f, self.nesting_depth + 1, "y", "-0.2")?;
                attribute(f, self.nesting_depth + 1, "width", "1.4")?;
                attribute(f, self.nesting_depth + 1, "height", "1.4")?;
                close_self_closing_tag(f, self.nesting_depth + 1)?;
                end_tag(f, self.nesting_depth, "svg")?;
            }
            IntermediateNodeType::Text { text: inner_text } => {
                open_start_tag(f, self.nesting_depth, container_type)?;
                common_attributes(f, self.nesting_depth, self.intermediate_node)?;
                close_start_tag(f, self.nesting_depth)?;
                text(f, self.nesting_depth, inner_text)?;
                end_tag(f, self.nesting_depth, container_type)?;
            }
            IntermediateNodeType::Frame { children } => {
                open_start_tag(f, self.nesting_depth, container_type)?;
                common_attributes(f, self.nesting_depth, self.intermediate_node)?;
                close_start_tag(f, self.nesting_depth)?;
                for child in children.iter() {
                    write!(
                        f,
                        "{}",
                        HtmlFormatter {
                            nesting_depth: self.nesting_depth + 1,
                            intermediate_node: child
                        }
                    )?;
                }
                end_tag(f, self.nesting_depth, container_type)?;
            }
        }
        Ok(())
    }
}
