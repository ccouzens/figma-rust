use crate::{
    inherited_properties::InheritedProperties,
    intermediate_node::{
        AlignItems, AlignSelf, FlexDirection, Inset, IntermediateNode, IntermediateNodeType,
    },
};

mod collapse_to_gap;
mod collapse_to_padding;
mod combine_parent_child;
mod drop_empty_absolute_frames;
mod elevate_frame_appearance_properties;

pub use collapse_to_gap::collapse_to_gap;
pub use collapse_to_padding::collapse_to_padding;
pub use combine_parent_child::combine_parent_child;
pub use drop_empty_absolute_frames::drop_empty_absolute_frames;
pub use elevate_frame_appearance_properties::elevate_frame_appearance_properties;

/**
Recursive node visitor with callback that can mutate the node
 */
pub fn recursive_visitor_mut(
    node: &mut IntermediateNode,
    inherited_properties: &InheritedProperties,
    visitor: &mut impl FnMut(&mut IntermediateNode, &InheritedProperties) -> bool,
) -> bool {
    let mut mutated = false;
    mutated |= visitor(node, inherited_properties);
    let inherited_properties = InheritedProperties::inherit(node, inherited_properties);

    if let IntermediateNodeType::Frame { ref mut children } = node.node_type {
        for child in children.iter_mut() {
            mutated |= recursive_visitor_mut(child, &inherited_properties, visitor);
        }
    }
    mutated
}

pub struct RecursiveVisitorMutSizedDownwardsProps<'a, 'b> {
    pub node: &'a mut IntermediateNode<'b>,
    pub width_from_descent_inclusive: bool,
    pub height_from_descent_inclusive: bool,
}

/**
Assuming the parent has a width, does the parent influence this node's width?

If parent is a flex-container, the flex-direction and align-items must be provided.
 */
fn width_set_by_parent(
    node: &IntermediateNode,
    parent_flex_direction: Option<FlexDirection>,
    parent_align_items: Option<AlignItems>,
) -> bool {
    match node.location.inset {
        None | Some([_, Inset::Auto, _, _]) | Some([_, _, _, Inset::Auto]) => {}
        Some([_, Inset::Linear { .. }, _, Inset::Linear { .. }]) => return true,
    }
    match parent_flex_direction {
        Some(FlexDirection::Row) => node.location.flex_grow.unwrap_or(0.0) != 0.0,
        Some(FlexDirection::Column) => {
            node.location.align_self == Some(AlignSelf::Stretch)
                || parent_align_items == Some(AlignItems::Stretch)
        }
        None => node.location.width.is_none() && node.location.inset.is_none(),
    }
}

/**
Assuming the parent has a height, does the parent influence this node's hieght?

If parent is a flex-container, the flex-direction and align-items must be provided.
*/
fn height_set_by_parent(
    node: &IntermediateNode,
    parent_flex_direction: Option<FlexDirection>,
    parent_align_items: Option<AlignItems>,
) -> bool {
    match node.location.inset {
        None | Some([Inset::Auto, _, _, _]) | Some([_, _, Inset::Auto, _]) => {}
        Some([Inset::Linear { .. }, _, Inset::Linear { .. }, _]) => return true,
    }
    match parent_flex_direction {
        Some(FlexDirection::Row) => {
            node.location.align_self == Some(AlignSelf::Stretch)
                || parent_align_items == Some(AlignItems::Stretch)
        }
        Some(FlexDirection::Column) => node.location.flex_grow.unwrap_or(0.0) != 0.0,
        None => false,
    }
}

/**
Does this node have enough information in its direct properties to determine its width.

Ignores cases where the parent controls the width and ignores this node's descendents
 */
fn width_set_by_self(node: &IntermediateNode) -> bool {
    node.location.width.is_some()
}

/**
Does this node have enough information in its direct properties to determine its height.

Ignores cases where the parent controls the height and ignores this node's descendents
 */
fn height_set_by_self(node: &IntermediateNode) -> bool {
    node.location.height.is_some()
}

/**
Recursive node visitor with callback that can mutate the node

Works out if the node's size is determined by its ancesters. If the
node's size is determined by its contents or its own properties that isn't
counted.
*/
pub fn recursive_visitor_mut_sized_downwards<'a, 'b>(
    props: RecursiveVisitorMutSizedDownwardsProps<'a, 'b>,
    visitor: &mut impl FnMut(&mut RecursiveVisitorMutSizedDownwardsProps<'a, 'b>) -> bool,
) -> bool {
    let mut mutated = false;
    let mut props = RecursiveVisitorMutSizedDownwardsProps {
        width_from_descent_inclusive: width_set_by_self(props.node)
            || props.width_from_descent_inclusive,
        height_from_descent_inclusive: height_set_by_self(props.node)
            || props.height_from_descent_inclusive,
        ..props
    };
    mutated |= visitor(&mut props);

    if let IntermediateNodeType::Frame { ref mut children } = props.node.node_type {
        let parent_flex_direction = props.node.flex_container.as_ref().map(|f| f.direction);
        let parent_align_items = props.node.flex_container.as_ref().map(|f| f.align_items);
        let static_child_count = children
            .iter()
            .filter(|c| c.location.inset.is_none())
            .count();
        for child in children.iter_mut() {
            // If there are multiple static children, assume the parent could be sized from a sibling
            let static_child = child.location.inset.is_none();
            let child_props = RecursiveVisitorMutSizedDownwardsProps {
                width_from_descent_inclusive: (static_child && static_child_count > 1)
                    || props.width_from_descent_inclusive
                        && width_set_by_parent(child, parent_flex_direction, parent_align_items),
                height_from_descent_inclusive: (static_child && static_child_count > 1)
                    || props.height_from_descent_inclusive
                        && height_set_by_parent(child, parent_flex_direction, parent_align_items),
                node: child,
            };

            mutated |= recursive_visitor_mut_sized_downwards(child_props, visitor);
        }
    }
    mutated
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
