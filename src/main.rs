mod figma_api;
mod motion_tokens;
mod size_tokens;
use indexmap::{IndexMap};
use serde::Serialize;

fn node_match_prefix(prefixes: &[&str], node: &figma_api::Node) -> bool {
    let node_prefix = node.name.split('/').next().unwrap_or_default().trim();
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

fn main() {
    let f: figma_api::File = serde_json::from_reader(std::io::stdin()).unwrap();

    let mut output = MapOrJson::Map(IndexMap::new());

    for c in f.document.depth_first_iter() {
        if let Some(json) =
            size_tokens::as_size_token(c)
        {
            if !insert_by_name(&mut output, &c.name.split('/').collect::<Vec<_>>(), json) {
                eprintln!("Failed to insert {}", &c.name);
            };
        }
    }
    for c in f.document.depth_first_iter() {
        if let Some(json) =
            motion_tokens::as_motion_token(c)
        {
            if !insert_by_name(&mut output, &c.name.split('/').collect::<Vec<_>>(), json) {
                eprintln!("Failed to insert {}", &c.name);
            };
        }
    }

    println!(
        "{}",
        serde_json::to_string_pretty(&output).unwrap_or_else(|_| "Err".into())
    )
}
