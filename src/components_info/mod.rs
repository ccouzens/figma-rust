use crate::figma_api;
use anyhow::{Context, Result};
use indexmap::{IndexMap, IndexSet};
use serde::Serialize;
use std::io::Write;

#[derive(Debug, Default, Serialize, PartialEq, Eq)]
struct ComponentInfo<'a> {
    path: Vec<&'a str>,
    interface: IndexMap<&'a str, IndexSet<&'a str>>,
}

pub fn main(
    file: &figma_api::File,
    stdout: &mut impl Write,
    _stderr: &mut impl Write,
) -> Result<()> {
    let mut info = Vec::new();
    for (node, parent_nodes) in file.document.depth_first_stack_iter() {
        if let figma_api::Node {
            node_type: figma_api::NodeType::ComponentSet { .. },
            ..
        } = node
        {
            let mut interface: IndexMap<&str, IndexSet<&str>> = IndexMap::default();
            for instance in node.children() {
                for key_value in instance.name.split(", ") {
                    if let Some((key, value)) = key_value.split_once('=') {
                        interface.entry(key).or_default().insert(value);
                    }
                }
            }
            info.push(ComponentInfo {
                path: parent_nodes.iter().map(|&n| n.name.as_str()).collect(),
                interface,
            });
        };
    }
    serde_json::to_writer_pretty(stdout, &info)
        .context("Failed to write JSON version of the info")?;

    Ok(())
}
