use std::cmp::Ordering;

use figma_schema::{
    AxisSizingMode, CounterAxisAlignItems, EffectType, LayoutAlign, LayoutConstraint,
    LayoutConstraintHorizontal, LayoutConstraintVertical, LayoutMode, LayoutPositioning, Node,
    NodeType, PrimaryAxisAlignItems, Rectangle, StrokeAlign, TextAutoResize, TextCase,
    TextDecoration, TypeStyle,
};

use super::CSSVariablesMap;

/// Get values for given CSS properties
///
/// The CSS values are not optimized, but can be made so by use of another tool like `lightningcss`.
pub trait CssProperties {
    fn align_items(&self) -> Option<String>;
    fn align_self(&self, parent: Option<&Node>) -> Option<String>;
    fn background(&self, css_variables: &mut CSSVariablesMap) -> Option<String>;
    fn border_radius(&self) -> Option<String>;
    fn bottom(&self, parent: Option<&Node>) -> Option<String>;
    fn box_shadow(&self) -> Option<String>;
    fn box_sizing(&self, parent: Option<&Node>) -> Option<String>;
    fn color(&self, css_variables: &mut CSSVariablesMap) -> Option<String>;
    fn display(&self) -> Option<String>;
    fn fill(&self, css_variables: &mut CSSVariablesMap) -> Option<String>;
    fn flex_direction(&self) -> Option<String>;
    fn flex_grow(&self) -> Option<String>;
    fn font(&self, css_variables: &mut CSSVariablesMap) -> Option<String>;
    fn gap(&self) -> Option<String>;
    fn height(&self, parent: Option<&Node>) -> Option<String>;
    fn justify_content(&self) -> Option<String>;
    fn left(&self, parent: Option<&Node>) -> Option<String>;
    fn opacity(&self) -> Option<String>;
    fn outline_offset(&self) -> Option<String>;
    fn outline(&self) -> Option<String>;
    fn padding(&self) -> Option<String>;
    fn position(&self, parent: Option<&Node>) -> Option<String>;
    fn right(&self, parent: Option<&Node>) -> Option<String>;
    fn text_decoration_line(&self) -> Option<String>;
    fn text_transform(&self) -> Option<String>;
    fn top(&self, parent: Option<&Node>) -> Option<String>;
    fn white_space(&self) -> Option<String>;
    fn width(&self, parent: Option<&Node>) -> Option<String>;
}

fn is_auto_layout(node: &Node) -> bool {
    matches!(
        node.layout_mode,
        Some(LayoutMode::Horizontal) | Some(LayoutMode::Vertical)
    )
}

