use crate::intermediate_node::{
    AlignItems, AlignSelf, Appearance, CSSVariablesMap, FlexContainer, FlexDirection,
    IntermediateNode, IntermediateNodeType, JustifyContent, Length,
};

use super::{recursive_visitor_mut_sized_downwards, RecursiveVisitorMutSizedDownwardsProps};

#[derive(Copy, Clone, Debug)]
enum Alignment {
    Justify(JustifyContent),
    Align(AlignItems),
}

impl Alignment {
    fn as_simplified_main_axis_alignment(&self) -> Option<JustifyContent> {
        match self {
            Alignment::Justify(JustifyContent::FlexStart | JustifyContent::SpaceBetween) => {
                Some(JustifyContent::FlexStart)
            }
            Alignment::Justify(JustifyContent::Center) => Some(JustifyContent::Center),
            Alignment::Justify(JustifyContent::FlexEnd) => Some(JustifyContent::FlexEnd),
            Alignment::Align(AlignItems::Baseline | AlignItems::FlexStart) => {
                Some(JustifyContent::FlexStart)
            }
            Alignment::Align(AlignItems::Center) => Some(JustifyContent::Center),
            Alignment::Align(AlignItems::FlexEnd) => Some(JustifyContent::FlexEnd),
            Alignment::Align(AlignItems::Stretch) => None,
        }
    }

    fn as_counter_axis_alignment(&self) -> AlignItems {
        match self {
            Alignment::Align(x) => *x,
            Alignment::Justify(JustifyContent::FlexStart | JustifyContent::SpaceBetween) => {
                AlignItems::FlexStart
            }
            Alignment::Justify(JustifyContent::FlexEnd) => AlignItems::FlexEnd,
            Alignment::Justify(JustifyContent::Center) => AlignItems::Center,
        }
    }
}

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
* The child must be the same size as the parent's context box
  - The child is allowed to be smaller or bigger if its content is sized as
    such, but mustn't have width or height properties sizing it differently.
  - Because we're concerned with the parent's content box, the parent is
    allowed padding.
  - We consider vertical sizing and horizontal sizing separately. For this
    comment only horizontal sizing (width) will be discussed.
    - The child is said to set the width if it has the `width` property set
    - The parent is said to set the width if
      - It has the width property
      - It has a flex-grow property set non-zero and the grandparent is
        flex-direction row
      - It is absolutely positioned with both left and right inset values
      - It has align-self stretch and the grandparent is flex-direction column
      - The grandparent is flex-direction row with justify-content stretch
      - The grandparent is flex-direction column with align-items stretch
    - Parent and child are not allowed to both set the width
    - If parent has width:
      - The alignment on the horizontal axis must be kept from the parent's
        existing value or the child's align-stretch value if set. Note that
        `AlignItems` has properties that `JustifyContent` cannot represent, so
        this may prevent this mutation.
    - If the child has width and flex-direction row use the child's align-items
      value.
    - If the child has width and flex-direction column use the child's
      justify-content value.
    - If the parent has width and the child has flex-direction row use the
      parent's horizontal axis alignment. But simplify it to start, center or
      end.
    - If the parent has width and the child has flex-direction column use the
      child's align items value. But this has to match the parent's horizontal
      axis alignment.

From the parent take:
* The figma properties (name, id and type)
* location align-self
* location flex-grow
* location inset
* opacity
* background
* border radius
* box shadow
* stroke
* href

From the child take:
* flex-container (direction and gap)
* The appearance properties, except for opacity. Take the parent values if not
set for the child.
* The node type and any associated properties (eg grandchildren, text or vectors).
* width and height. Add in the parent padding if present.

From a combination take:
* padding - add it together
*/

/**
 * Look at parent, child combinations
 *
 * For each combination work out:
 * - is width coming from the parent, and if so the parent's horizontal alignment
 * - is height coming from the parent, and if so the parent's vertical alignment
 * - is width coming from the child - if both do not continue
 * - is height coming from the child - if both do not continue
 * - if the parent is determining the main axis size, is the parent alignment compatible with justify content (start, center or end)?
 * - if the parent is determining the counter axis size, convert the parent alignment to AlignItems. Is it the same as the child align-items value?
 */
