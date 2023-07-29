use std::borrow::Cow;

use crate::{
    intermediate_node::{
        CSSVariablesMap, FlexContainer, FlexDirection, FrameAppearance, IntermediateNode,
        IntermediateNodeType, Length, Location,
    },
    InheritedProperties,
};

use super::recursive_visitor_mut;

/**
Look for inbetween nodes (excluding absolutely positioned children)
of flex nodes that are purely sizing on the primary axis.  If these
nodes have a consistent size they can be removed and the size of
the node can be used as the parent's gap.
*/
pub fn collapse_to_gap(node: &mut IntermediateNode, _css_variables: &mut CSSVariablesMap) -> bool {
    recursive_visitor_mut(
        node,
        &InheritedProperties::default(),
        &mut |parent, _inherited_properties| {
            if let IntermediateNode {
                flex_container:
                    Some(FlexContainer {
                        direction,
                        gap: parent_gap,
                        ..
                    }),
                node_type: IntermediateNodeType::Frame { children },
                ..
            } = parent
            {
                if parent_gap != &Length::Zero {
                    return false;
                }
                let static_children = children
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| c.location.inset.is_none())
                    .collect::<Vec<_>>();

                if static_children.len() % 2 != 1 {
                    return false;
                }

                let mut potential_gaps =
                    static_children.iter().map(|(_, pg)| pg).skip(1).step_by(2);
                let potential_gap_indexes = static_children
                    .iter()
                    .map(|(i, _)| i)
                    .skip(1)
                    .step_by(2)
                    .cloned()
                    .collect::<Vec<_>>();

                if let Some(IntermediateNode {
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
                }) = potential_gaps.next()
                {
                    if grand_children.is_empty()
                        && flex_grow.unwrap_or(0.0) == 0.0
                        && potential_gaps.all(|pg| match pg {
                            IntermediateNode {
                                node_type:
                                    IntermediateNodeType::Frame {
                                        children: other_grand_children,
                                    },
                                ..
                            } => {
                                &pg.location.padding == padding
                                    && pg.location.flex_grow.unwrap_or(0.0) == 0.0
                                    && &pg.location.height == height
                                    && &pg.location.width == width
                                    && other_grand_children.is_empty()
                            }
                            _ => false,
                        })
                    {
                        let width = width
                            .as_ref()
                            .map(Cow::Borrowed)
                            .unwrap_or_else(|| Cow::Owned(padding[1].clone() + padding[3].clone()));
                        let height = height
                            .as_ref()
                            .map(Cow::Borrowed)
                            .unwrap_or_else(|| Cow::Owned(padding[0].clone() + padding[2].clone()));
                        let (primary_axis_size, counter_axis_size) = match direction {
                            FlexDirection::Row => (width, height),
                            FlexDirection::Column => (height, width),
                        };

                        if counter_axis_size.as_ref() == &Length::Zero {
                            *parent_gap = primary_axis_size.into_owned();
                            let mut i: usize = 0;
                            children.retain(|_| {
                                i += 1;
                                potential_gap_indexes.binary_search(&(i - 1)).is_err()
                            });
                            return true;
                        }
                    }
                }
            }
            false
        },
    )
}
