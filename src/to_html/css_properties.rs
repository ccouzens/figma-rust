use crate::figma_api::Node;

/// Get values for given CSS properties
///
/// The CSS values are not optimized, but can be made so by use of another tool like `lightningcss`.
pub trait CssProperties {
    fn border_radius(&self) -> Option<String>;
    fn background(&self) -> Option<String>;
    fn padding(&self) -> Option<String>;
    fn opacity(&self) -> Option<String>;
}

impl CssProperties for Node {
    fn border_radius(&self) -> Option<String> {
        self.rectangle_corner_radii()
            .map(|[nw, ne, se, sw]| format!("{nw}px {ne}px {se}px {sw}px"))
    }

    fn background(&self) -> Option<String> {
        self.fills()
            .iter()
            .filter(|paint| {
                paint.visible()
                    && paint.opacity() != 0.0
                    && paint.color().map(|p| p.alpha != 0.0).unwrap_or(false)
            })
            .flat_map(|paint| paint.color())
            .next()
            .or_else(|| self.background_color())
            .map(|color| color.to_rgb_string())
    }

    fn padding(&self) -> Option<String> {
        Some(format!(
            "{}px {}px {}px {}px",
            self.padding_top(),
            self.padding_right(),
            self.padding_bottom(),
            self.padding_left()
        ))
    }

    fn opacity(&self) -> Option<String> {
        Some(format!("{}", self.opacity()))
    }
}
