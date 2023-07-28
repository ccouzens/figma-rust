use crate::{
    intermediate_node::{
        AlignItems, AlignSelf, CSSVariablesMap, FlexContainer, FrameAppearance, Inset,
        IntermediateNode, IntermediateNodeType, Length,
    },
    InheritedProperties,
};

use super::recursive_visitor_mut;

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
    let mut mutated = recursive_visitor_mut(
        node,
        &InheritedProperties::default(),
        &mut |grand_parent, _inherited_properties| {
            let mut mutated = false;
            if let IntermediateNode {
                node_type: IntermediateNodeType::Frame { children: parents },
                ..
            } = grand_parent
            {
                for parent in parents.iter_mut() {
                    if parent.location.inset.is_some()
                        || !matches!(
                            grand_parent.flex_container,
                            Some(FlexContainer {
                                align_items: AlignItems::Stretch,
                                ..
                            })
                        )
                    {
                        mutated = parent_child_elevator(parent) | mutated;
                    }
                }
            }
            mutated
        },
    );

    mutated = parent_child_elevator(node) | mutated;
    mutated
}

/**
Check if the child properties are eligible to be elevated to the parent and do so.
Assumes the grandparent either doesn't exist, or has already been validated
to not apply align-items: stretch to the parent.
*/
fn parent_child_elevator(parent: &mut IntermediateNode) -> bool {
    if parent.location.width.is_some() || parent.location.height.is_some() {
        return false;
    }

    if let Some(flex_grow) = parent.location.flex_grow {
        if flex_grow != 0.0 {
            return false;
        }
    }

    if let Some(inset) = &parent.location.inset {
        if !(matches!(inset, &[Inset::Auto, _, _, _] | &[_, _, Inset::Auto, _])
            && matches!(inset, &[_, Inset::Auto, _, _] | &[_, _, _, Inset::Auto]))
        {
            return false;
        }
    }

    if matches!(parent.location.align_self, Some(AlignSelf::Stretch)) {
        return false;
    }

    if !matches!(
        parent.location.padding,
        [Length::Zero, Length::Zero, Length::Zero, Length::Zero]
    ) {
        return false;
    }

    if let IntermediateNode {
        node_type: IntermediateNodeType::Frame { children },
        ..
    } = parent
    {
        let mut static_children = children.iter_mut().filter(|n| n.location.inset.is_none());
        if let (Some(child), None) = (static_children.next(), static_children.next()) {
            if child.frame_appearance.border_radius.is_some()
                && !matches!(
                    parent,
                    IntermediateNode {
                        frame_appearance: FrameAppearance {
                            background: None,
                            border_radius: None,
                            box_shadow: None,
                            stroke: None
                        },
                        href: None,
                        ..
                    }
                )
            {
                return false;
            }

            let mut mutated = false;

            if parent.frame_appearance.background.is_none()
                && child.frame_appearance.background.is_some()
            {
                parent.frame_appearance.background = child.frame_appearance.background.take();
                mutated = true;
            }

            if parent.frame_appearance.border_radius.is_none()
                && child.frame_appearance.border_radius.is_some()
            {
                parent.frame_appearance.border_radius = child.frame_appearance.border_radius.take();
                mutated = true;
            }

            if parent.frame_appearance.box_shadow.is_none()
                && child.frame_appearance.box_shadow.is_some()
            {
                parent.frame_appearance.box_shadow = child.frame_appearance.box_shadow.take();
                mutated = true;
            }

            if parent.frame_appearance.stroke.is_none() && child.frame_appearance.stroke.is_some() {
                parent.frame_appearance.stroke = child.frame_appearance.stroke.take();
                mutated = true;
            }

            if child.href.is_some() {
                parent.href = child.href.take();
                mutated = true;
            }

            return mutated;
        }
    }
    false
}
