use clap::Arg;

pub(super) fn row(arg: &Arg) -> String {
    format!(
        "| {} | {} | {} |\n",
        escape(&label(arg)),
        if arg.is_required_set() { "yes" } else { "no" },
        escape(&help(arg))
    )
}

fn label(arg: &Arg) -> String {
    let mut labels = Vec::new();
    if let Some(short) = arg.get_short() {
        labels.push(format!("`-{short}`"));
    }
    if let Some(long) = arg.get_long() {
        labels.push(format!("`--{long}`"));
    }
    if labels.is_empty() {
        labels.push(format!("`{}`", arg.get_id()));
    }
    labels.join(", ")
}

fn help(arg: &Arg) -> String {
    arg.get_help().map(ToString::to_string).unwrap_or_default()
}

fn escape(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
}
