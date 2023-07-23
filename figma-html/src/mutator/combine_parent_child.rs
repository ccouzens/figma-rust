use crate::{
    inherited_properties::InheritedProperties,
    intermediate_node::{
        AlignItems, AlignSelf, Appearance, CSSVariablesMap, FlexContainer, Inset, IntermediateNode,
        IntermediateNodeType, Length,
    },
};

use super::recursive_visitor_mut;

/**
Combine child nodes into their parent

In order to do this certain conditions have to be met:
* No other children of the parent node
* No background on the child node
* The child must not be absolutely positioned
* The child must not have an opacity
* The child must not have a border radius as the clipping complicates
  interactions with padding.
* The child must not have box shadow or stroke as it complicates interactions
  with padding and the parent box shadow and stroke.
* The child must not have an href, as the target area might grow if combined
  with the parent's padding.
* The parent must have its size set entirely by the child.
  - The only exception to this is the parent's padding
  - Neither width or height can be set on the parent
  - flex-grow must be 0 or unset on the parent parent
  - If the parent is absolutely positioned, at least one of the top bottom
    values should be auto and at least one of the left right values should be
    auto.
  - The parent cannot be align-self stretch.
  - The grandparent cannot have align-items stretch.
  - This is because it's complicated to combine the child's flex properties
    into the parnt if it's not the full size of the parent.

From the parent take:
* The figma properties (name, id and type)
* location align-self
* location flex-grow (which is to say unset it, as the parent shouldn't have
  it)
* location inset
* opacity
* background
* border radius
* box shadow
* stroke
* href

From the child take:
* width and height. Add in the parent padding if present.
* flex-container
* The appearance properties, except for opacity. Take the parent values if not
  set for the child.
* The node type and any associated properties (eg grandchildren, text or vectors).


From a combination take:
* padding - add it together

*/
pub fn combine_parent_child(
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
                if !matches!(
                    grand_parent.flex_container,
                    Some(FlexContainer {
                        align_items: AlignItems::Stretch,
                        ..
                    })
                ) {
                    for parent in parents.iter_mut() {
                        mutated = parent_child_combiner(parent) | mutated;
                    }
                }
            }
            mutated
        },
    );

    mutated = parent_child_combiner(node) | mutated;
    mutated
}

/**
Check the parent and child are eligible to be combined and do so.
Assumes the grandparent either doesn't exist, or has already been validated
to not apply align-items: stretch to the parent.
*/
fn parent_child_combiner(parent: &mut IntermediateNode) -> bool {
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

    if let IntermediateNode {
        node_type: IntermediateNodeType::Frame { children },
        ..
    } = parent
    {
        if children.len() != 1 {
            return false;
        }
        if let Some(child) = children.first_mut() {
            if child.frame_appearance.background.is_some()
                || child.location.inset.is_some()
                || child.appearance.opacity.unwrap_or(1.0) != 1.0
                || child.frame_appearance.border_radius.is_some()
                || child.frame_appearance.box_shadow.is_some()
                || child.frame_appearance.stroke.is_some()
                || child.href.is_some()
            {
                return false;
            }

            if let Some(width) = child.location.width.take() {
                parent.location.width = Some(
                    width + parent.location.padding[1].clone() + parent.location.padding[3].clone(),
                );
            }

            if let Some(height) = child.location.height.take() {
                parent.location.height = Some(
                    height
                        + parent.location.padding[0].clone()
                        + parent.location.padding[2].clone(),
                );
            }

            parent.flex_container = child.flex_container.take();
            parent.appearance = Appearance {
                color: child
                    .appearance
                    .color
                    .take()
                    .or(parent.appearance.color.take()),
                fill: child
                    .appearance
                    .fill
                    .take()
                    .or(parent.appearance.fill.take()),
                font: child
                    .appearance
                    .font
                    .take()
                    .or(parent.appearance.font.take()),
                opacity: parent.appearance.opacity.take(),
                preserve_whitespace: child.appearance.preserve_whitespace
                    || parent.appearance.preserve_whitespace,
                text_tranform: child
                    .appearance
                    .text_tranform
                    .take()
                    .or(parent.appearance.text_tranform.take()),
                text_decoration_line: child
                    .appearance
                    .text_decoration_line
                    .take()
                    .or(parent.appearance.text_decoration_line.take()),
            };

            for i in 0..4 {
                parent.location.padding[i] =
                    std::mem::replace(&mut parent.location.padding[i], Length::Zero)
                        + std::mem::replace(&mut child.location.padding[i], Length::Zero);
            }

            parent.node_type = std::mem::replace(
                &mut child.node_type,
                IntermediateNodeType::Frame { children: vec![] },
            );

            return true;
        }
    }
    false
}
