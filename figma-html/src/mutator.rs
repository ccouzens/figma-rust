use crate::{
    inherited_properties::InheritedProperties,
    intermediate_node::{IntermediateNode, IntermediateNodeType},
};

mod collapse_empty_siblings;

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

pub use collapse_empty_siblings::collapse_empty_siblings;
