use std::fmt::Write;

use crate::escape::html;
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

fn write_badges(svg: &mut String, node: &ViewNode) {
    let mut badge_x = node.x + 14.0;
    for badge in node.badges.iter().take(3) {
        let badge_width = 16.0 + badge.len() as f64 * 7.0;
        write_badge(svg, node, badge, badge_x, badge_width);
        badge_x += badge_width + 6.0;
    }
}

fn write_badge(svg: &mut String, node: &ViewNode, badge: &str, x: f64, width: f64) {
    writeln!(
        svg,
        r##"<rect x="{}" y="{}" width="{}" height="18" rx="6" fill="#e2e8f0"/><text x="{}" y="{}" fill="#334155" font-family="Inter, system-ui, sans-serif" font-size="10">{}</text>"##,
        x,
        node.y + 34.0,
        width,
        x + 8.0,
        node.y + 47.0,
        html(badge)
    )
    .unwrap();
}
