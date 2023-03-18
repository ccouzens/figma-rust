use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StyleType {
    Fill,
    Text,
    Effect,
    Grid,
}

/// [Figma documentation](https://www.figma.com/developers/api#style-type)
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Style {
    pub key: String,
    pub name: String,
    pub description: String,
    pub remote: bool,
    pub style_type: StyleType,
}
