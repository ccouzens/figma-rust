use crate::{
    inherited_properties::InheritedProperties,
    intermediate_node::{
        CSSVariablesMap, FrameAppearance, IntermediateNode, IntermediateNodeType, Location,
    },
};

use super::recursive_filter;

pub fn drop_empty_absolute_frames(
    node: &mut IntermediateNode,
    _css_variables: &mut CSSVariablesMap,
) -> bool {
    recursive_filter(
        node,
        &InheritedProperties::default(),
        &|node, _inherited_properties| {
            !matches!(node, IntermediateNode {
                             node_type: IntermediateNodeType::Frame { children },
                             location: Location { inset: Some(_), .. },
                             frame_appearance:
                                 FrameAppearance {
                                     background: None,
                                     box_shadow: None,
                                     stroke: None,
                                     ..
                                 },
                             ..
                        } if children.is_empty())
        },
    )
}
