pub(super) fn tuple_subject(composition_id: &str, members: &[String]) -> String {
    format!(
        "state_tuple:{}.{}",
        composition_id.trim_start_matches("composition:"),
        members
            .iter()
            .map(|member| member.replace([':', '.'], "_"))
            .collect::<Vec<_>>()
            .join("__")
    )
}
