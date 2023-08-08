use crate::intermediate_node::{
    AlignItems, AlignSelf, CSSVariablesMap, FlexDirection, IntermediateNode, IntermediateNodeType,
    Length,
};

use super::{
    combine_parent_child::BLOCK_FLEX, recursive_visitor_mut_sized_downwards,
    RecursiveVisitorMutSizedDownwardsProps,
};

/**
Elevate frame appearance properties and href to the parent node.

If the parent node takes its size entirely from a single child node, some
properties like background may be able to be moved from the child to the
parent.

In order for this to happen certain conditions need to be met:
* The child node must be the only statically positioned child of the parent.
* The parent may not have padding, however the child component may.
* The parent must be the size of the child:
  - The parent may not have a width or height
  - flex-grow must be 0 or unset on the parent parent
  - If the parent is absolutely positioned, at least one of the top bottom
    values should be auto and at least one of the left right values should be
    auto.
  - The parent cannot be align-self stretch.
  - The grandparent cannot have align-items stretch.
* If the child has border radius set then the parent cannot have any of the
  other properties set. This is because elevating border-radius would modify
  the existing property (eg clipping the corners of the existing background).

The way properties individually work are:
* if background is set on the parent, do not modify either background value.
  This is because the child background may not be 100% opaque so the two
  backgrounds need to be blended together by the browser. If both backgrounds
  are constant values it may be possible to precompute the resulting value, but
  this isn't possible with variable colours.
* Only elevate box-shadow, stroke and border radius for similar reasons of both
  values being required to accurately show the result.
* href of the child will be elevated if href is set on the child. The parent's
  existing value doesn't matter if the child has an href value.

When a property is elevated it is removed from the child and added to the
parent.
 */
pub fn elevate_frame_appearance_properties(
    node: &mut IntermediateNode,
    _css_variables: &mut CSSVariablesMap,
) -> bool {
    recursive_visitor_mut_sized_downwards(
        RecursiveVisitorMutSizedDownwardsProps {
            node,
            width_from_descent_inclusive: false,
            height_from_descent_inclusive: false,
        },
        &mut |RecursiveVisitorMutSizedDownwardsProps {
                  node: parent,
                  width_from_descent_inclusive,
                  height_from_descent_inclusive,
                  ..
              }| {
            let children = match parent {
                IntermediateNode {
                    node_type: IntermediateNodeType::Frame { children },
                    ..
                } => children,
                _ => return false,
            };

            let mut static_children = children
                .iter_mut()
                .filter(|c| c.location.inset.is_none())
                .collect::<Vec<_>>();

            if static_children.len() != 1 {
                return false;
            }

            let static_child = match static_children.first_mut() {
                Some(child) => child,
                None => return false,
            };

            {
                let parent_flex = parent.flex_container.as_ref().unwrap_or(&BLOCK_FLEX);
                let (
                    parent_is_main_axis_sized,
                    parent_is_counter_axis_sized,
                    _child_is_main_axis_sized,
                    child_is_counter_axis_sized,
                ) = match parent_flex.direction {
                    FlexDirection::Row => (
                        *width_from_descent_inclusive,
                        *height_from_descent_inclusive,
                        static_child.location.width.is_some(),
                        static_child.location.height.is_some(),
                    ),
                    FlexDirection::Column => (
                        *height_from_descent_inclusive,
                        *width_from_descent_inclusive,
                        static_child.location.height.is_some(),
                        static_child.location.width.is_some(),
                    ),
                };

                if parent_is_main_axis_sized && static_child.location.flex_grow != Some(1.0) {
                    return false;
                }
                if parent_is_counter_axis_sized && child_is_counter_axis_sized {
                    return false;
                }
                if parent_is_counter_axis_sized
                    && parent_flex.align_items != AlignItems::Stretch
                    && static_child.location.align_self != Some(AlignSelf::Stretch)
                {
                    return false;
                }
            }
            if !matches!(
                parent.location.padding,
                [Length::Zero, Length::Zero, Length::Zero, Length::Zero]
            ) {
                return false;
            }

            let mut mutated = false;

            if parent.frame_appearance.background.is_none()
                && static_child.frame_appearance.background.is_some()
            {
                parent.frame_appearance.background =
                    static_child.frame_appearance.background.take();
                mutated = true;
            }

            if (parent.frame_appearance.border_radius.is_none()
                || parent.frame_appearance.border_radius
                    == static_child.frame_appearance.border_radius)
                && static_child.frame_appearance.border_radius.is_some()
            {
                parent.frame_appearance.border_radius =
                    static_child.frame_appearance.border_radius.take();
                mutated = true;
            }

            if parent.frame_appearance.box_shadow.is_none()
                && static_child.frame_appearance.box_shadow.is_some()
            {
                parent.frame_appearance.box_shadow =
                    static_child.frame_appearance.box_shadow.take();
                mutated = true;
            }

            if parent.frame_appearance.stroke.is_none()
                && static_child.frame_appearance.stroke.is_some()
            {
                parent.frame_appearance.stroke = static_child.frame_appearance.stroke.take();
                mutated = true;
            }

            if static_child.href.is_some() {
                parent.href = static_child.href.take();
                mutated = true;
            }

            mutated
        },
    )
}
