use crate::figma_api::Node;

/// Get values for given CSS properties
///
/// The CSS values are not optimized, but can be made so by use of another tool like `lightningcss`.
pub trait CssProperties {
    fn border_radius(&self) -> Option<String>;
}

impl CssProperties for Node {
    fn border_radius(&self) -> Option<String> {
        self.rectangle_corner_radii()
            .map(|[nw, ne, se, sw]| format!("{nw}px {ne}px {se}px {sw}px"))
    }
}
