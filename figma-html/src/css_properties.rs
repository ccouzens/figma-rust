use std::cmp::Ordering;

use figma_schema::{Color, EffectType, Node, NodeType, Rectangle, TextCase};

use super::CSSVariablesMap;

/// Get values for given CSS properties
///
/// The CSS values are not optimized, but can be made so by use of another tool like `lightningcss`.
pub trait CssProperties {
    fn background(&self, css_variables: &mut CSSVariablesMap) -> Option<String>;
    fn box_shadow(&self) -> Option<String>;
    fn font(&self, css_variables: &mut CSSVariablesMap) -> Option<String>;
}

pub fn fills_color(node: &Node, css_variables: &mut CSSVariablesMap) -> Option<String> {
    let color_value = node
        .fills()
        .iter()
        .filter(|paint| paint.visible() && paint.opacity() != 0.0)
        .flat_map(|paint| paint.color())
        .flat_map(|c| c.to_option_rgb_string())
        .next()
        .or_else(|| {
            node.fills()
                .iter()
                .filter(|paint| paint.visible())
                .flat_map(|paint| paint.color())
                .map(|c| {
                    Color {
                        alpha: 0.0,
                        ..c.clone()
                    }
                    .to_rgb_string()
                })
                .next()
        })?;

    match node.styles.as_ref().and_then(|s| s.fill.as_deref()) {
        Some(s_ref) => match css_variables.get_mut(s_ref) {
            Some(v) => {
                v.value = Some(color_value);
                Some(format!("var({})", v.name))
            }
            None => Some(color_value),
        },
        None => Some(color_value),
    }
}

pub fn stroke_color(node: &Node) -> Option<String> {
    node.strokes()
        .iter()
        .filter(|p| p.visible() && p.opacity() != 0.0)
        .flat_map(|stroke| stroke.color())
        .flat_map(|color| color.to_option_rgb_string())
        .next()
}

pub fn absolute_bounding_box(node: &Node) -> Option<Rectangle> {
    if let Some(r) = node.absolute_bounding_box.clone() {
        return Some(r);
    }
    let bounding_boxes = || {
        node.enabled_children()
            .filter_map(|c| c.absolute_render_bounds.as_ref())
    };
    let min_x = bounding_boxes()
        .filter_map(|r| r.x)
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less))?;
    let min_y = bounding_boxes()
        .filter_map(|r| r.y)
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less))?;
    let max_x = bounding_boxes()
        .filter_map(|r| match (r.x, r.width) {
            (Some(x), Some(width)) => Some(x + width),
            _ => None,
        })
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less))?;
    let max_y = bounding_boxes()
        .filter_map(|r| match (r.y, r.height) {
            (Some(y), Some(height)) => Some(y + height),
            _ => None,
        })
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Less))?;
    Some(Rectangle {
        x: Some(min_x),
        y: Some(min_y),
        width: Some(max_x - min_x),
        height: Some(max_y - min_y),
    })
}

impl CssProperties for Node {
    fn background(&self, css_variables: &mut CSSVariablesMap) -> Option<String> {
        match self.r#type {
            NodeType::Text | NodeType::Vector | NodeType::BooleanOperation => None,
            _ => fills_color(self, css_variables).or_else(|| {
                self.background_color()
                    .and_then(|c| c.to_option_rgb_string())
            }),
        }
    }

    fn box_shadow(&self) -> Option<String> {
        let shadows = itertools::join(
            self.effects
                .as_deref()
                .unwrap_or(&[])
                .iter()
                .filter(|e| e.r#type == EffectType::InnerShadow)
                .filter(|e| e.visible)
                .filter_map(|e| {
                    let x_offset = e.offset.as_ref()?.x;
                    let y_offset = e.offset.as_ref()?.y;
                    let spread = e.spread();
                    let color = e.color.as_ref().and_then(|c| c.to_option_rgb_string())?;
                    Some(format!(
                        "inset {x_offset}px {y_offset}px {spread}px {color}"
                    ))
                }),
            ", ",
        );

        if shadows.is_empty() {
            None
        } else {
            Some(shadows)
        }
    }

    fn font(&self, css_variables: &mut CSSVariablesMap) -> Option<String> {
        let font_style = self.style.as_ref()?;

        let style = if matches!(font_style.italic, Some(true)) {
            "italic"
        } else {
            ""
        };
        let variant = if matches!(
            font_style.text_case,
            Some(TextCase::SmallCaps | TextCase::SmallCapsForced)
        ) {
            "small-caps"
        } else {
            ""
        };
        let weight = font_style.font_weight;
        let size = font_style.font_size;
        let line_height = font_style.line_height_px;
        let family = &font_style.font_family;

        let font_value = format!(r#"{style} {variant} {weight} {size}px/{line_height}px "{family}""#);

        match self.styles.as_ref().and_then(|s| s.text.as_deref()) {
            Some(s_ref) => match css_variables.get_mut(s_ref) {
                Some(v) => {
                    v.value = Some(font_value);
                    Some(format!("var({})", v.name))
                }
                None => Some(font_value),
            },
            None => Some(font_value),
        }
    }
}
