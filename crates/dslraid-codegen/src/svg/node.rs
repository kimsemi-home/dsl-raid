use std::fmt::Write;

use crate::escape::html;
use crate::svg::badge::write_badges;
use crate::svg::style::{stroke, stroke_width};
use crate::view::{ViewModel, ViewNode};

pub(crate) fn write_nodes(svg: &mut String, view: &ViewModel) {
    for node in &view.nodes {
        write_node(svg, node);
    }
}

fn write_node(svg: &mut String, node: &ViewNode) {
    write_box(svg, node);
    write_title(svg, node);
    write_badges(svg, node);
    svg.push_str("</g>");
}

fn write_box(svg: &mut String, node: &ViewNode) {
    let style = node.style.as_ref();
    writeln!(
        svg,
        r##"<g id="{}"><rect x="{}" y="{}" width="{}" height="{}" rx="8" fill="#ffffff" stroke="{}" stroke-width="{}"/>"##,
        html(&node.id),
        node.x,
        node.y,
        node.width,
        node.height,
        stroke(style),
        stroke_width(style)
    )
    .unwrap();
}

fn write_title(svg: &mut String, node: &ViewNode) {
    writeln!(
        svg,
        r##"<text x="{}" y="{}" fill="#0f172a" font-family="Inter, system-ui, sans-serif" font-size="15" font-weight="700">{}</text>"##,
        node.x + 14.0,
        node.y + 24.0,
        html(&node.label)
    )
    .unwrap();
}
