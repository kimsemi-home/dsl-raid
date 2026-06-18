use std::fmt::Write;

use crate::escape::html;
use crate::view::ViewNode;

pub(crate) fn write_badges(svg: &mut String, node: &ViewNode) {
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
