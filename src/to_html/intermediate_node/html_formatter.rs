use std::fmt::Display;

use super::{IntermediateNode, IntermediateNodeType};

pub struct HtmlFormatter<'a> {
    pub intermediate_node: &'a IntermediateNode<'a>,
    pub nesting_depth: u16,
}

use html_escape::{encode_double_quoted_attribute, encode_text};

fn indent(f: &mut std::fmt::Formatter<'_>, level: u16) -> std::fmt::Result {
    for _ in 0..level {
        write!(f, "  ")?
    }

    Ok(())
}

fn open_start_tag(f: &mut std::fmt::Formatter<'_>, _level: u16, name: &str) -> std::fmt::Result {
    writeln!(f, "<{name}")
}

fn close_start_tag(f: &mut std::fmt::Formatter<'_>, level: u16) -> std::fmt::Result {
    indent(f, level + 1)?;
    write!(f, ">")
}

fn attribute(
    f: &mut std::fmt::Formatter<'_>,
    level: u16,
    name: &str,
    value: &str,
) -> std::fmt::Result {
    indent(f, level + 2)?;
    writeln!(f, "{name}=\"{}\"", encode_double_quoted_attribute(value))
}

fn text(f: &mut std::fmt::Formatter<'_>, _level: u16, value: &str) -> std::fmt::Result {
    write!(f, "{}", encode_text(value))
}

fn end_tag(f: &mut std::fmt::Formatter<'_>, level: u16, name: &str) -> std::fmt::Result {
    writeln!(f, "</{name}")?;
    indent(f, level)?;
    write!(f, ">")
}

fn common_attributes(
    f: &mut std::fmt::Formatter<'_>,
    level: u16,
    intermediate_node: &IntermediateNode<'_>,
) -> std::fmt::Result {
    attribute(f, level, "data-figma-name", intermediate_node.figma.name)?;
    attribute(f, level, "data-figma-id", intermediate_node.figma.id)?;
    attribute(
        f,
        level,
        "data-figma-type",
        &format!("{:?}", intermediate_node.figma.r#type),
    )?;
    Ok(())
}

impl<'a> Display for HtmlFormatter<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.intermediate_node.node_type {
            IntermediateNodeType::Vector => {
                open_start_tag(f, self.nesting_depth, "svg")?;
                common_attributes(f, self.nesting_depth, self.intermediate_node)?;
                attribute(f, self.nesting_depth, "viewBox", "-5 -81 100 100")?;
                close_start_tag(f, self.nesting_depth)?;
                open_start_tag(f, self.nesting_depth + 1, "text")?;
                attribute(f, self.nesting_depth + 1, "font-size", "90")?;
                close_start_tag(f, self.nesting_depth + 1)?;
                text(f, self.nesting_depth + 1, "�")?;
                end_tag(f, self.nesting_depth + 1, "text")?;
                end_tag(f, self.nesting_depth, "svg")?;
            }
            IntermediateNodeType::Text { text: inner_text } => {
                open_start_tag(f, self.nesting_depth, "div")?;
                common_attributes(f, self.nesting_depth, self.intermediate_node)?;
                close_start_tag(f, self.nesting_depth)?;
                text(f, self.nesting_depth, inner_text)?;
                end_tag(f, self.nesting_depth, "div")?;
            }
            IntermediateNodeType::Frame { children } => {
                open_start_tag(f, self.nesting_depth, "div")?;
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
                end_tag(f, self.nesting_depth, "div")?;
            }
        }
        Ok(())
    }
}
