use crate::{
    inherited_properties::InheritedProperties,
    intermediate_node::{IntermediateNode, IntermediateNodeType},
};

mod collapse_to_padding;
mod combine_parent_child;
mod drop_empty_absolute_frames;

pub use collapse_to_padding::collapse_to_padding;
pub use combine_parent_child::combine_parent_child;
pub use drop_empty_absolute_frames::drop_empty_absolute_frames;

pub fn recursive_visitor_mut(
    node: &mut IntermediateNode,
    inherited_properties: &InheritedProperties,
    visitor: &mut impl FnMut(&mut IntermediateNode, &InheritedProperties),
) {
    visitor(node, inherited_properties);
    let inherited_properties = InheritedProperties::inherit(node, inherited_properties);

    if let IntermediateNodeType::Frame { ref mut children } = node.node_type {
        for child in children.iter_mut() {
            recursive_visitor_mut(child, &inherited_properties, visitor);
        }
    }
}

/**
Visit every node except for the root node.

If the visitor returns `false` then the node will be removed.

The root node is not visited as it cannot be removed.
*/
pub fn recursive_filter(
    node: &mut IntermediateNode,
    inherited_properties: &InheritedProperties,
    visitor: &impl Fn(&IntermediateNode, &InheritedProperties) -> bool,
) -> bool {
    let mut mutated = false;
    let inherited_properties = InheritedProperties::inherit(node, inherited_properties);

    if let IntermediateNodeType::Frame { ref mut children } = node.node_type {
        for child in children.iter_mut() {
            mutated = recursive_filter(child, &inherited_properties, visitor) || mutated;
        }

        *children = children
            .drain(..)
            .filter(|child| {
                let keep = visitor(child, &inherited_properties);
                mutated |= !keep;
                keep
            })
            .collect();
    }
    mutated
}
