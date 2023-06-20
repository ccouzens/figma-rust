use crate::intermediate_node::{IntermediateNode, IntermediateNodeType};

mod collapse_empty_siblings;

pub fn recursive_visitor_mut(
    node: &mut IntermediateNode,
    visitor: &mut impl FnMut(&mut IntermediateNode),
) {
    // TODO/BUG pass in defaulted CSS values from parent scope. Values can be inherited from parent nodes, so we need to know them to make deductions about the children.
    visitor(node);
    if let IntermediateNodeType::Frame { ref mut children } = node.node_type {
        for child in children.iter_mut() {
            recursive_visitor_mut(child, visitor);
        }
    }
}

pub use collapse_empty_siblings::collapse_empty_siblings;
