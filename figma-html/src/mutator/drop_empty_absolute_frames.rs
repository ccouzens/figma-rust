use crate::{
    inherited_properties::InheritedProperties,
    intermediate_node::{
        CSSVariablesMap, FrameAppearance, IntermediateNode, IntermediateNodeType, Location,
    },
};

use super::recursive_filter;

/**
Drop empty absolutely positioned frames

Drop frames with:
- no children
- no background background or stroke
- absolutely positioned so no affect on parent size
 */
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
                            href: None,
                             ..
                        } if children.is_empty())
        },
    )
}
