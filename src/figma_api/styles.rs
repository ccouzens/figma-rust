use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[typeshare::typeshare]
pub struct Styles {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stroke: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    effect: Option<String>,
}
