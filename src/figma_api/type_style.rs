use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[typeshare::typeshare]
pub enum TextCase {
    Upper,
    Lower,
    Title,
    SmallCaps,
    SmallCapsForced,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[typeshare::typeshare]
pub enum TextDecoration {
    Strikethrough,
    Underline,
}

/// Metadata for character formatting
///
/// [Figma documentation](https://www.figma.com/developers/api#typestyle-type)
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[typeshare::typeshare]
pub struct TypeStyle {
    /// Font family of text (standard name)
    pub font_family: String,
    /// Numeric font weight
    pub font_weight: f64,
    /// Font size in px
    pub font_size: f64,
    /// Text casing applied to the node, default is the original casing
    pub text_case: Option<TextCase>,
    /// Text decoration applied to the node, default is none
    pub text_decoration: Option<TextDecoration>,
    /// Line height in px
    pub line_height_px: f64,
}
