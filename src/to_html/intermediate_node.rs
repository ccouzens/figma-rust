use figma_schema::{Node as FigmaNode, NodeType as FigmaNodeType, StrokeAlign};

pub type CSSVariablesMap<'a> = IndexMap<&'a str, CSSVariable>;

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

pub struct FrameAppearance {
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
    pub r#type: FigmaNodeType,
}

pub enum IntermediateNodeType<'a> {
    Vector,
    Text { text: &'a str },
    Frame { children: Vec<IntermediateNode<'a>> },
}

pub struct IntermediateNode<'a> {
    pub figma: Figma<'a>,
    pub flex_container: Option<FlexContainer>,
    pub location: Location,
    pub appearance: Appearance,
    pub frame_appearance: Option<FrameAppearance>,
    pub content_appearance: ContentAppearance,
    pub node_type: IntermediateNodeType<'a>,
}

impl<'a> IntermediateNode<'a> {
    fn from_figma_node(
        node: &'a FigmaNode,
        parent: Option<&'a FigmaNode>,
        css_variables: &mut CSSVariablesMap,
    ) -> Self {
        IntermediateNode {
            figma: Figma {
                name: &node.name,
                id: &node.id,
                r#type: node.r#type,
            },
            flex_container: _,
            location: _,
            appearance: _,
            frame_appearance: _,
            content_appearance: _,
            node_type: match node.r#type {
                FigmaNodeType::Vector | FigmaNodeType::BooleanOperation => {
                    IntermediateNodeType::Vector
                }
                FigmaNodeType::Text => IntermediateNodeType::Text {
                    text: node.characters.as_deref().unwrap_or(""),
                },
                _ => IntermediateNodeType::Frame {
                    children: node
                        .enabled_children()
                        .map(|child| Self::from_figma_node(child, Some(node), css_variables))
                        .collect(),
                },
            },
        }
    }
}
