use figma_schema::{NodeType, StrokeAlign};

pub enum AlignItems {
    FlexStart,
    Center,
    FlexEnd,
    Baseline,
}

pub enum AlignSelf {
    Stretch,
}

pub enum FlexDirection {
    Row,
    Column,
}

pub enum Inset {
    Auto,
    /// To be used like so: calc(100% * dy / dx + c px)
    Linear {
        dy: f64,
        dx: f64,
        c: f64,
    },
}

pub enum JustifyContent {
    FlexStart,
    Center,
    FlexEnd,
    SpaceBetween,
}

pub enum StrokeStyle {
    Solid,
    Dashed,
}

pub struct FlexContainer {
    pub align_items: AlignItems,
    pub direction: FlexDirection,
    pub gap: f64,
    pub justify_content: Option<JustifyContent>,
}

pub struct Location {
    pub padding: [f64; 4],
    pub align_self: Option<AlignSelf>,
    pub flex_grow: Option<f64>,
    pub inset: Option<[Inset; 4]>,
    pub height: Option<f64>,
    pub width: Option<f64>,
}

pub struct Appearance {
    pub opacity: Option<f64>,
}

pub struct BoxAppearance {
    pub background: Option<String>,
    pub border_radius: Option<[f64; 4]>,
    pub box_shadow: Option<String>,
    pub stroke: Option<Stroke>,
}

pub struct Stroke {
    pub weights: [f64; 4],
    pub style: StrokeStyle,
    pub offset: StrokeAlign,
}

pub struct ContentAppearance {
    pub color: Option<String>,
    pub fill: Option<String>,
    pub font: Option<String>,
}

pub struct Figma<'a> {
    pub name: &'a str,
    pub id: &'a str,
    pub r#type: NodeType,
}

pub enum IntermediateNodeType<'a> {
    Vector,
    Text { text: &'a str },
    Box { children: Vec<IntermediateNode<'a>> },
}

pub struct IntermediateNode<'a> {
    pub figma: Figma<'a>,
    pub flex_container: Option<FlexContainer>,
    pub location: Location,
    pub appearance: Appearance,
    pub box_appearance: Option<BoxAppearance>,
    pub content_appearance: ContentAppearance,
    pub node_type: IntermediateNodeType<'a>,
}
