pub(super) const WIDTH: f64 = 168.0;
pub(super) const HEIGHT: f64 = 58.0;

const COL_GAP: f64 = 230.0;
const ROW_GAP: f64 = 150.0;

pub(super) fn x(index: usize) -> f64 {
    80.0 + (index as f64 % 3.0) * COL_GAP
}

pub(super) fn y(index: usize) -> f64 {
    90.0 + (index as f64 / 3.0).floor() * ROW_GAP
}
