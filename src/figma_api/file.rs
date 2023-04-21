use super::{Component, Node, Style};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[typeshare::typeshare]
pub struct File {
    pub document: Node,
    #[typeshare(serialized_as = "std::collections::HashMap<String, Component>")]
    pub components: IndexMap<String, Component>,
    #[typeshare(serialized_as = "std::collections::HashMap<String, Style>")]
    pub styles: IndexMap<String, Style>,
    pub name: String,
    pub schema_version: u8,
    pub version: String,
}
