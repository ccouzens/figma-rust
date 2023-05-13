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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[typeshare::typeshare]
pub enum TextAutoResize {
    Height,
    WidthAndHeight,
    /// The text will be shortened and trailing text will be replaced with "…" if the text contents is larger than the bounds
    Truncate,
}

/// Type of hyperlink
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[typeshare::typeshare]
pub enum HyperlinkType {
    Url,
    Node,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[typeshare::typeshare]
pub struct Hyperlink {
    pub r#type: Option<HyperlinkType>,
    /// URL being linked to, if URL type
    pub url: Option<String>,
    /// ID of frame hyperlink points to, if NODE type
    pub node_id: Option<String>,
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
    /// Space between paragraphs in px, 0 if not present
    pub paragraph_spacing: Option<f64>,
    /// Whether or not text is italicized
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    /// Numeric font weight
    pub font_weight: f64,
    /// Font size in px
    pub font_size: f64,
    /// Text casing applied to the node, default is the original casing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_case: Option<TextCase>,
    /// Text decoration applied to the node, default is none
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_decoration: Option<TextDecoration>,
    /// Dimensions along which text will auto resize, default is that the text does not auto-resize
    pub text_auto_resize: Option<TextAutoResize>,
    /// Link to a URL or frame
    pub hyperlink: Option<Hyperlink>,
    /// Line height in px
    pub line_height_px: f64,
}
