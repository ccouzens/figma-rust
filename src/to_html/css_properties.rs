use crate::figma_api::{EffectType, Node, NodeType, StrokeAlign};

/// Get values for given CSS properties
///
/// The CSS values are not optimized, but can be made so by use of another tool like `lightningcss`.
pub trait CssProperties {
    fn background(&self) -> Option<String>;
    fn border_radius(&self) -> Option<String>;
    fn box_shadow(&self) -> Option<String>;
    fn color(&self) -> Option<String>;
    fn fill(&self) -> Option<String>;
    fn height(&self) -> Option<String>;
    fn line_height(&self) -> Option<String>;
    fn font_family(&self) -> Option<String>;
    fn font_size(&self) -> Option<String>;
    fn font_weight(&self) -> Option<String>;
    fn opacity(&self) -> Option<String>;
    fn outline(&self) -> Option<String>;
    fn outline_offset(&self) -> Option<String>;
    fn padding(&self) -> Option<String>;
    fn width(&self) -> Option<String>;
}

fn fills_color(node: &Node) -> Option<String> {
    node.fills()
        .iter()
        .filter(|paint| paint.visible() && paint.opacity() != 0.0)
        .flat_map(|paint| paint.color())
        .flat_map(|c| c.to_option_rgb_string())
        .next()
}

fn stroke_color(node: &Node) -> Option<String> {
    node.strokes()
        .iter()
        .filter(|p| p.visible())
        .flat_map(|stroke| stroke.color())
        .flat_map(|color| color.to_option_rgb_string())
        .next()
}

impl CssProperties for Node {
    fn background(&self) -> Option<String> {
        match self.r#type {
            NodeType::Text | NodeType::Vector => None,
            _ => fills_color(self).or_else(|| {
                self.background_color()
                    .and_then(|c| c.to_option_rgb_string())
            }),
        }
    }

    fn border_radius(&self) -> Option<String> {
        self.rectangle_corner_radii()
            .map(|[nw, ne, se, sw]| format!("{nw}px {ne}px {se}px {sw}px"))
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

    fn color(&self) -> Option<String> {
        match self.r#type {
            NodeType::Text => fills_color(self),
            _ => None,
        }
    }

    fn fill(&self) -> Option<String> {
        match self.r#type {
            NodeType::Vector => fills_color(self),
            _ => None,
        }
    }

    fn font_family(&self) -> Option<String> {
        self.style.as_ref().map(|s| s.font_family.clone())
    }

    fn font_size(&self) -> Option<String> {
        self.style
            .as_ref()
            .map(|s| s.font_size)
            .map(|fs| format!("{fs}px"))
    }

    fn font_weight(&self) -> Option<String> {
        self.style
            .as_ref()
            .map(|s| s.font_weight)
            .map(|fw| format!("{fw}"))
    }

    fn height(&self) -> Option<String> {
        match self.r#type {
            NodeType::Vector => self
                .absolute_bounding_box()
                .and_then(|b| b.height)
                .map(|h| format!("{h}px")),
            _ => None,
        }
    }

    fn line_height(&self) -> Option<String> {
        let lh = self.style.as_ref().map(|s| s.line_height_px).unwrap_or(0.0);
        Some(format!("{lh}px"))
    }

    fn padding(&self) -> Option<String> {
        let top = self.padding_top();
        let right = self.padding_right();
        let bottom = self.padding_bottom();
        let left = self.padding_left();
        if top == 0.0 && right == 0.0 && bottom == 0.0 && left == 0.0 {
            None
        } else {
            Some(format!("{top}px {right}px {bottom}px {left}px"))
        }
    }

    fn opacity(&self) -> Option<String> {
        let opacity = self.opacity();
        if opacity == 1.0 {
            None
        } else {
            Some(format!("{}", opacity))
        }
    }

    fn outline(&self) -> Option<String> {
        let color = stroke_color(self)?;
        let width = self.stroke_weight()?;
        if width == 0.0 {
            None
        } else {
            Some(format!("{width}px solid {color}"))
        }
    }

    fn outline_offset(&self) -> Option<String> {
        let stroke_align = self.stroke_align()?;
        self.outline()?;
        let width = self.stroke_weight()?;
        match stroke_align {
            StrokeAlign::Inside => Some(format!("-{width}px")),
            StrokeAlign::Outside => None,
            StrokeAlign::Center => Some(format!("-{}px", width / 2.0)),
        }
    }

    fn width(&self) -> Option<String> {
        match self.r#type {
            NodeType::Vector => self
                .absolute_bounding_box()
                .and_then(|b| b.width)
                .map(|w| format!("{w}px")),
            _ => None,
        }
    }
}
