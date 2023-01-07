use crate::figma_api::Node;

use super::figma_api;
use anyhow::{Context, Result};
use indexmap::{IndexMap, IndexSet};
use serde::Serialize;
use std::io::Write;

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum MapOrInterface<'a> {
    Map(IndexMap<&'a str, MapOrInterface<'a>>),
    Interface(Interface<'a>),
}

const FAILED_TO_WRITE: &str = "Failed to write";

fn indent(stdout: &mut impl Write, indenation: u16) -> std::io::Result<()> {
    for _ in 0..indenation {
        write!(stdout, "  ")?;
    }
    Ok(())
}

fn to_identifier(raw: &str, capitalize: bool) -> Result<String> {
    fn valid_char(c: char) -> bool {
        c.is_alphanumeric() || c == '_' || c == '$'
    }
    let mut output = String::new();
    let mut chars = raw.chars();
    let first = chars
        .by_ref()
        .find(|c| valid_char(*c) && !c.is_numeric())
        .with_context(|| format!("Couldn't find first letter in {:?}", raw))?;
    if capitalize {
        output.extend(first.to_uppercase());
    } else {
        output.extend(first.to_lowercase());
    }
    let mut word_gap = false;
    for c in chars {
        if valid_char(c) {
            if word_gap {
                output.extend(c.to_uppercase());
            } else {
                output.push(c);
            }
            word_gap = false;
        } else {
            word_gap = true;
        }
    }
    Ok(output)
}

impl<'a> MapOrInterface<'a> {
    fn output(&self, stdout: &mut impl Write, indentation: u16) -> Result<()> {
        match self {
            MapOrInterface::Map(mapping) => {
                for (i, (&key, value)) in mapping.iter().enumerate() {
                    if i != 0 {
                        writeln!(stdout).context(FAILED_TO_WRITE)?;
                    }
                    indent(stdout, indentation).context(FAILED_TO_WRITE)?;
                    writeln!(
                        stdout,
                        "{} {} {{",
                        match value {
                            MapOrInterface::Map(_) => "namespace",
                            MapOrInterface::Interface(_) => "export interface",
                        },
                        to_identifier(key, true).context("Couldn't create name")?
                    )
                    .context(FAILED_TO_WRITE)?;
                    value.output(stdout, indentation + 1)?;
                    indent(stdout, indentation).context(FAILED_TO_WRITE)?;
                    writeln!(stdout, "}}").context(FAILED_TO_WRITE)?;
                }
            }
            MapOrInterface::Interface(interface) => {
                for (&key, value) in interface.0.iter() {
                    indent(stdout, indentation).context(FAILED_TO_WRITE)?;
                    write!(
                        stdout,
                        "{}: ",
                        to_identifier(key, false).context("Couldn't create name")?
                    )
                    .context(FAILED_TO_WRITE)?;
                    if value.contains("True") && value.contains("False") && value.len() == 2 {
                        write!(stdout, "boolean").context(FAILED_TO_WRITE)?;
                    } else if let Ok(values) = value
                        .iter()
                        .map(|&v| str::parse::<f64>(v))
                        .collect::<Result<Vec<f64>, _>>()
                    {
                        for (i, v) in values.iter().enumerate() {
                            if i != 0 {
                                write!(stdout, " | ").context(FAILED_TO_WRITE)?;
                            }
                            write!(stdout, "{}", v).context(FAILED_TO_WRITE)?;
                        }
                    } else {
                        for (i, v) in value.iter().enumerate() {
                            if i != 0 {
                                write!(stdout, " | ").context(FAILED_TO_WRITE)?;
                            }
                            write!(
                                stdout,
                                "{}",
                                serde_json::to_string(v)
                                    .context("Failed to convert to JSON string")?
                            )
                            .context(FAILED_TO_WRITE)?;
                        }
                    }
                    writeln!(stdout, ",",).context(FAILED_TO_WRITE)?;
                }
            }
        };
        Ok(())
    }
}

#[derive(Debug, Default, Serialize)]
struct Interface<'a>(IndexMap<&'a str, IndexSet<&'a str>>);

fn insert_by_name<'a>(
    transformed: &mut MapOrInterface<'a>,
    nodes: &[&'a Node],
    value: Interface<'a>,
) -> bool {
    match transformed {
        MapOrInterface::Map(map) => {
            let head = nodes[0].name.trim();
            let rest = &nodes[1..];
            if rest.is_empty() {
                match map.entry(head) {
                    indexmap::map::Entry::Occupied(_) => false,
                    indexmap::map::Entry::Vacant(vacancy) => {
                        vacancy.insert(MapOrInterface::Interface(value));
                        true
                    }
                }
            } else {
                let entry = map
                    .entry(head)
                    .or_insert_with(|| MapOrInterface::Map(IndexMap::new()));
                insert_by_name(entry, rest, value)
            }
        }
        MapOrInterface::Interface(..) => false,
    }
}

pub fn main(
    file: &figma_api::File,
    stdout: &mut impl Write,
    stderr: &mut impl Write,
) -> Result<()> {
    let mut transformed = MapOrInterface::Map(IndexMap::new());

    for (node, parent_nodes) in file.document.depth_first_stack_iter() {
        if let figma_api::Node {
            node_type: figma_api::NodeType::ComponentSet { .. },
            ..
        } = node
        {
            let mut interface = Interface::default();
            for instance in node.children() {
                for key_value in instance.name.split(", ") {
                    if let Some((key, value)) = key_value.split_once('=') {
                        interface.0.entry(key).or_default().insert(value);
                    }
                }
            }
            if !insert_by_name(&mut transformed, &parent_nodes[1..], interface) {
                writeln!(
                    stderr,
                    "Failed to insert {:?}",
                    parent_nodes.iter().map(|n| &n.name).collect::<Vec<_>>()
                )
                .unwrap();
            };
        };
    }
    transformed
        .output(stdout, 0)
        .context("Failed to write TypeScript")?;

    Ok(())
}
