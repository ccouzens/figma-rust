use figma_schema::NodeType;

pub enum AlignItems {
    Normal,
    FlexStart,
    Center,
    FlexEnd,
    Baseline,
}

pub enum AlignSelf {
    Auto,
    Stretch,
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

pub struct FlexContainer {
    pub align_items: AlignItems,
}

pub struct Location {
    pub padding: [f64; 4],
    pub align_self: AlignSelf,
    pub inset: [Inset; 4],
}

pub struct BoxAppearance {
    pub background: Option<String>,
    pub border_radius: [f64; 4],
}

pub struct ContentAppearance {
    pub color: Option<String>,
    pub fill: Option<String>,
}

pub struct Figma<'a> {
    pub name: &'a str,
    pub id: &'a str,
    pub r#type: NodeType,
}

pub enum IntermediateNode {
    Vector,
    Text,
    Box { children: Vec<IntermediateNode> },
}
