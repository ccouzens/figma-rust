use crate::figma_api::Node;

use super::figma_api;
use anyhow::{Context, Result};
use serde::Serialize;
use std::collections::{btree_map, BTreeMap};
use std::{collections::BTreeSet, io::Write};

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum MapOrInterface<'a> {
    Map(BTreeMap<&'a str, MapOrInterface<'a>>),
    Interface(Interface<'a>),
}

#[derive(Debug, Default, Serialize)]
struct Interface<'a>(BTreeMap<&'a str, BTreeSet<&'a str>>);

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
                    btree_map::Entry::Occupied(_) => false,
                    btree_map::Entry::Vacant(vacancy) => {
                        vacancy.insert(MapOrInterface::Interface(value));
                        true
                    }
                }
            } else {
                let entry = map
                    .entry(head)
                    .or_insert_with(|| MapOrInterface::Map(BTreeMap::new()));
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
    let mut transformed = MapOrInterface::Map(BTreeMap::new());

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
    serde_json::to_writer_pretty(stdout, &transformed)
        .context("Failed to write JSON version of the interfaces")?;

    Ok(())
}