fn fills_color(node: &Node, css_variables: &mut CSSVariablesMap) -> Option<String> {
    let color_value = node
        .fills()
        .iter()
        .filter(|paint| paint.visible() && paint.opacity() != 0.0)
        .flat_map(|paint| paint.color())
        .flat_map(|c| c.to_option_rgb_string())
        .next()?;

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

fn stroke_color(node: &Node) -> Option<String> {
    node.strokes()
        .iter()
        .filter(|p| p.visible() && p.opacity() != 0.0)
        .flat_map(|stroke| stroke.color())
        .flat_map(|color| color.to_option_rgb_string())
        .next()
}

fn absolute_bounding_box(node: &Node) -> Option<Rectangle> {
    if let Some(r) = node.absolute_bounding_box.clone() {
        return Some(r);
    }
    let bounding_boxes = || {
        node.enabled_children()
            .filter_map(|c| c.absolute_bounding_box.as_ref())
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
    fn align_items(self: &Node) -> Option<String> {
        match self.counter_axis_align_items {
            None => None,
            Some(CounterAxisAlignItems::Min) => Some("flex-start".into()),
            Some(CounterAxisAlignItems::Center) => Some("center".into()),
            Some(CounterAxisAlignItems::Max) => Some("flex-end".into()),
            Some(CounterAxisAlignItems::Baseline) => Some("baseline".into()),
        }
    }

    fn align_self(self: &Node, parent: Option<&Node>) -> Option<String> {
        if parent.map(is_auto_layout) != Some(true) {
            return None;
        }
        match self.layout_align {
            Some(LayoutAlign::Stretch) => Some("stretch".into()),
            _ => None,
        }
    }

    fn background(&self, css_variables: &mut CSSVariablesMap) -> Option<String> {
        match self.r#type {
            NodeType::Text | NodeType::Vector | NodeType::BooleanOperation => None,
            _ => fills_color(self, css_variables).or_else(|| {
                self.background_color()
                    .and_then(|c| c.to_option_rgb_string())
            }),
        }
    }

    fn border_radius(&self) -> Option<String> {
        self.rectangle_corner_radii()
            .map(|[nw, ne, se, sw]| format!("{nw}px {ne}px {se}px {sw}px"))
    }

    fn bottom(&self, parent: Option<&Node>) -> Option<String> {
        let parent = parent?;
        if is_auto_layout(parent)
            && !matches!(self.layout_positioning, Some(LayoutPositioning::Absolute))
        {
            return None;
        }
        let parent_rectangle = absolute_bounding_box(parent)?;
        let self_rectangle = absolute_bounding_box(self)?;
        let parent_height = parent_rectangle.height?;
        let bottom =
            parent_rectangle.y? + parent_height - self_rectangle.y? - self_rectangle.height?;
        match self.constraints.as_ref()?.vertical {
            LayoutConstraintVertical::Top => None,
            LayoutConstraintVertical::Bottom | LayoutConstraintVertical::TopBottom => {
                Some(format!("{}px", -bottom))
            }
            LayoutConstraintVertical::Center => None,
            LayoutConstraintVertical::Scale => {
                Some(format!("calc(100% * {bottom}/{parent_height})"))
            }
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

    fn box_sizing(&self, parent: Option<&Node>) -> Option<String> {
        if self.padding().is_some()
            && (self.width(parent).is_some() || self.height(parent).is_some())
        {
            Some("border-box".into())
        } else {
            None
        }
    }

    fn color(&self, css_variables: &mut CSSVariablesMap) -> Option<String> {
        match self.r#type {
            NodeType::Text => fills_color(self, css_variables),
            _ => None,
        }
    }

    fn display(&self) -> Option<String> {
        if is_auto_layout(self) || self.characters.as_deref().unwrap_or("").contains('\n') {
            Some("flex".into())
        } else {
            None
        }
    }

    fn flex_direction(&self) -> Option<String> {
        if self.characters.as_deref().unwrap_or("").contains('\n') {
            return Some("column".into());
        }
        match self.layout_mode {
            Some(LayoutMode::Horizontal) => Some("row".into()),
            Some(LayoutMode::Vertical) => Some("column".into()),
            _ => None,
        }
    }

    fn flex_grow(&self) -> Option<String> {
        let grow = self.layout_grow?;
        if grow != 0.0 {
            Some(format!("{grow}"))
        } else {
            None
        }
    }

    fn fill(&self, css_variables: &mut CSSVariablesMap) -> Option<String> {
        match self.r#type {
            NodeType::Vector | NodeType::BooleanOperation => fills_color(self, css_variables),
            _ => None,
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

        let font_value = format!("{style} {variant} {weight} {size}px/{line_height}px {family}");

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

    fn gap(&self) -> Option<String> {
        let item_spacing = self
            .item_spacing
            .or_else(|| self.style.as_ref().and_then(|s| s.paragraph_spacing))?;
        if item_spacing == 0.0 {
            None
        } else {
            Some(format!("{item_spacing}px"))
        }
    }

    fn height(&self, parent: Option<&Node>) -> Option<String> {
        if matches!(
            parent,
            Some(Node {
                layout_mode: Some(LayoutMode::Horizontal),
                ..
            })
        ) && matches!(self.layout_align, Some(LayoutAlign::Stretch))
        {
            return None;
        }
        if matches!(
            parent,
            Some(Node {
                layout_mode: Some(LayoutMode::Vertical),
                ..
            })
        ) && self.layout_grow == Some(1.0)
        {
            return None;
        }
        if matches!(self.layout_mode, Some(LayoutMode::Vertical))
            && !matches!(self.primary_axis_sizing_mode, Some(AxisSizingMode::Fixed))
        {
            return None;
        }
        if matches!(self.layout_mode, Some(LayoutMode::Horizontal))
            && !matches!(self.counter_axis_sizing_mode, Some(AxisSizingMode::Fixed))
        {
            return None;
        }
        if self.characters.is_some() {
            return None;
        }
        if matches!(
            self.constraints,
            Some(LayoutConstraint {
                vertical: LayoutConstraintVertical::TopBottom,
                ..
            })
        ) {
            return None;
        }
        absolute_bounding_box(self)
            .and_then(|b| b.height)
            .map(|h| format!("{h}px"))
    }

    fn justify_content(&self) -> Option<String> {
        match self.primary_axis_align_items {
            None => None,
            Some(PrimaryAxisAlignItems::Min) => Some("flex-start".into()),
            Some(PrimaryAxisAlignItems::Center) => Some("center".into()),
            Some(PrimaryAxisAlignItems::Max) => Some("flex-end".into()),
            Some(PrimaryAxisAlignItems::SpaceBetween) => Some("space-between".into()),
        }
    }

    fn left(&self, parent: Option<&Node>) -> Option<String> {
        let parent = parent?;
        if is_auto_layout(parent)
            && !matches!(self.layout_positioning, Some(LayoutPositioning::Absolute))
        {
            return None;
        }
        let parent_rectangle = absolute_bounding_box(parent)?;
        let left = absolute_bounding_box(self)?.x? - parent_rectangle.x?;
        let parent_width = parent_rectangle.width?;
        match self.constraints.as_ref()?.horizontal {
            LayoutConstraintHorizontal::Left | LayoutConstraintHorizontal::LeftRight => {
                Some(format!("{left}px"))
            }
            LayoutConstraintHorizontal::Right => None,
            LayoutConstraintHorizontal::Center => {
                Some(format!("calc(50% + {}px)", left - parent_width / 2.0))
            }
            LayoutConstraintHorizontal::Scale => {
                Some(format!("calc(100% * {left}/{parent_width})"))
            }
        }
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

    fn position(&self, parent: Option<&Node>) -> Option<String> {
        if let Some(parent) = parent {
            if !is_auto_layout(parent)
                || matches!(self.layout_positioning, Some(LayoutPositioning::Absolute))
            {
                return Some("absolute".into());
            }
        }
        if !is_auto_layout(self) && self.enabled_children().next().is_some()
            || self
                .enabled_children()
                .any(|n| matches!(n.layout_positioning, Some(LayoutPositioning::Absolute)))
        {
            Some("relative".into())
        } else {
            None
        }
    }

    fn opacity(&self) -> Option<String> {
        if matches!(self.r#type, NodeType::Vector | NodeType::BooleanOperation)
            && !self.fills().is_empty()
            && self.fills().iter().all(|f| f.opacity() == 0.0)
        {
            return Some("0".into());
        }

        let opacity = self.opacity?;
        if opacity != 1.0 {
            Some(format!("{opacity}"))
        } else {
            None
        }
    }

    fn outline(&self) -> Option<String> {
        let color = stroke_color(self)?;
        let style = if self.stroke_dashes.as_ref().map(|sd| sd.is_empty()) == Some(false) {
            "dashed"
        } else {
            "solid"
        };
        let width = self.stroke_weight()?;
        if width == 0.0 {
            None
        } else {
            Some(format!("{width}px {style} {color}"))
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

    fn right(&self, parent: Option<&Node>) -> Option<String> {
        let parent = parent?;
        if is_auto_layout(parent)
            && !matches!(self.layout_positioning, Some(LayoutPositioning::Absolute))
        {
            return None;
        }
        let parent_rectangle = absolute_bounding_box(parent)?;
        let self_rectangle = absolute_bounding_box(self)?;
        let parent_width = parent_rectangle.width?;
        let right = parent_rectangle.x? + parent_width - self_rectangle.x? - self_rectangle.width?;
        match self.constraints.as_ref()?.horizontal {
            LayoutConstraintHorizontal::Left => None,
            LayoutConstraintHorizontal::Right | LayoutConstraintHorizontal::LeftRight => {
                Some(format!("{}px", -right))
            }
            LayoutConstraintHorizontal::Center => None,
            LayoutConstraintHorizontal::Scale => {
                Some(format!("calc(100% * {right}/{parent_width})"))
            }
        }
    }

    fn text_decoration_line(&self) -> Option<String> {
        match self.style.as_ref()?.text_decoration.as_ref()? {
            TextDecoration::Strikethrough => Some("line-through".into()),
            TextDecoration::Underline => Some("underline".into()),
        }
    }

    fn text_transform(&self) -> Option<String> {
        match self.style.as_ref()?.text_case.as_ref()? {
            TextCase::Upper => Some("uppercase".into()),
            TextCase::Lower => Some("lowercase".into()),
            TextCase::Title => Some("capitalize".into()),
            TextCase::SmallCaps => None,
            TextCase::SmallCapsForced => None,
        }
    }

    fn top(&self, parent: Option<&Node>) -> Option<String> {
        let parent = parent?;
        if is_auto_layout(parent)
            && !matches!(self.layout_positioning, Some(LayoutPositioning::Absolute))
        {
            return None;
        }
        let parent_rectangle = absolute_bounding_box(parent)?;
        let top = absolute_bounding_box(self)?.y? - parent_rectangle.y?;
        let parent_height = parent_rectangle.height?;
        match self.constraints.as_ref()?.vertical {
            LayoutConstraintVertical::Top | LayoutConstraintVertical::TopBottom => {
                Some(format!("{top}px"))
            }
            LayoutConstraintVertical::Bottom => None,
            LayoutConstraintVertical::Center => {
                Some(format!("calc(50% + {}px)", top - parent_height / 2.0))
            }
            LayoutConstraintVertical::Scale => Some(format!("calc(100% * {top}/{parent_height})")),
        }
    }

    fn white_space(&self) -> Option<String> {
        let characters = self.characters.as_deref()?;
        // If any line includes repeated, leading or trailing whitespace then we should preserve it
        if characters.split('\n').any(|line| {
            let mut last_char_was_whitespace = true;
            for c in line.chars() {
                if c.is_ascii_whitespace() {
                    if last_char_was_whitespace {
                        return true;
                    }
                    last_char_was_whitespace = true;
                } else {
                    last_char_was_whitespace = false;
                }
            }
            return last_char_was_whitespace;
        }) {
            Some("pre-wrap".into())
        } else {
            None
        }
    }

    fn width(&self, parent: Option<&Node>) -> Option<String> {
        if matches!(
            parent,
            Some(Node {
                layout_mode: Some(LayoutMode::Vertical),
                ..
            })
        ) && matches!(self.layout_align, Some(LayoutAlign::Stretch))
        {
            return None;
        }
        if matches!(
            parent,
            Some(Node {
                layout_mode: Some(LayoutMode::Horizontal),
                ..
            })
        ) && self.layout_grow == Some(1.0)
        {
            return None;
        }
        if matches!(self.layout_mode, Some(LayoutMode::Horizontal))
            && !matches!(self.primary_axis_sizing_mode, Some(AxisSizingMode::Fixed))
        {
            return None;
        }
        if matches!(self.layout_mode, Some(LayoutMode::Vertical))
            && !matches!(self.counter_axis_sizing_mode, Some(AxisSizingMode::Fixed))
        {
            return None;
        }
        if matches!(
            self,
            Node {
                style: Some(TypeStyle {
                    text_auto_resize: Some(TextAutoResize::WidthAndHeight),
                    ..
                }),
                ..
            }
        ) {
            return None;
        }
        if matches!(
            self.constraints,
            Some(LayoutConstraint {
                horizontal: LayoutConstraintHorizontal::LeftRight,
                ..
            })
        ) {
            return None;
        }
        absolute_bounding_box(self)
            .and_then(|b| b.width)
            .map(|w| format!("{w}px"))
    }
}
