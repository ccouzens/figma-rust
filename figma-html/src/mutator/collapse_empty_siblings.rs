use std::borrow::Cow;

use crate::intermediate_node::{
    CSSVariablesMap, FlexContainer, FlexDirection, FrameAppearance, IntermediateNode,
    IntermediateNodeType, JustifyContent, Length,
};

use super::recursive_visitor_mut;

/**
 * Look for pairs of nodes where one is merely sizing and can be converted into `padding` on the other.
 *
 * The node to be removed will be referred to as the empty node.
 * The node to stay will be referred to as the kept node.
 *
 * The empty node must be of type `IntermediateNodeType::Frame` with no children.
 * Both nodes must have equal flex-grow values.
 * If flex-grow isn't 0, the sizes of either node mustn't come from width (if direction row) or height (if direction column).
 *
 * The parent must be a flex container.
 * JustifyContent must be either flex-start (default), center, or flex-end so as not to have unpredictable gaps between the children.
 * The gap of the parent container is added to the size.
 *
 * The empty node must be the first or last child of the parent (ignoring any absolutely positioned children).
 * The kept node must be an immediate sibling of the empty node (ignoring any absolutely positioned siblings).
 *
 * Any appearance properties on the empty node are discarded.
 *
 * The only frame-appearance property either the empty node or the kept node are allowed is background, and only if it matches each
 * other. If background is set, it must either match the parent or the parent is not allowed a gap.
 *
 * Neither node is allowed an href.
 *
 * The size (width/height or padding) of the empty node is added to the padding of the kept node.
 * If the kept node uses width (if direction row) or height (if direction column) the size is also added to them.
 *
 * Returns true if any substitutions were made, false otherwise.
 */
pub fn collapse_empty_siblings(
    node: &mut IntermediateNode,
    _css_variables: &mut CSSVariablesMap,
) -> bool {
    let mut mutated = false;

    recursive_visitor_mut(node, &mut |parent| {
        for &direction_forwards in [false, true].iter() {
            if let IntermediateNode {
                frame_appearance:
                    FrameAppearance {
                        background: parent_node_background,
                        ..
                    },
                flex_container:
                    Some(FlexContainer {
                        direction: flex_direction,
                        gap,
                        justify_content:
                            None
                            | Some(
                                JustifyContent::FlexStart
                                | JustifyContent::Center
                                | JustifyContent::FlexEnd,
                            ),
                        ..
                    }),
                node_type: IntermediateNodeType::Frame { children },
                ..
            } = parent
            {
                // need to repeat this going backwards
                let mut static_children =
                    children.iter_mut().filter(|c| c.location.inset.is_none());
                if let (
                    Some(
                        ref empty_node @ IntermediateNode {
                            node_type:
                                IntermediateNodeType::Frame {
                                    children: ref empty_node_children,
                                },
                            frame_appearance:
                                FrameAppearance {
                                    background: ref empty_node_background,
                                    border_radius: None,
                                    box_shadow: None,
                                    stroke: None,
                                },
                            href: None,
                            ..
                        },
                    ),
                    Some(
                        ref mut kept_node @ IntermediateNode {
                            frame_appearance:
                                FrameAppearance {
                                    background: _,
                                    border_radius: None,
                                    box_shadow: None,
                                    stroke: None,
                                },
                            href: None,
                            ..
                        },
                    ),
                ) = (
                    match direction_forwards {
                        true => static_children.next(),
                        false => static_children.next_back(),
                    },
                    match direction_forwards {
                        true => static_children.next(),
                        false => static_children.next_back(),
                    },
                ) {
                    if empty_node.location.flex_grow == kept_node.location.flex_grow
                        && empty_node_children.is_empty()
                        && empty_node_background.as_deref()
                            == kept_node.frame_appearance.background.as_deref()
                        && (empty_node.location.flex_grow.unwrap_or(0.0) == 0.0
                            || (*flex_direction == FlexDirection::Row
                                && empty_node.location.width.is_none()
                                && kept_node.location.width.is_none())
                            || (*flex_direction == FlexDirection::Column
                                && empty_node.location.height.is_none()
                                && kept_node.location.height.is_none()))
                        && (gap == &Length::Zero
                            || (empty_node_background.is_none()
                                || empty_node_background == parent_node_background))
                    {
                        let width = empty_node
                            .location
                            .width
                            .as_ref()
                            .map(Cow::Borrowed)
                            .unwrap_or_else(|| {
                                Cow::Owned(
                                    empty_node.location.padding[1].clone()
                                        + empty_node.location.padding[3].clone(),
                                )
                            });
                        let height = empty_node
                            .location
                            .height
                            .as_ref()
                            .map(Cow::Borrowed)
                            .unwrap_or_else(|| {
                                Cow::Owned(
                                    empty_node.location.padding[0].clone()
                                        + empty_node.location.padding[2].clone(),
                                )
                            });
                        if (*flex_direction == FlexDirection::Row
                            && height.as_ref() == &Length::Zero)
                            || (*flex_direction == FlexDirection::Column
                                && width.as_ref() == &Length::Zero)
                        {
                            match flex_direction {
                                FlexDirection::Row => {
                                    let width = width.into_owned() + gap.clone();
                                    if let Some(k_width) = kept_node.location.width.take() {
                                        kept_node.location.width = Some(width.clone() + k_width);
                                    }
                                    let i = if direction_forwards { 3 } else { 1 };
                                    kept_node.location.padding[i] = std::mem::replace(
                                        &mut kept_node.location.padding[i],
                                        Length::Zero,
                                    ) + width;
                                }
                                FlexDirection::Column => {
                                    let height = height.into_owned() + gap.clone();
                                    if let Some(k_height) = kept_node.location.height.take() {
                                        kept_node.location.height = Some(height.clone() + k_height);
                                    }
                                    let i = if direction_forwards { 0 } else { 2 };
                                    kept_node.location.padding[i] = std::mem::replace(
                                        &mut kept_node.location.padding[i],
                                        Length::Zero,
                                    ) + height;
                                }
                            }
                            match direction_forwards {
                                true => {
                                    children.remove(0);
                                }
                                false => {
                                    children.pop();
                                }
                            }
                            mutated = true
                        }
                    }
                }
            }
        }
    });

    mutated
}
