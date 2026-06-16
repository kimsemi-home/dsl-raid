use crate::view::InspectorRow;

pub(crate) fn member_rows(members: &[String]) -> Vec<InspectorRow> {
    members
        .iter()
        .map(|member| row("Member", member, Some(member.clone())))
        .collect()
}

pub(crate) fn row(label: &str, value: &str, subject: Option<String>) -> InspectorRow {
    InspectorRow {
        label: label.to_string(),
        value: value.to_string(),
        subject,
    }
}
