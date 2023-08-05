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
    fn from_flex_container_width(
        fc: &FlexContainer,
        child_alignment: Option<&AlignSelf>,
        child_flex_grow: Option<f64>,
    ) -> Self {
        match fc {
            FlexContainer {
                direction: FlexDirection::Row,
                justify_content,
                ..
            } => {
                if child_flex_grow == Some(1.0) {
                    Alignment::Align(AlignItems::Stretch)
                } else {
                    Alignment::Justify(justify_content.unwrap_or(JustifyContent::FlexStart))
                }
            }

            FlexContainer {
                direction: FlexDirection::Column,
                align_items,
                ..
            } => Alignment::Align(match child_alignment {
                Some(AlignSelf::Stretch) => AlignItems::Stretch,
                None => *align_items,
            }),
        }
    }

    fn from_flex_container_height(
        fc: &FlexContainer,
        child_alignment: Option<&AlignSelf>,
        child_flex_grow: Option<f64>,
    ) -> Self {
        match fc {
            FlexContainer {
                direction: FlexDirection::Row,
                align_items,
                ..
            } => Alignment::Align(match child_alignment {
                Some(AlignSelf::Stretch) => AlignItems::Stretch,
                None => *align_items,
            }),
            FlexContainer {
                direction: FlexDirection::Column,
                justify_content,
                ..
            } => {
                if child_flex_grow == Some(1.0) {
                    Alignment::Align(AlignItems::Stretch)
                } else {
                    Alignment::Justify(justify_content.unwrap_or(JustifyContent::FlexStart))
                }
            }
        }
    }

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

enum FlexAttempt {
    Parent,
    Child,
    Horizonal,
    Vertical,
}

const BLOCK_FLEX: FlexContainer = FlexContainer {
    align_items: AlignItems::Stretch,
    direction: FlexDirection::Column,
    gap: Length::Zero,
    justify_content: None,
};

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

            let grandchildren = match &mut child.node_type {
                IntermediateNodeType::Frame {
                    children: ref mut grandchildren,
                } => {
                    if grandchildren.iter().any(|gc| gc.location.inset.is_some()) {
                        return false;
                    }

                    Some(grandchildren)
                }
                _ => None,
            };

            let child_has_multiple_children = grandchildren
                .as_ref()
                .map(|gc| gc.len() > 1)
                .unwrap_or(false);

            let parent_flex = parent
                .flex_container
                .as_ref()
                .unwrap_or(&BLOCK_FLEX)
                .clone();
            let child_flex = child.flex_container.as_ref().unwrap_or(&BLOCK_FLEX).clone();

            let parent_width_alignment = Alignment::from_flex_container_width(
                &parent_flex,
                child.location.align_self.as_ref(),
                child.location.flex_grow,
            );
            let parent_height_alignment = Alignment::from_flex_container_height(
                &parent_flex,
                child.location.align_self.as_ref(),
                child.location.flex_grow,
            );

            let grandchild_align_self = match (&grandchildren, child_has_multiple_children) {
                (None, _) | (_, true) => None,
                (Some(gc), false) => gc.first().and_then(|n| n.location.align_self.as_ref()),
            };
            let grandchild_flex_grow = match (&grandchildren, child_has_multiple_children) {
                (None, _) | (_, true) => None,
                (Some(gc), false) => gc.first().and_then(|n| n.location.flex_grow),
            };

            let child_width_alignment = Alignment::from_flex_container_width(
                &child_flex,
                grandchild_align_self,
                grandchild_flex_grow,
            );
            let child_height_alignment = Alignment::from_flex_container_height(
                &child_flex,
                grandchild_align_self,
                grandchild_flex_grow,
            );

            for attempt in [
                FlexAttempt::Parent,
                FlexAttempt::Child,
                FlexAttempt::Horizonal,
                FlexAttempt::Vertical,
            ] {
                if !matches!(attempt, FlexAttempt::Child) && child_has_multiple_children {
                    continue;
                }

                let mut flex_container: FlexContainer = match (
                    attempt,
                    parent.flex_container.as_ref(),
                    child.flex_container.as_ref(),
                ) {
                    (FlexAttempt::Parent, None, _) => continue,
                    (FlexAttempt::Parent, Some(fc), _) => fc.clone(),
                    (FlexAttempt::Child, _, Some(fc)) => fc.clone(),
                    (FlexAttempt::Horizonal, _, _) => FlexContainer {
                        direction: FlexDirection::Row,
                        ..BLOCK_FLEX
                    },
                    (FlexAttempt::Vertical, _, _) | (FlexAttempt::Child, _, None) => {
                        BLOCK_FLEX.clone()
                    }
                };
                let (
                    child_main_axis_alignment,
                    child_counter_axis_alignment,
                    parent_main_axis_alignment,
                    parent_counter_axis_alignment,
                    child_is_main_sized,
                    child_is_counter_axis_sized,
                    parent_is_main_axis_sized,
                    parent_is_counter_axis_sized,
                ) = match flex_container.direction {
                    FlexDirection::Column => (
                        child_height_alignment,
                        child_width_alignment,
                        parent_height_alignment,
                        parent_width_alignment,
                        child.location.height.is_some(),
                        child.location.width.is_some(),
                        *height_from_descent_inclusive,
                        *width_from_descent_inclusive,
                    ),
                    FlexDirection::Row => (
                        child_width_alignment,
                        child_height_alignment,
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
                    child_main_axis_alignment.as_simplified_main_axis_alignment(),
                    parent_main_axis_alignment.as_simplified_main_axis_alignment(),
                ) {
                    (true, true, _, _) => return false,
                    (true, false, Some(j), _) => Some(j),
                    (true, false, None, _) => continue,
                    (false, true, _, None) => continue,
                    (false, true, _, Some(j)) => Some(j),
                    (false, false, _, Some(j)) => Some(j),
                    (false, false, Some(j), None) => Some(j),
                    (false, false, None, None) => continue,
                };

                let align_items = match (
                    child_is_counter_axis_sized,
                    parent_is_counter_axis_sized,
                    child_has_multiple_children,
                    child_counter_axis_alignment.as_counter_axis_alignment(),
                    parent_counter_axis_alignment.as_counter_axis_alignment(),
                ) {
                    (true, true, _, _, _) => return false,
                    (false, true, false, _, pa) => pa,
                    (false, true, true, ca, pa) => {
                        if ca == pa {
                            pa
                        } else {
                            continue;
                        }
                    }
                    (false, false, true, ca, _) => ca,
                    (false, false, false, _, pa) => pa,
                    (true, false, _, ca, _) => ca,
                };

                match (grandchildren, child_has_multiple_children) {
                    (None, _) | (_, true) => {}
                    (Some(gc), false) => {
                        if let Some(gc) = gc.first_mut() {
                            gc.location.align_self = None;
                            gc.location.flex_grow = None;
                        }
                    }
                };

                if let Some(width) = child.location.width.take() {
                    parent.location.width = Some(
                        width
                            + parent.location.padding[1].clone()
                            + parent.location.padding[3].clone(),
                    );
                }

                if let Some(height) = child.location.height.take() {
                    parent.location.height = Some(
                        height
                            + parent.location.padding[0].clone()
                            + parent.location.padding[2].clone(),
                    );
                }

                flex_container.justify_content = justify_content;
                flex_container.align_items = align_items;
                parent.flex_container = Some(flex_container);

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
            false
        },
    )
}
