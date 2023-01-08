use crate::figma_api::Node;

use super::figma_api;
use anyhow::{Context, Result};
use indexmap::{IndexMap, IndexSet};
use serde::Serialize;
use std::{borrow::Cow, io::Write};

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum MapOrInterface<'a> {
    Map(IndexMap<&'a str, MapOrInterface<'a>>),
    Interface(Interface<'a>),
}

#[derive(Debug, Serialize)]
struct Interface<'a> {
    types: IndexMap<&'a str, IndexSet<&'a str>>,
    parent_nodes: Vec<&'a figma_api::Node>,
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
    fn output_consts(&self, stdout: &mut impl Write, indentation: u16) -> Result<()> {
        match self {
            MapOrInterface::Map(mapping) => {
                for (i, (&key, value)) in mapping.iter().enumerate() {
                    if i != 0 {
                        writeln!(stdout).context(FAILED_TO_WRITE)?;
                    }
                    indent(stdout, indentation).context(FAILED_TO_WRITE)?;
                    writeln!(
                        stdout,
                        "{key}: {{",
                        key = serde_json::to_value(key.trim()).context("Couldn't create name")?
                    )
                    .context(FAILED_TO_WRITE)?;
                    value.output_consts(stdout, indentation + 1)?;
                    indent(stdout, indentation).context(FAILED_TO_WRITE)?;
                    writeln!(stdout, "}},").context(FAILED_TO_WRITE)?;
                }
            }
            MapOrInterface::Interface(interface) => {
                for (&key, values) in interface.types.iter() {
                    indent(stdout, indentation).context(FAILED_TO_WRITE)?;
                    write!(
                        stdout,
                        "{key}: [",
                        key = to_identifier(key, false).context("Couldn't create name")?
                    )
                    .context(FAILED_TO_WRITE)?;
                    let ts_values: Vec<Cow<str>> = values
                        .iter()
                        .map(|&v| match v {
                            "True" => Some(Cow::Borrowed("true")),
                            "False" => Some(Cow::Borrowed("false")),
                            _ => None,
                        })
                        .collect::<Option<_>>()
                        .map(Ok)
                        .or_else(|| {
                            values
                                .iter()
                                .map(|&v| str::parse::<f64>(v).ok().map(|_| Cow::Borrowed(v)))
                                .collect::<Option<_>>()
                                .map(Ok)
                        })
                        .unwrap_or_else(|| {
                            values
                                .iter()
                                .map(|v| {
                                    serde_json::to_string(v)
                                        .map(Cow::Owned)
                                        .context("Failed to convert to JSON string")
                                })
                                .collect()
                        })?;
                    for (i, v) in ts_values.iter().enumerate() {
                        if i != 0 {
                            write!(stdout, ", ").context(FAILED_TO_WRITE)?;
                        }
                        write!(stdout, "{}", v).context(FAILED_TO_WRITE)?;
                    }
                    writeln!(stdout, "],").context(FAILED_TO_WRITE)?;
                }
            }
        };
        Ok(())
    }

    fn output_interfaces(
        &self,
        stdout: &mut impl Write,
        indentation: u16,
        const_identifier: &str,
    ) -> Result<()> {
        match self {
            MapOrInterface::Map(mapping) => {
                for (i, (&key, value)) in mapping.iter().enumerate() {
                    if i != 0 {
                        writeln!(stdout).context(FAILED_TO_WRITE)?;
                    }
                    indent(stdout, indentation).context(FAILED_TO_WRITE)?;
                    writeln!(
                        stdout,
                        "{key}: {{",
                        key = serde_json::to_value(key.trim()).context("Couldn't create name")?
                    )
                    .context(FAILED_TO_WRITE)?;
                    value.output_interfaces(stdout, indentation + 1, const_identifier)?;
                    indent(stdout, indentation).context(FAILED_TO_WRITE)?;
                    writeln!(stdout, "}};").context(FAILED_TO_WRITE)?;
                }
            }
            MapOrInterface::Interface(interface) => {
                for &key in interface.types.keys() {
                    let key_identifier =
                        to_identifier(key, false).context("Couldn't create name")?;
                    indent(stdout, indentation).context(FAILED_TO_WRITE)?;
                    write!(stdout, "{key_identifier}: typeof {const_identifier}",)
                        .context(FAILED_TO_WRITE)?;

                    for node in interface.parent_nodes.iter().skip(1) {
                        write!(
                            stdout,
                            "[{key}]",
                            key = serde_json::to_value(node.name.trim())
                                .context("Couldn't create name")?
                        )
                        .context(FAILED_TO_WRITE)?;
                    }
                    writeln!(stdout, r#"["{key_identifier}"][number];"#).context(FAILED_TO_WRITE)?;
                }
            }
        };
        Ok(())
    }
}

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
            let mut interface = Interface {
                types: Default::default(),
                parent_nodes: parent_nodes.clone(),
            };
            for instance in node.children() {
                for key_value in instance.name.split(", ") {
                    if let Some((key, value)) = key_value.split_once('=') {
                        interface.types.entry(key).or_default().insert(value);
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
    let main_identifier = to_identifier(&file.name, true)
        .context("Failed to convert file name to TypeScript identifier")?;

    let const_identifier = format!("{}Consts", main_identifier);
    writeln!(
        stdout,
        r#"// Generated by `figma-rust component-interfaces
// Using file version {version}"

/**
 * Component consts for Figma file {name}
 */
export const {const_identifier} = {{"#,
        version = &file.version,
        name = &file.name,
    )
    .context(FAILED_TO_WRITE)?;

    transformed
        .output_consts(stdout, 1)
        .context("Failed to write TypeScript consts")?;

    writeln!(
        stdout,
        r#"}} as const;

/**
 * Component types for Figma file {name}
 */
export interface {main_identifier}Types {{"#,
        name = &file.name,
    )
    .context(FAILED_TO_WRITE)?;

    transformed
        .output_interfaces(stdout, 1, &const_identifier)
        .context("Failed to write TypeScript interfaces")?;
    writeln!(stdout, "}};",).context(FAILED_TO_WRITE)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expected_stdout() {
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        main(
            &serde_json::from_str(include_str!(
                "../../example-figma-files/gov-uk-design-system.json"
            ))
            .unwrap(),
            &mut stdout,
            &mut stderr,
        )
        .unwrap();

        let output = String::from_utf8_lossy(&stdout);
        let expected = include_str!("./example-output.ts");
        assert_eq!(output.len(), expected.len());

        // Don't use assert_eq! as the output is too long to sensibly read
        assert!(output == expected);
    }
}
