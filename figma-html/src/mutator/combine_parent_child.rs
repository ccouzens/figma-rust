use crate::intermediate_node::{
    AlignItems, AlignSelf, Appearance, CSSVariablesMap, FlexContainer, FlexDirection,
    IntermediateNode, IntermediateNodeType, JustifyContent, Length,
};

use super::{recursive_visitor_mut_sized_downwards, RecursiveVisitorMutSizedDownwardsProps};

#[derive(Copy, Clone, Debug, PartialEq)]
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

    /**
    Convert to main axis alignment (justify content).

    The bool signifies if the children should have flex-grow set
    */
    fn as_simplified_main_axis_alignment(
        &self,
        single_child: bool,
    ) -> Option<(JustifyContent, bool)> {
        match (self, single_child) {
            (Alignment::Justify(j), _) => Some((*j, false)),
            (Alignment::Align(AlignItems::FlexStart), _) => {
                Some((JustifyContent::FlexStart, false))
            }
            (Alignment::Align(AlignItems::Center), _) => Some((JustifyContent::Center, false)),
            (Alignment::Align(AlignItems::FlexEnd), _) => Some((JustifyContent::FlexEnd, false)),
            (Alignment::Align(AlignItems::Stretch), true) => {
                Some((JustifyContent::FlexStart, true))
            }
            (Alignment::Align(AlignItems::Stretch), false) => None,
            (Alignment::Align(AlignItems::Baseline), true) => {
                Some((JustifyContent::FlexStart, false))
            }
            (Alignment::Align(AlignItems::Baseline), false) => None,
        }
    }

    fn as_counter_axis_alignment(&self, single_child: bool) -> Option<AlignItems> {
        match (self, single_child) {
            (Alignment::Align(x), _) => Some(*x),
            (Alignment::Justify(JustifyContent::FlexStart), _)
            | (Alignment::Justify(JustifyContent::SpaceBetween), true) => {
                Some(AlignItems::FlexStart)
            }
            (Alignment::Justify(JustifyContent::FlexEnd), _) => Some(AlignItems::FlexEnd),
            (Alignment::Justify(JustifyContent::Center), _) => Some(AlignItems::Center),
            (Alignment::Justify(JustifyContent::SpaceBetween), false) => None,
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

                let mut flex_container: FlexContainer = match attempt {
                    FlexAttempt::Parent => parent_flex.clone(),
                    FlexAttempt::Child => child_flex.clone(),
                    FlexAttempt::Horizonal => FlexContainer {
                        direction: FlexDirection::Row,
                        ..BLOCK_FLEX
                    },
                    FlexAttempt::Vertical => BLOCK_FLEX.clone(),
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
                        child.location.width.is_some()
                            || child.location.flex_grow == Some(1.0)
                                && *width_from_descent_inclusive,
                        child.location.height.is_some()
                            || (child.location.align_self == Some(AlignSelf::Stretch)
                                || parent_height_alignment
                                    == Alignment::Align(AlignItems::Stretch))
                                && *height_from_descent_inclusive,
                        *width_from_descent_inclusive,
                        *height_from_descent_inclusive,
                    ),
                };

                let (main_axis_equal, counter_axis_equal) =
                    match flex_container.direction == parent_flex.direction {
                        true => (
                            child.location.flex_grow == Some(1.0),
                            (child.location.align_self == Some(AlignSelf::Stretch)
                                || parent_flex.align_items == AlignItems::Stretch)
                                && !child_is_counter_axis_sized,
                        ),
                        false => (
                            (child.location.align_self == Some(AlignSelf::Stretch)
                                || parent_flex.align_items == AlignItems::Stretch)
                                && !child_is_main_sized,
                            child.location.flex_grow == Some(1.0),
                        ),
                    };

                let (justify_content, use_flex_grow) = match (
                    parent_is_main_axis_sized,
                    child_is_main_sized,
                    parent_main_axis_alignment.as_simplified_main_axis_alignment(true),
                    child_main_axis_alignment
                        .as_simplified_main_axis_alignment(!child_has_multiple_children),
                    main_axis_equal,
                ) {
                    (true, true, _, _, _) => return false,
                    (true, false, Some(pj), _, false) => pj,
                    (true, false, None, _, false) => continue,
                    (true, false, _, Some(cj), true) => cj,
                    (true, false, _, None, true) => continue,
                    (false, true, _, Some(cj), _) => cj,
                    (false, true, _, None, _) => continue,
                    (false, false, None, None, _) => continue,
                    (false, false, None, Some(cj), _) => cj,
                    (false, false, Some(_pj), None, true) => continue,
                    (false, false, Some(pj), None, false) => pj,
                    (false, false, _, Some(cj), true) => cj,
                    // Flex no stretch is boring, avoid it
                    (false, false, Some((JustifyContent::FlexStart, false)), Some(cj), false) => cj,
                    (false, false, Some(pj), Some((JustifyContent::FlexStart, false)), false) => pj,
                    (false, false, Some(pj), Some(_cj), false) => pj,
                };

                let align_items = match (
                    parent_is_counter_axis_sized,
                    child_is_counter_axis_sized,
                    parent_counter_axis_alignment.as_counter_axis_alignment(true),
                    child_counter_axis_alignment
                        .as_counter_axis_alignment(!child_has_multiple_children),
                    child_has_multiple_children,
                    counter_axis_equal,
                ) {
                    (true, true, _, _, _, _) => return false,
                    (false, _, _, Some(ca), true, _) => ca,
                    (false, _, _, None, true, _) => {
                        continue;
                    }
                    (true, false, Some(pa), _, false, false) => pa,
                    (true, false, Some(pa), Some(ca), true, false) => {
                        if pa == ca {
                            pa
                        } else {
                            continue;
                        }
                    }
                    (true, false, None, _, _, false) => continue,
                    (_, _, _, Some(ca), _, true) => ca,
                    (_, _, _, None, _, true) => continue,
                    (_, _, _, None, true, _) => continue,
                    (false, _, Some(pa), None, false, false) => pa,
                    (_, _, None, None, _, _) => continue,

                    (false, _, None, Some(ca), false, false) => ca,
                    (false, true, _, Some(ca), false, false) => ca,
                    (false, false, Some(pa), Some(_), false, false) => pa,
                };

                match (grandchildren, child_has_multiple_children) {
                    (None, _) | (_, true) => {}
                    (Some(gc), false) => {
                        if let Some(gc) = gc.first_mut() {
                            gc.location.align_self = None;
                            gc.location.flex_grow = if use_flex_grow { Some(1.0) } else { None };
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

                flex_container.justify_content = Some(justify_content);
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
