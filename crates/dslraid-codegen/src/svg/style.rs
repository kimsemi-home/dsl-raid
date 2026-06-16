use crate::view::StyleToken;

pub(crate) fn stroke(style: Option<&StyleToken>) -> &'static str {
    match style.map(|style| style.tone.as_str()) {
        Some("success") => "#0f766e",
        Some("warning") => "#b45309",
        Some("danger") => "#b91c1c",
        _ => "#334155",
    }
}

pub(crate) fn stroke_width(style: Option<&StyleToken>) -> u8 {
    if style.map(|style| style.emphasis.as_str()) == Some("strong") {
        3
    } else {
        2
    }
}
