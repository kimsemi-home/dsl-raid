use std::fmt::Write;

use crate::svg::edge::write_edges;
use crate::svg::node::write_nodes;
use crate::svg::size::canvas_size;
use crate::view::ViewModel;

pub fn render_svg(view: &ViewModel) -> String {
    let (max_x, max_y) = canvas_size(view);
    let mut svg = String::new();
    write_open(&mut svg, max_x, max_y);
    write_defs(&mut svg);
    write_edges(&mut svg, view);
    write_nodes(&mut svg, view);
    svg.push_str("</svg>\n");
    svg
}

fn write_open(svg: &mut String, max_x: f64, max_y: f64) {
    writeln!(
        svg,
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{max_x}" height="{max_y}" viewBox="0 0 {max_x} {max_y}" role="img" aria-label="DSLRaid projection">"#
    )
    .unwrap();
}

fn write_defs(svg: &mut String) {
    svg.push_str(
        r##"<defs><marker id="arrow" markerWidth="10" markerHeight="10" refX="8" refY="3" orient="auto"><path d="M0,0 L0,6 L9,3 z" fill="#4b5563"/></marker></defs>"##,
    );
    svg.push_str(r##"<rect width="100%" height="100%" fill="#f8fafc"/>"##);
}
