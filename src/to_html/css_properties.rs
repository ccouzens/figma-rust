use crate::figma_api::{Node, NodeType};

/// Get values for given CSS properties
///
/// The CSS values are not optimized, but can be made so by use of another tool like `lightningcss`.
pub trait CssProperties {
    fn border_radius(&self) -> Option<String>;
    fn background(&self) -> Option<String>;
    fn color(&self) -> Option<String>;
    fn line_height(&self) -> Option<String>;
    fn font_family(&self) -> Option<String>;
    fn font_size(&self) -> Option<String>;
    fn font_weight(&self) -> Option<String>;
    fn opacity(&self) -> Option<String>;
    fn padding(&self) -> Option<String>;
}

fn fills_color(node: &Node) -> Option<String> {
    node.fills()
        .iter()
        .filter(|paint| paint.visible() && paint.opacity() != 0.0)
        .flat_map(|paint| paint.color())
        .flat_map(|c| c.to_option_rgb_string())
        .next()
}

impl CssProperties for Node {
    fn border_radius(&self) -> Option<String> {
        self.rectangle_corner_radii()
            .map(|[nw, ne, se, sw]| format!("{nw}px {ne}px {se}px {sw}px"))
    }

    fn background(&self) -> Option<String> {
        if self.r#type == NodeType::Text {
            return None;
        }
        fills_color(self).or_else(|| {
            self.background_color()
                .and_then(|c| c.to_option_rgb_string())
        })
    }

    fn color(&self) -> Option<String> {
        if self.r#type != NodeType::Text {
            return None;
        }
        fills_color(self)
    }

    fn line_height(&self) -> Option<String> {
        self.style
            .as_ref()
            .map(|s| s.line_height_px)
            .map(|lh| format!("{lh}px"))
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
}
