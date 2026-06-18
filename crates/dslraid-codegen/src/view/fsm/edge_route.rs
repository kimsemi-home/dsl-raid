use crate::view::{Point, ViewNode};

pub(crate) fn route(from: &ViewNode, to: &ViewNode) -> Vec<Point> {
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
