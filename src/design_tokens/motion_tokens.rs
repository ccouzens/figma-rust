use serde_json::json;

use crate::figma_api::{self, Node};

pub fn as_motion_token(node: &Node) -> Option<serde_json::Value> {
    let duration = node.transition_duration()?;
    let easing = node.transition_easing()?;
    Some(json!({
        "category": "motion",
        "exportKey": "motion",
        "type": {
            "value": "scroll_animate",
            "type": "string"
        },
        "duration": {
            "value": duration / 1000.0,
            "type": "number",
            "unit": "s"
        },
        "easing": {
            "value": match easing {
                figma_api::EasingType::Linear => "linear",
                figma_api::EasingType::EaseIn => "ease-in",
                figma_api::EasingType::EaseOut => "ease-out",
                figma_api::EasingType::EaseInAndOut => "ease-in-out",
                figma_api::EasingType::EaseInBack => "ease-in-back",
                figma_api::EasingType::EaseOutBack => "ease-out-back",
                figma_api::EasingType::EaseInAndOutBack => "ease-in-out-back",
                figma_api::EasingType::CustomBezier => "custom-cubic-bezier",
                figma_api::EasingType::Gentle => "gentle",
                figma_api::EasingType::Quick => "quick",
                figma_api::EasingType::Bouncy => "bouncy",
                figma_api::EasingType::Slow => "slow",
                figma_api::EasingType::CustomSpring => "custom-spring",
            },
            "type": "string"
        },
        "easingFunction": match easing {
            figma_api::EasingType::Linear => json!({
                "x1": { "value": 0.0, "type": "number" },
                "x2": { "value": 1.0, "type": "number" },
                "y1": { "value": 0.0, "type": "number" },
                "y2": { "value": 1.0, "type": "number" }
            }),
            figma_api::EasingType::EaseIn => json!({
                "x1": { "value": 0.42, "type": "number" },
                "x2": { "value": 1.0, "type": "number" },
                "y1": { "value": 0.0, "type": "number" },
                "y2": { "value": 1.0, "type": "number" }
            }),
            figma_api::EasingType::EaseOut => json!({
                "x1": { "value": 0.0, "type": "number" },
                "x2": { "value": 0.58, "type": "number" },
                "y1": { "value": 0.0, "type": "number" },
                "y2": { "value": 1.0, "type": "number" }
            }),
            figma_api::EasingType::EaseInAndOut => json!({
                "x1": { "value": 0.42, "type": "number" },
                "x2": { "value": 0.58, "type": "number" },
                "y1": { "value": 0.0, "type": "number" },
                "y2": { "value": 1.0, "type": "number" }
            }),
            figma_api::EasingType::EaseInBack => json!({
                "x1": { "value": 0.3, "type": "number" },
                "x2": { "value": 0.7, "type": "number" },
                "y1": { "value": -0.05, "type": "number" },
                "y2": { "value": -0.5, "type": "number" }
            }),
            figma_api::EasingType::EaseOutBack => json!({
                "x1": { "value": 0.45, "type": "number" },
                "x2": { "value": 0.8, "type": "number" },
                "y1": { "value": 1.45, "type": "number" },
                "y2": { "value": 1.0, "type": "number" }
            }),
            figma_api::EasingType::EaseInAndOutBack => json!({
                "x1": { "value": 0.7, "type": "number" },
                "x2": { "value": 0.4, "type": "number" },
                "y1": { "value": -0.4, "type": "number" },
                "y2": { "value": 1.4, "type": "number" }
            }),
            // Ideally we'd read CustomBezier from the API, but it isn't provided so copy EaseInAndOut
            figma_api::EasingType::CustomBezier => json!({
                "x1": { "value": 0.42, "type": "number" },
                "x2": { "value": 0.58, "type": "number" },
                "y1": { "value": 0.0, "type": "number" },
                "y2": { "value": 1.0, "type": "number" }
            }),
            figma_api::EasingType::Gentle => json!({
                "mass": { "value": 1, "type": "number" },
                "stiffness": { "value": 100, "type": "number" },
                "damping": { "value": 15, "type": "number" }
            }),
            figma_api::EasingType::Quick => json!({
                "mass": { "value": 1, "type": "number" },
                "stiffness": { "value": 300, "type": "number" },
                "damping": { "value": 20, "type": "number" }
            }),
            figma_api::EasingType::Bouncy => json!({
                "mass": { "value": 1, "type": "number" },
                "stiffness": { "value": 600, "type": "number" },
                "damping": { "value": 15, "type": "number" }
            }),
            figma_api::EasingType::Slow => json!({
                "mass": { "value": 1, "type": "number" },
                "stiffness": { "value": 80, "type": "number" },
                "damping": { "value": 20, "type": "number" }
            }),
            // Ideally we'd read CustomSpring from the API, but it isn't provided so copy Gentle
            figma_api::EasingType::CustomSpring => json!({
                "mass": { "value": 1, "type": "number" },
                "stiffness": { "value": 100, "type": "number" },
                "damping": { "value": 15, "type": "number" }
            }),
        }
    }))
}
