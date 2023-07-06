use std::borrow::Cow;

use figma_schema::TextCase;

use crate::intermediate_node::IntermediateNode;

#[derive(Default)]
pub struct InheritedProperties<'a> {
    // align-items - not inherited
    // flex-direction - not inherited
    // gap - not inherited
    // justify-content - not inherited

    // padding - not inherited
    // align-self - not inherited
    // flex-grow - not inherited
    // inset - not inherited
    // height - not inherited
    // width - not inherited
    pub color: Option<Cow<'a, str>>,
    pub fill: Option<Cow<'a, str>>,
    pub font: Option<Cow<'a, str>>,
    // opacity - not inherited
    pub preserve_whitespace: bool,
    pub text_tranform: Option<TextCase>,
    // text-decoration-line - not inherited

    // background - not inherited
    // border-radius - not inherited
    // box-shadow - not inherited
    // outline/border - not inherited
    // outline-offset - not inherited
}

impl<'a> InheritedProperties<'a> {
    pub fn inherit<'b>(node: &'b IntermediateNode<'b>, inherited: &'a Self) -> Self {
        Self {
            color: node
                .appearance
                .color
                .clone()
                .map(Cow::Owned)
                .or_else(|| inherited.color.as_deref().map(Cow::Borrowed)),
            fill: node
                .appearance
                .fill
                .clone()
                .map(Cow::Owned)
                .or_else(|| inherited.fill.as_deref().map(Cow::Borrowed)),
            font: node
                .appearance
                .font
                .clone()
                .map(Cow::Owned)
                .or_else(|| inherited.font.as_deref().map(Cow::Borrowed)),
            preserve_whitespace: node.appearance.preserve_whitespace
                || inherited.preserve_whitespace,
            text_tranform: node.appearance.text_tranform.or(inherited.text_tranform),
        }
    }
}
