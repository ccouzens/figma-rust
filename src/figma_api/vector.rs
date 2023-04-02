use serde::{Deserialize, Serialize};

/// [Figma documentation](https://www.figma.com/developers/api#vector-type)
#[derive(Debug, Deserialize, Serialize)]
#[typeshare::typeshare]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}