pub fn combine_parent_child(
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

            if children.len() != 1 {
                return false;
            }

            let child = match children.first_mut() {
                Some(child) => child,
                None => return false,
            };

            if child.frame_appearance.background.is_some()
                || child.appearance.opacity.unwrap_or(1.0) != 1.0
                || child.frame_appearance.border_radius.is_some()
                || child.frame_appearance.box_shadow.is_some()
                || child.frame_appearance.stroke.is_some()
                || child.href.is_some()
                || child.location.inset.is_some()
            {
                return false;
            }

            let child_has_multiple_children = match &child.node_type {
                IntermediateNodeType::Frame { children } if children.len() > 1 => true,
                _ => false,
            };

            let parent_width_alignment = match parent.flex_container.as_ref() {
                None => Alignment::Align(AlignItems::Stretch),

                Some(FlexContainer {
                    direction: FlexDirection::Row,
                    justify_content,
                    ..
                }) => Alignment::Justify(justify_content.unwrap_or(JustifyContent::FlexStart)),

                Some(FlexContainer {
                    direction: FlexDirection::Column,
                    align_items,
                    ..
                }) => Alignment::Align(match child.location.align_self {
                    Some(AlignSelf::Stretch) => AlignItems::Stretch,
                    None => *align_items,
                }),
            };

            let parent_height_alignment = match parent.flex_container.as_ref() {
                None => Alignment::Align(AlignItems::FlexStart),
                Some(FlexContainer {
                    direction: FlexDirection::Row,
                    align_items,
                    ..
                }) => Alignment::Align(match child.location.align_self {
                    Some(AlignSelf::Stretch) => AlignItems::Stretch,
                    None => *align_items,
                }),
                Some(FlexContainer {
                    direction: FlexDirection::Column,
                    justify_content,
                    ..
                }) => Alignment::Justify(justify_content.unwrap_or(JustifyContent::FlexStart)),
            };

            let (
                parent_main_axis_alignment,
                parent_counter_axis_alignment,
                child_is_main_sized,
                child_is_counter_axis_sized,
                parent_is_main_axis_sized,
                parent_is_counter_axis_sized,
            ) = match child.flex_container.as_ref().map(|f| f.direction) {
                None | Some(FlexDirection::Column) => (
                    parent_height_alignment,
                    parent_width_alignment,
                    child.location.height.is_some(),
                    child.location.width.is_some(),
                    *height_from_descent_inclusive,
                    *width_from_descent_inclusive,
                ),
                Some(FlexDirection::Row) => (
                    parent_width_alignment,
                    parent_height_alignment,
                    child.location.width.is_some(),
                    child.location.height.is_some(),
                    *width_from_descent_inclusive,
                    *height_from_descent_inclusive,
                ),
            };

            let justify_content = match (
                child_is_main_sized,
                parent_is_main_axis_sized,
                parent_main_axis_alignment.as_simplified_main_axis_alignment(),
            ) {
                (true, true, _) => return false,
                (true, false, _) => child
                    .flex_container
                    .as_ref()
                    .and_then(|f| f.justify_content),
                (false, true, None) => return false,
                (false, true, Some(j)) => Some(j),
                (false, false, Some(j)) => Some(j),
                (false, false, None) => child
                    .flex_container
                    .as_ref()
                    .and_then(|f| f.justify_content),
            };

            let align_items = match (
                child_is_counter_axis_sized,
                parent_is_counter_axis_sized,
                child_has_multiple_children,
                parent_counter_axis_alignment.as_counter_axis_alignment(),
            ) {
                (true, true, _, _) => return false,
                (true, false, _, _) | (false, false, true, _) => child
                    .flex_container
                    .as_ref()
                    .map(|f| f.align_items)
                    .unwrap_or(AlignItems::Stretch),
                (false, true, true, parent_align_items) => {
                    if parent_align_items
                        == child
                            .flex_container
                            .as_ref()
                            .map(|f| f.align_items)
                            .unwrap_or(AlignItems::Stretch)
                    {
                        parent_align_items
                    } else {
                        return false;
                    }
                }
                (false, _, false, parent_align_items) => parent_align_items,
            };

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
            match parent.flex_container {
                Some(ref mut f) => {
                    f.justify_content = justify_content;
                    f.align_items = align_items;
                }
                None => {
                    if justify_content.is_some() {
                        parent.flex_container = Some(FlexContainer {
                            align_items: align_items,
                            direction: FlexDirection::Column,
                            gap: Length::Zero,
                            justify_content,
                        })
                    }
                }
            };
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

            true
        },
    )
}
