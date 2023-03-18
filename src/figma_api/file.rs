use super::{Component, Node, Style};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub document: Node,
    pub components: IndexMap<String, Component>,
    pub styles: IndexMap<String, Style>,
    pub name: String,
    pub schema_version: u8,
    pub version: String,
}
