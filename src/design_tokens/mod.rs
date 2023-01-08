mod border_tokens;
mod breakpoint_tokens;
mod motion_tokens;
mod opacity_tokens;
mod radius_tokens;
mod size_tokens;
mod spacing_tokens;
use super::figma_api;
use anyhow::{Context, Result};
use indexmap::IndexMap;
use serde::Serialize;
use serde_json::json;
use std::{borrow::Cow, io::Write, iter::once};

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

fn insert_by_name(output: &mut MapOrJson, name: &[&str], value: serde_json::Value) -> bool {
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

fn token_document_transformer(
    file: &figma_api::File,
    output: &mut MapOrJson,
    prefixes: &[&str],
    stderr: &mut impl Write,
    transformer: impl Fn(&figma_api::Node, &figma_api::File) -> Option<serde_json::Value>,
) {
    for (node, nodes) in file.document.depth_first_stack_iter() {
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
                writeln!(stderr, "Failed to insert {}", name).unwrap();
            };
        }
    }
}

fn token_style_transformer(
    f: &figma_api::File,
    output: &mut MapOrJson,
    name: &str,
    stderr: &mut impl Write,
    style_type: figma_api::StyleType,
) {
    for style in f.styles.values() {
        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        struct ExportToken<'a> {
            category: &'a str,
            export_key: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            comment: Option<&'a str>,
        }

        if style.style_type == style_type
            && !style
                .name
                .trim_start()
                .starts_with(|c| c == '.' || c == '_' || c == '*')
            && !insert_by_name(
                output,
                &once(name).chain(style.name.split('/')).collect::<Vec<_>>(),
                json!(ExportToken {
                    category: name,
                    export_key: name,
                    comment: if style.description.is_empty() {
                        None
                    } else {
                        Some(&style.description)
                    },
                }),
            )
        {
            writeln!(stderr, "Failed to insert {}", name).unwrap();
        };
    }
}

pub fn main(
    file: &figma_api::File,
    stdout: &mut impl Write,
    stderr: &mut impl Write,
) -> Result<()> {
    let mut output = MapOrJson::Map(IndexMap::new());

    token_document_transformer(
        file,
        &mut output,
        &["size", "sizes"],
        stderr,
        size_tokens::as_size_token,
    );
    token_document_transformer(file, &mut output, &["breakpoints"], stderr, |node, _| {
        breakpoint_tokens::as_breakpoint_token(node, file)
    });
    token_document_transformer(file, &mut output, &["spacing"], stderr, |node, _| {
        spacing_tokens::as_spacing_token(node, file)
    });
    token_document_transformer(
        file,
        &mut output,
        &["borders", "border"],
        stderr,
        |node, _| border_tokens::as_border_token(node, file),
    );
    token_document_transformer(
        file,
        &mut output,
        &["radius", "radii"],
        stderr,
        |node, _| radius_tokens::as_radius_token(node, file),
    );
    token_document_transformer(file, &mut output, &["motion"], stderr, |node, _| {
        motion_tokens::as_motion_token(node)
    });
    token_document_transformer(
        file,
        &mut output,
        &["opacities", "opacity"],
        stderr,
        |node, _| opacity_tokens::as_opacity_token(node, file),
    );
    token_style_transformer(
        file,
        &mut output,
        "color",
        stderr,
        figma_api::StyleType::Fill,
    );
    token_style_transformer(
        file,
        &mut output,
        "grid",
        stderr,
        figma_api::StyleType::Grid,
    );
    token_style_transformer(
        file,
        &mut output,
        "font",
        stderr,
        figma_api::StyleType::Text,
    );
    token_style_transformer(
        file,
        &mut output,
        "effect",
        stderr,
        figma_api::StyleType::Effect,
    );

    serde_json::to_writer_pretty(stdout, &output).context("Failed to write design tokens")?;
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
                "../../example-figma-files/design-tokens-for-figma.json"
            ))
            .unwrap(),
            &mut stdout,
            &mut stderr,
        )
        .unwrap();
        // Don't use assert_eq! as the output is too long to sensibly read
        assert!(String::from_utf8_lossy(&stdout) == include_str!("./example-output.json"));
    }

    #[test]
    fn expected_stderr() {
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        main(
            &serde_json::from_str(include_str!(
                "../../example-figma-files/design-tokens-for-figma.json"
            ))
            .unwrap(),
            &mut stdout,
            &mut stderr,
        )
        .unwrap();
        assert_eq!(
            String::from_utf8_lossy(&stderr),
            r#"Failed to insert sizes/in variant 60
Failed to insert sizes/in variant 90
Failed to insert sizes/in variant 120
Failed to insert sizes/40
Failed to insert sizes/60
Failed to insert sizes/80
"#
        );
    }
}
