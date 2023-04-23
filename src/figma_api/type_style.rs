use serde::{Deserialize, Serialize};

/// Metadata for character formatting
///
/// [Figma documentation](https://www.figma.com/developers/api#typestyle-type)
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[typeshare::typeshare]
pub struct TypeStyle {
    /// Font family of text (standard name)
    pub font_family: String,
    pub font_weight: f64,
    pub font_size: f64,
    pub line_height_px: f64,
}
