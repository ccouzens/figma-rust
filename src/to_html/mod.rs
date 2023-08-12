use figma_html::{
    figma_node_to_intermediate_node, file_collect_css_variables, find_figma_node_by_id,
    intermediate_node_to_html_writer, mutator,
};

use anyhow::{Context, Result};
use std::io::Write;

pub fn main(
    file: &figma_schema::File,
    stdout: &mut impl Write,
    _stderr: &mut impl Write,
    node_id: &str,
) -> Result<()> {
    let (body, _) = find_figma_node_by_id(file, node_id)
        .with_context(|| format!("Failed to find node with id {}", node_id))?;

    let mut css_variables = file_collect_css_variables(file);

    let mut node = figma_node_to_intermediate_node(body, &mut css_variables);
    while mutator::combine_parent_child(&mut node, &mut css_variables)
        || mutator::collapse_to_padding(&mut node, &mut css_variables)
        || mutator::collapse_to_gap(&mut node, &mut css_variables)
        || mutator::drop_empty_absolute_frames(&mut node, &mut css_variables)
        || mutator::elevate_frame_appearance_properties(&mut node, &mut css_variables)
    {}

    intermediate_node_to_html_writer(stdout, &node, &css_variables, "")
        .context("Failed to write HTML to stdout")
}
