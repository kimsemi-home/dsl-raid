use super::model::TupleNode;
use super::tuple::local;
use crate::view::{StyleToken, ViewNode};

const WIDTH: f64 = 280.0;
const HEIGHT: f64 = 76.0;
const COL_GAP: f64 = 330.0;
const ROW_GAP: f64 = 132.0;

pub(crate) fn nodes(items: &[TupleNode]) -> Vec<ViewNode> {
    let mut items = items.to_vec();
    items.sort_by(|left, right| left.subject.cmp(&right.subject));
    items
        .iter()
        .enumerate()
        .map(|(index, item)| node(item, index))
        .collect()
}

fn node(item: &TupleNode, index: usize) -> ViewNode {
    ViewNode {
        id: format!("layout:{}", local(&item.subject)),
        subject: item.subject.clone(),
        x: 70.0 + (index as f64 % 3.0) * COL_GAP,
        y: 90.0 + (index as f64 / 3.0).floor() * ROW_GAP,
        width: WIDTH,
        height: HEIGHT,
        label: label(item),
        badges: badges(item),
        style: Some(style(item)),
    }
}

fn label(item: &TupleNode) -> String {
    item.members
        .iter()
        .map(|member| local(member).replace('.', "="))
        .collect::<Vec<_>>()
        .join(" / ")
}

fn badges(item: &TupleNode) -> Vec<String> {
    let mut badges = vec!["tuple".to_string()];
    if item.initial {
        badges.push("initial".to_string());
    }
    if item.terminal {
        badges.push("terminal".to_string());
    }
    badges
}

fn style(item: &TupleNode) -> StyleToken {
    StyleToken {
        tone: if item.terminal { "success" } else { "default" }.to_string(),
        emphasis: if item.initial || item.terminal {
            "strong"
        } else {
            "normal"
        }
        .to_string(),
    }
}
