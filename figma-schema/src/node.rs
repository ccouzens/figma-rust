use super::{Color, Component, EasingType, Effect, File, Paint, Rectangle, Styles, TypeStyle};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[typeshare::typeshare]
pub enum StrokeAlign {
    /// stroke drawn inside the shape boundary
    Inside,
    /// stroke drawn outside the shape boundary
    Outside,
    /// stroke drawn centered along the shape boundary
    Center,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[typeshare::typeshare]
pub enum LayoutMode {
    None,
    Horizontal,
    Vertical,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[typeshare::typeshare]
pub enum AxisSizingMode {
    Fixed,
    Auto,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[typeshare::typeshare]
pub enum PrimaryAxisAlignItems {
    Min,
    Center,
    Max,
    SpaceBetween,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[typeshare::typeshare]
pub enum CounterAxisAlignItems {
    Min,
    Center,
    Max,
    Baseline,
}

/// Individual stroke weights
#[derive(Debug, Deserialize, Serialize)]
#[typeshare::typeshare]
pub struct StrokeWeights {
    /// The top stroke weight
    pub top: f64,
    /// The right stroke weight
    pub right: f64,
    /// The bottom stroke weight
    pub bottom: f64,
    /// The left stroke weight
    pub left: f64,
}

/// [Figma documentation](https://www.figma.com/developers/api#node-types)
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[typeshare::typeshare]
pub struct Node {
    /// A string uniquely identifying this node within the document.
    pub id: String,
    /// The name given to the node by the user in the tool.
    pub name: String,
    /// Whether or not the node is visible on the canvas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<bool>,
    /// The type of the node
    pub r#type: NodeType,
    /// An array of nodes that are direct children of this node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<Node>>,
    /// Background color of the canvas
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<Color>,
    /// An array of fill paints applied to the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fills: Option<Vec<Paint>>,
    /// An array of stroke paints applied to the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strokes: Option<Vec<Paint>>,
    /// The weight of strokes on the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_weight: Option<f64>,
    /// An object including the top, bottom, left, and right stroke weights. Only returned if individual stroke weights are used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual_stroke_weights: Option<StrokeWeights>,
    /// Position of stroke relative to vector outline
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_align: Option<StrokeAlign>,
    /// An array of floating point numbers describing the pattern of dash length and gap lengths that the vector path follows. For example a value of [1, 2] indicates that the path has a dash of length 1 followed by a gap of length 2, repeated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_dashes: Option<Vec<f64>>,
    /// Radius of each corner of the node if a single radius is set for all corners
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_radius: Option<f64>,
    /// Array of length 4 of the radius of each corner of the node, starting in the top left and proceeding clockwise
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rectangle_corner_radii: Option<[f64; 4]>,
    /// The duration of the prototyping transition on this node (in milliseconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_duration: Option<f64>,
    /// The easing curve used in the prototyping transition on this node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_easing: Option<EasingType>,
    /// Opacity of the node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<f64>,
    /// Bounding box of the node in absolute space coordinates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_bounding_box: Option<Rectangle>,
    /// The bounds of the rendered node in the file in absolute space coordinates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_render_bounds: Option<Rectangle>,
    /// Whether the primary axis has a fixed length (determined by the user) or an automatic length (determined by the layout engine). This property is only applicable for auto-layout frames.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_axis_sizing_mode: Option<AxisSizingMode>,
    /// Whether the counter axis has a fixed length (determined by the user) or an automatic length (determined by the layout engine). This property is only applicable for auto-layout frames.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counter_axis_sizing_mode: Option<AxisSizingMode>,
    /// Determines how the auto-layout frame’s children should be aligned in the primary axis direction. This property is only applicable for auto-layout frames.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_axis_align_items: Option<PrimaryAxisAlignItems>,
    /// Determines how the auto-layout frame’s children should be aligned in the counter axis direction. This property is only applicable for auto-layout frames.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub counter_axis_align_items: Option<CounterAxisAlignItems>,
    /// The distance between children of the frame. Can be negative. This property is only applicable for auto-layout frames.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_spacing: Option<f64>,
    /// Whether this layer uses auto-layout to position its children.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout_mode: Option<LayoutMode>,
    /// The padding between the left border of the frame and its children. This property is only applicable for auto-layout frames.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_left: Option<f64>,
    /// The padding between the right border of the frame and its children. This property is only applicable for auto-layout frames.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_right: Option<f64>,
    /// The padding between the top border of the frame and its children. This property is only applicable for auto-layout frames.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_top: Option<f64>,
    /// The padding between the bottom border of the frame and its children. This property is only applicable for auto-layout frames.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_bottom: Option<f64>,
    /// An array of effects attached to this node
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effects: Option<Vec<Effect>>,
    /// A mapping of a StyleType to style ID of styles present on this node. The style ID can be used to look up more information about the style in the top-level styles field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub styles: Option<Styles>,
    /// Text contained within a text box
    #[serde(skip_serializing_if = "Option::is_none")]
    pub characters: Option<String>,
    /// Style of text including font family and weight
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<TypeStyle>,
    /// This property is applicable only for direct children of auto-layout frames, ignored otherwise. Determines whether a layer should stretch along the parent’s primary axis. A 0 corresponds to a fixed size and 1 corresponds to stretch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout_grow: Option<f64>,
}

impl Node {
    pub fn visible(&self) -> bool {
        self.visible.unwrap_or(true)
    }

