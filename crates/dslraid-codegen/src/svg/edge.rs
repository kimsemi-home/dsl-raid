use std::fmt::Write;

use crate::escape::html;
use crate::view::{Point, ViewEdge, ViewModel};

pub(crate) fn write_edges(svg: &mut String, view: &ViewModel) {
    for edge in &view.edges {
        if edge.route.len() < 2 {
            continue;
        }
        write_edge(svg, edge);
        if let Some(label) = &edge.label {
            write_label(svg, edge, label);
        }
    }
}

fn write_edge(svg: &mut String, edge: &ViewEdge) {
    writeln!(
        svg,
        r##"<path id="{}" d="{}" fill="none" stroke="#4b5563" stroke-width="2" marker-end="url(#arrow)"/>"##,
        html(&edge.id),
        path(&edge.route)
    )
    .unwrap();
}

fn write_label(svg: &mut String, edge: &ViewEdge, label: &str) {
    let a = &edge.route[0];
    let b = edge.route.last().unwrap();
    writeln!(
        svg,
        r##"<text x="{}" y="{}" fill="#334155" font-family="Inter, system-ui, sans-serif" font-size="12">{}</text>"##,
        (a.x + b.x) / 2.0,
        (a.y + b.y) / 2.0 - 8.0,
        html(label)
    )
    .unwrap();
}

fn path(route: &[Point]) -> String {
    let mut path = String::new();
    for (idx, point) in route.iter().enumerate() {
        if idx == 0 {
            write!(path, "M{} {}", point.x, point.y).unwrap();
        } else {
            write!(path, " L{} {}", point.x, point.y).unwrap();
        }
    }
    path
}
