pub(super) fn header(out: &mut String, columns: &[&str]) {
    out.push('|');
    for column in columns {
        out.push(' ');
        out.push_str(column);
        out.push_str(" |");
    }
    out.push('\n');
    out.push('|');
    for _ in columns {
        out.push_str(" --- |");
    }
    out.push('\n');
}

pub(super) fn row(out: &mut String, values: &[String]) {
    out.push('|');
    for value in values {
        out.push(' ');
        out.push_str(&cell(value));
        out.push_str(" |");
    }
    out.push('\n');
}

pub(super) fn cell(value: &str) -> String {
    value.replace('|', "\\|").replace('\n', " ")
}