    pub fn background_color(&self) -> Option<&Color> {
        self.background_color.as_ref()
    }

    pub fn absolute_bounding_box(&self) -> Option<&Rectangle> {
        self.absolute_bounding_box.as_ref()
    }

    pub fn corner_radius(&self) -> Option<f64> {
        self.corner_radius
    }

    pub fn rectangle_corner_radii(&self) -> Option<[f64; 4]> {
        self.rectangle_corner_radii
            .or_else(|| self.corner_radius.map(|r| [r, r, r, r]))
    }

    pub fn transition_duration(&self) -> Option<f64> {
        self.transition_duration
    }

    pub fn transition_easing(&self) -> Option<&EasingType> {
        self.transition_easing.as_ref()
    }

    pub fn opacity(&self) -> f64 {
        self.opacity.unwrap_or(1.0)
    }

    pub fn padding_left(&self) -> f64 {
        self.padding_left.unwrap_or(0.0)
    }

    pub fn padding_right(&self) -> f64 {
        self.padding_right.unwrap_or(0.0)
    }

    pub fn padding_top(&self) -> f64 {
        self.padding_top.unwrap_or(0.0)
    }

    pub fn padding_bottom(&self) -> f64 {
        self.padding_bottom.unwrap_or(0.0)
    }

    pub fn children(&self) -> &[Node] {
        self.children.as_deref().unwrap_or_default()
    }

    pub fn enabled_children(&self) -> impl Iterator<Item = &Node> {
        self.children().iter().filter(|c| c.visible())
    }

    pub fn fills(&self) -> &[Paint] {
        self.fills.as_deref().unwrap_or_default()
    }

    pub fn strokes(&self) -> &[Paint] {
        self.strokes.as_deref().unwrap_or_default()
    }

    pub fn depth_first_stack_iter(&self) -> NodeDepthFirstStackIterator {
        NodeDepthFirstStackIterator {
            stack: vec![self],
            iter_stack: vec![self.children().iter()],
        }
    }

    pub fn component<'a>(&self, file: &'a File) -> Option<&'a Component> {
        file.components.get(&self.id)
    }

    pub fn stroke_weight(&self) -> Option<f64> {
        self.stroke_weight
    }

    pub fn stroke_align(&self) -> Option<&StrokeAlign> {
        self.stroke_align.as_ref()
    }
}

/// Node type indicates what kind of node you are working with: for example, a FRAME node versus a RECTANGLE node. A node can have additional properties associated with it depending on its node type.
#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[typeshare::typeshare]
pub enum NodeType {
    Document,
    Canvas,
    Frame,
    Group,
    Vector,
    BooleanOperation,
    Star,
    Line,
    Ellipse,
    RegularPolygon,
    Rectangle,
    Text,
    Slice,
    Component,
    ComponentSet,
    Instance,
    Sticky,
    ShapeWithText,
    Connector,
    Section,
}

pub struct NodeDepthFirstStackIterator<'a> {
    iter_stack: Vec<std::slice::Iter<'a, Node>>,
    stack: Vec<&'a Node>,
}

impl<'a> Iterator for NodeDepthFirstStackIterator<'a> {
    type Item = (&'a Node, Vec<&'a Node>);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut bottom_of_iter_stack = self.iter_stack.pop()?;
            let bottom_of_stack = self.stack.pop()?;
            if let Some(current) = bottom_of_iter_stack.next() {
                self.iter_stack.push(bottom_of_iter_stack);
                self.iter_stack.push(current.children().iter());
                self.stack.push(bottom_of_stack);
                self.stack.push(current);
                return Some((current, self.stack.clone()));
            }
        }
    }
}
