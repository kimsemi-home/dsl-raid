use super::model::TupleEdge;
use super::tuple::local;
use crate::view::{Point, StyleToken, ViewEdge, ViewNode};

pub(crate) fn edges(items: &[TupleEdge], nodes: &[ViewNode]) -> Vec<ViewEdge> {
    let mut items = items.to_vec();
    items.sort_by(|left, right| left.subject.cmp(&right.subject));
    items.iter().filter_map(|item| edge(item, nodes)).collect()
}

fn edge(item: &TupleEdge, nodes: &[ViewNode]) -> Option<ViewEdge> {
    let from = nodes.iter().find(|node| node.subject == item.from)?;
    let to = nodes.iter().find(|node| node.subject == item.to)?;
    Some(ViewEdge {
        id: format!("layout:{}", local(&item.subject)),
        subject: item.subject.clone(),
        from: from.id.clone(),
        to: to.id.clone(),
        label: Some(label(item)),
        route: route(from, to),
        style: Some(StyleToken {
            tone: "default".to_string(),
            emphasis: "normal".to_string(),
        }),
    })
}

fn route(from: &ViewNode, to: &ViewNode) -> Vec<Point> {
    vec![
        Point {
            x: from.x + from.width,
            y: from.y + from.height / 2.0,
        },
        Point {
            x: to.x,
            y: to.y + to.height / 2.0,
        },
    ]
}

fn label(item: &TupleEdge) -> String {
    item.event
        .as_deref()
        .or_else(|| item.members.first().map(String::as_str))
        .map(local)
        .unwrap_or("epsilon")
        .to_string()
}
