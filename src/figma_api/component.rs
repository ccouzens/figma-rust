use serde::{Deserialize, Serialize};

/// [Figma documentation](https://www.figma.com/developers/api#component-type)
#[derive(Debug, Deserialize, Serialize)]
pub struct Component {
    pub key: String,
    pub name: String,
    pub description: String,
}
