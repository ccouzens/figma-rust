use std::borrow::Cow;

use crate::{
    intermediate_node::{
        CSSVariablesMap, FlexContainer, FlexDirection, FrameAppearance, IntermediateNode,
        IntermediateNodeType, JustifyContent, Length, Location,
    },
    InheritedProperties,
};

use super::recursive_visitor_mut;

/**
Look for first and last children nodes (excluding absolutely positioned
children) of flex nodes that are purely sizing on the primary axis. These nodes
can be removed and the size of the node can be added to the parent's
corresponding padding.
 */
pub fn collapse_to_padding(
    node: &mut IntermediateNode,
    _css_variables: &mut CSSVariablesMap,
) -> bool {
    recursive_visitor_mut(
        node,
        &InheritedProperties::default(),
        &mut |parent, _inherited_properties| {
            if let IntermediateNode {
                flex_container:
                    Some(FlexContainer {
                        direction,
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
                location:
                    Location {
                        padding: parent_padding,
                        ..
                    },
                node_type: IntermediateNodeType::Frame { children },
                ..
            } = parent
            {
                for &direction_forwards in [false, true].iter() {
                    let padding_side = &mut parent_padding[match (&direction, direction_forwards) {
                        (FlexDirection::Row, true) => 1,
                        (FlexDirection::Row, false) => 3,
                        (FlexDirection::Column, true) => 0,
                        (FlexDirection::Column, false) => 2,
                    }];

                    let mut static_children = children
                        .iter()
                        .enumerate()
                        .filter(|(_, c)| c.location.inset.is_none());

                    if let Some((
                        index,
                        IntermediateNode {
                            location:
                                Location {
                                    padding,
                                    flex_grow,
                                    height,
                                    width,
                                    ..
                                },
                            frame_appearance:
                                FrameAppearance {
                                    background: None,
                                    box_shadow: None,
                                    stroke: None,
                                    ..
                                },
                            node_type:
                                IntermediateNodeType::Frame {
                                    children: grand_children,
                                },
                            href: None,
                            ..
                        },
                    )) = match direction_forwards {
                        true => static_children.next(),
                        false => static_children.next_back(),
                    } {
                        if grand_children.is_empty() && flex_grow.unwrap_or(0.0) == 0.0 {
                            let width = width.as_ref().map(Cow::Borrowed).unwrap_or_else(|| {
                                Cow::Owned(padding[1].clone() + padding[3].clone())
                            });
                            let height = height.as_ref().map(Cow::Borrowed).unwrap_or_else(|| {
                                Cow::Owned(padding[0].clone() + padding[2].clone())
                            });
                            let (primary_axis_size, counter_axis_size) = match direction {
                                FlexDirection::Row => (width, height),
                                FlexDirection::Column => (height, width),
                            };

                            if counter_axis_size.as_ref() == &Length::Zero {
                                *padding_side = padding_side.clone()
                                    + gap.clone()
                                    + primary_axis_size.into_owned();
                                children.remove(index);
                                return true;
                            }
                        }
                    }
                }
            }
            false
        },
    )
}
