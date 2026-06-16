use clap::Command;
use std::fmt::Write;

pub(super) fn push_command(out: &mut String, mut command: Command, path: String) {
    let about = command.get_about().map(ToString::to_string);
    let usage = command.render_usage().to_string();
    let args = command
        .get_arguments()
        .map(super::cli_arg::row)
        .collect::<Vec<_>>();
    let subcommands = command.get_subcommands().cloned().collect::<Vec<Command>>();
    write_header(out, &path, about.as_deref(), &usage, &args, &subcommands);
    for subcommand in subcommands {
        let subpath = format!("{path} {}", subcommand.get_name());
        push_command(out, subcommand, subpath);
    }
}

fn write_header(
    out: &mut String,
    path: &str,
    about: Option<&str>,
    usage: &str,
    args: &[String],
    subcommands: &[Command],
) {
    let _ = writeln!(out, "## `{path}`\n");
    if let Some(about) = about {
        let _ = writeln!(out, "{}\n", about.trim());
    }
    let _ = writeln!(out, "```text\n{}\n```\n", usage.trim());
    write_args(out, args);
    write_subcommands(out, subcommands);
}

fn write_args(out: &mut String, args: &[String]) {
    if args.is_empty() {
        return;
    }
    out.push_str("| Argument | Required | Help |\n| --- | --- | --- |\n");
    for row in args {
        out.push_str(row);
    }
    out.push('\n');
}

fn write_subcommands(out: &mut String, subcommands: &[Command]) {
    if subcommands.is_empty() {
        return;
    }
    let names = subcommands
        .iter()
        .map(|command| format!("`{}`", command.get_name()))
        .collect::<Vec<_>>()
        .join(", ");
    let _ = writeln!(out, "Subcommands: {names}\n");
}
