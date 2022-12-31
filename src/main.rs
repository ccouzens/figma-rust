mod border_tokens;
mod breakpoint_tokens;
mod figma_api;
mod motion_tokens;
mod size_tokens;
mod spacing_tokens;
use std::{borrow::Cow, iter::once};

use indexmap::IndexMap;
use serde::Serialize;

fn node_match_prefix(prefixes: &[&str], name: &str) -> bool {
    let node_prefix = name.split('/').next().unwrap_or_default().trim();
    prefixes.contains(&node_prefix)
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum MapOrJson {
    Map(IndexMap<String, MapOrJson>),
    Json(serde_json::Value),
}

fn insert_by_name<'a>(output: &mut MapOrJson, name: &[&'a str], value: serde_json::Value) -> bool {
    match output {
        MapOrJson::Map(map) => {
            let head = name[0].trim().to_lowercase();
            let rest = &name[1..];
            if rest.is_empty() {
                match map.entry(head) {
                    indexmap::map::Entry::Occupied(_) => false,
                    indexmap::map::Entry::Vacant(vacancy) => {
                        vacancy.insert(MapOrJson::Json(value));
                        true
                    }
                }
            } else {
                let entry = map
                    .entry(head)
                    .or_insert_with(|| MapOrJson::Map(IndexMap::new()));
                insert_by_name(entry, rest, value)
            }
        }
        MapOrJson::Json(..) => false,
    }
}

fn token_transformer(
    file: &figma_api::File,
    output: &mut MapOrJson,
    prefixes: &[&str],
    transformer: impl Fn(&figma_api::Node, &figma_api::File) -> Option<serde_json::Value>,
) {
    for nodes in file.document.depth_first_stack_iter() {
        let node = *nodes.last().unwrap();

        let parent = nodes.iter().nth_back(1).cloned();
        let name = match parent {
            Some(figma_api::Node {
                name: parent_name,
                node_type: figma_api::NodeType::ComponentSet { .. },
                ..
            }) => Cow::Owned(
                once(parent_name.as_str())
                    .chain(
                        node.name
                            .split(',')
                            .map(str::trim)
                            .filter(|n| !n.starts_with('_') && !n.starts_with('.'))
                            .filter_map(|n| n.split('=').nth(1))
                            .map(str::trim),
                    )
                    .fold(String::new(), |mut acc, p| {
                        if !acc.is_empty() {
                            acc.push('/');
                        }
                        acc.push_str(p);
                        acc
                    }),
            ),
            _ => Cow::Borrowed(&node.name),
        };
        if !nodes
            .iter()
            .rev()
            .skip(1)
            .any(|n| n.name.split('/').next().map(str::trim) == Some("_tokens"))
        {
            continue;
        }
        if !node_match_prefix(prefixes, &name) {
            continue;
        }
        if let Some(json) = transformer(node, file) {
            if !insert_by_name(output, &name.split('/').collect::<Vec<_>>(), json) {
                eprintln!("Failed to insert {}", name);
            };
        }
    }
}

fn main() {
    let f: figma_api::File = serde_json::from_reader(std::io::stdin()).unwrap();

    let mut output = MapOrJson::Map(IndexMap::new());

    token_transformer(
        &f,
        &mut output,
        &["size", "sizes"],
        size_tokens::as_size_token,
    );
    token_transformer(&f, &mut output, &["breakpoints"], |node, _| {
        breakpoint_tokens::as_breakpoint_token(node, &f)
    });
    token_transformer(&f, &mut output, &["spacing"], |node, _| {
        spacing_tokens::as_spacing_token(node, &f)
    });
    token_transformer(&f, &mut output, &["borders", "border"], |node, _| {
        border_tokens::as_border_token(node, &f)
    });
    token_transformer(&f, &mut output, &["motion"], |node, _| {
        motion_tokens::as_motion_token(node)
    });

    println!(
        "{}",
        serde_json::to_string_pretty(&output).unwrap_or_else(|_| "Err".into())
    )
}
