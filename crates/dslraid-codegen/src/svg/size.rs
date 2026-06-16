use crate::view::ViewModel;

pub(crate) fn canvas_size(view: &ViewModel) -> (f64, f64) {
    let width = view
        .nodes
        .iter()
        .map(|node| node.x + node.width)
        .fold(800.0, f64::max)
        + 80.0;
    let height = view
        .nodes
        .iter()
        .map(|node| node.y + node.height)
        .fold(480.0, f64::max)
        + 80.0;
    (width, height)
}
