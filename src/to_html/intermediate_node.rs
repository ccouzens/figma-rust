enum AlignSelf {
    Auto,
    Stretch,
}

enum Inset {
    Auto,
    /// To be used like so: calc(100% * dy / dx + c px)
    Linear {
        dy: f64,
        dx: f64,
        c: f64,
    },
}

pub struct Location {
    padding: [f64; 4],
    align_self: AlignSelf,
    inset: [Inset; 4],
}

pub struct BoxAppearance {
    background: Option<String>,
    border_radius: [f64; 4],
}

pub enum IntermediateNode {
    Vector,
    Text,
    Box { children: Vec<IntermediateNode> },
}
