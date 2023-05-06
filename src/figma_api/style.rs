use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[typeshare::typeshare]
pub enum StyleType {
    Fill,
    Text,
    Effect,
    Grid,
}

/// [Figma documentation](https://www.figma.com/developers/api#style-type)
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[typeshare::typeshare]
pub struct Style {
    pub key: String,
    pub name: String,
    pub description: String,
    pub remote: bool,
    pub style_type: StyleType,
}
