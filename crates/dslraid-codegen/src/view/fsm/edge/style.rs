use dslraid_core::Transition;

use crate::view::StyleToken;

pub(super) fn edge_style(transition: &Transition, diagnostic: Option<&str>) -> StyleToken {
    StyleToken {
        tone: tone(transition, diagnostic).to_string(),
        emphasis: "normal".to_string(),
    }
}

fn tone<'a>(transition: &'a Transition, diagnostic: Option<&'a str>) -> &'a str {
    diagnostic.unwrap_or(if transition.requires.is_empty() {
        "default"
    } else {
        "warning"
    })
}
