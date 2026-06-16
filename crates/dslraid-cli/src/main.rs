use anyhow::{anyhow, bail, Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use dslraid_analyzer::{validate_core_ir, ValidateOptions, ValidationReport};
use dslraid_codegen::{generate_code, project_view, render_svg, CodegenTarget};
use dslraid_core::{load_core_ir, sha256_json, validate_json_schema, CORE_SCHEMA_PATH};
use serde_json::Value;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

mod commands;

#[derive(Debug, Parser)]
#[command(
    name = "dslraid",
    version,
    about = "Executable architecture IR browser CLI"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Init {
        #[arg(default_value = ".dslraid.json")]
        out: PathBuf,
    },
    Normalize {
        input: PathBuf,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Migrate {
        input: PathBuf,
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Validate {
        input: PathBuf,
        #[arg(long, default_value = CORE_SCHEMA_PATH)]
        schema: PathBuf,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
        #[arg(long = "deny")]
        deny: Vec<String>,
    },
    Schema {
        #[command(subcommand)]
        command: SchemaCommand,
    },
    Quality,
    Golden {
        #[command(subcommand)]
        command: GoldenCommand,
    },
    Project {
        input: PathBuf,
        #[arg(long)]
        projection: Option<String>,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Render {
        input: PathBuf,
        #[arg(long)]
        projection: Option<String>,
        #[arg(long, value_enum, default_value_t = RenderFormat::Svg)]
        format: RenderFormat,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Codegen {
        input: PathBuf,
        #[arg(long, value_enum)]
        target: CliCodegenTarget,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Compose {
        input: PathBuf,
        #[arg(long)]
        composition: Option<String>,
        #[arg(long, default_value = "diagnostics-only")]
        materialize: String,
        #[arg(long, default_value_t = 5000)]
        limit: usize,
        #[arg(long)]
        focus: Option<String>,
        #[arg(long, default_value_t = 1)]
        depth: usize,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Diff {
        base: PathBuf,
        head: PathBuf,
        #[arg(long, value_enum, default_value_t = DiffFormat::Text)]
        format: DiffFormat,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Query {
        input: PathBuf,
        expression: String,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    Trace {
        #[command(subcommand)]
        command: TraceCommand,
    },
    Coverage {
        #[command(subcommand)]
        command: CoverageCommand,
    },
    Artifact {
        #[command(subcommand)]
        command: ArtifactCommand,
    },
    Compat {
        #[command(subcommand)]
        command: CompatCommand,
    },
    Export {
        #[arg(value_enum)]
        target: CliExportTarget,
        input: PathBuf,
        #[arg(long)]
        out: Option<PathBuf>,
    },
}

#[derive(Debug, Subcommand)]
enum SchemaCommand {
    Validate { schema: PathBuf, input: PathBuf },
}

#[derive(Debug, Subcommand)]
enum GoldenCommand {
    Check { path: PathBuf },
    Update { path: PathBuf },
}

#[derive(Debug, Subcommand)]
enum TraceCommand {
    Import {
        input: PathBuf,
        #[arg(long)]
        design_ir: Option<PathBuf>,
        #[arg(long)]
        run_id: Option<String>,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Check {
        trace: PathBuf,
        #[arg(long)]
        design_ir: PathBuf,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
}

#[derive(Debug, Subcommand)]
enum CoverageCommand {
    Build {
        #[arg(long)]
        trace: PathBuf,
        #[arg(long)]
        design_ir: PathBuf,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    Check {
        coverage: PathBuf,
        #[arg(long)]
        design_ir: PathBuf,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
}

#[derive(Debug, Subcommand)]
enum ArtifactCommand {
    Verify {
        input: PathBuf,
        #[arg(long)]
        lock: Option<PathBuf>,
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
}

#[derive(Debug, Subcommand)]
enum CompatCommand {
    Check { input: PathBuf },
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum DiffFormat {
    Text,
    Json,
    Markdown,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum RenderFormat {
    Svg,
    Json,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliCodegenTarget {
    Rust,
    Go,
    Typescript,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum CliExportTarget {
    Mermaid,
    Dot,
    Json,
    Svg,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Init { out } => init_project(&out),
        Command::Normalize { input, out } => normalize(&input, out.as_deref()),
        Command::Migrate {
            input,
            from,
            to,
            out,
        } => migrate(&input, &from, &to, out.as_deref()),
        Command::Validate {
            input,
            schema,
            format,
            deny,
        } => validate(&input, &schema, format, deny),
        Command::Schema { command } => match command {
            SchemaCommand::Validate { schema, input } => schema_validate(&schema, &input),
        },
        Command::Quality => commands::quality::run(),
        Command::Golden { command } => match command {
            GoldenCommand::Check { path } => golden_check(&path),
            GoldenCommand::Update { path } => golden_update(&path),
        },
        Command::Project {
            input,
            projection,
            out,
        } => project(&input, projection.as_deref(), out.as_deref()),
        Command::Render {
            input,
            projection,
            format,
            out,
        } => render(&input, projection.as_deref(), format, out.as_deref()),
        Command::Codegen { input, target, out } => codegen(&input, target, out.as_deref()),
        Command::Compose {
            input,
            composition,
            materialize,
            limit,
            focus,
            depth,
            format,
            out,
        } => commands::compose::run(commands::compose::RunOptions {
            input: &input,
            composition: composition.as_deref(),
            materialize: &materialize,
            limit,
            focus: focus.as_deref(),
            depth,
            format,
            out: out.as_deref(),
        }),
        Command::Diff {
            base,
            head,
            format,
            out,
        } => commands::diff::run(&base, &head, format, out.as_deref()),
        Command::Query {
            input,
            expression,
            format,
        } => commands::query::run(&input, &expression, format),
        Command::Trace { command } => match command {
            TraceCommand::Import {
                input,
                design_ir,
                run_id,
                out,
            } => commands::trace::import(
                &input,
                design_ir.as_deref(),
                run_id.as_deref(),
                out.as_deref(),
            ),
            TraceCommand::Check {
                trace,
                design_ir,
                format,
            } => commands::trace::check(&trace, &design_ir, format),
        },
        Command::Coverage { command } => match command {
            CoverageCommand::Build {
                trace,
                design_ir,
                out,
            } => commands::coverage::build(&trace, &design_ir, out.as_deref()),
            CoverageCommand::Check {
                coverage,
                design_ir,
                format,
            } => commands::coverage::check(&coverage, &design_ir, format),
        },
        Command::Artifact { command } => match command {
            ArtifactCommand::Verify {
                input,
                lock,
                format,
            } => commands::artifact::verify(&input, lock.as_deref(), format),
        },
        Command::Compat { command } => match command {
            CompatCommand::Check { input } => compat_check(&input),
        },
        Command::Export { target, input, out } => export(&input, target, out.as_deref()),
    }
}

fn init_project(out: &Path) -> Result<()> {
    if out.exists() {
        bail!("{} already exists", out.display());
    }
    let template = serde_json::json!({
        "ir_version": "0.1.0",
        "project": {
            "id": "dslraid-project",
            "name": "DSLRaid Project",
            "visibility": "public"
        },
        "contexts": [],
        "requirements": [],
        "capabilities": [],
        "policies": [],
        "commands": [],
        "fsms": [],
        "compositions": [],
        "projections": [],
        "derivations": [],
        "artifacts": [],
        "diagnostics": []
    });
    write_bytes(out, serde_json::to_string_pretty(&template)?.as_bytes())?;
    println!("created {}", out.display());
    Ok(())
}

fn normalize(input: &Path, out: Option<&Path>) -> Result<()> {
    let ir = load_core_ir(input)?;
    let bytes = serde_json::to_vec_pretty(&ir)?;
    write_or_stdout(out, &bytes)
}

fn migrate(input: &Path, from: &str, to: &str, out: Option<&Path>) -> Result<()> {
    let mut ir = load_core_ir(input)?;
    if ir.ir_version != from {
        bail!(
            "input IR version is {}, but --from was {}",
            ir.ir_version,
            from
        );
    }
    if from != to {
        bail!("no migration rule registered for {from} -> {to}");
    }
    ir.ir_version = to.to_string();
    let bytes = serde_json::to_vec_pretty(&ir)?;
    write_or_stdout(out, &bytes)
}

fn validate(input: &Path, schema: &Path, format: OutputFormat, deny: Vec<String>) -> Result<()> {
    let schema_issues = validate_json_schema(schema, input)?;
    if !schema_issues.is_empty() {
        match format {
            OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&schema_issues)?),
            OutputFormat::Text => {
                for issue in &schema_issues {
                    println!("schema error at {}: {}", issue.instance_path, issue.message);
                }
            }
        }
        bail!(
            "schema validation failed with {} issues",
            schema_issues.len()
        );
    }

    let ir = load_core_ir(input)?;
    let report = validation_report(&ir, input, "validate", deny.clone())?;
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&report)?),
        OutputFormat::Text => print_report_text(&report),
    }
    if !report.is_success(&deny) {
        bail!("validation failed");
    }
    Ok(())
}

fn validation_report(
    ir: &dslraid_core::CoreIr,
    input: &Path,
    mode: &str,
    deny: Vec<String>,
) -> Result<ValidationReport> {
    let hash = sha256_json(ir)?;
    Ok(validate_core_ir(
        ir,
        ValidateOptions {
            source_path: input.display().to_string(),
            ir_hash: Some(hash),
            mode: mode.to_string(),
            deny,
        },
    ))
}

fn schema_validate(schema: &Path, input: &Path) -> Result<()> {
    let issues = validate_json_schema(schema, input)?;
    if issues.is_empty() {
        println!("schema ok: {}", input.display());
        Ok(())
    } else {
        for issue in &issues {
            println!("schema error at {}: {}", issue.instance_path, issue.message);
        }
        bail!("schema validation failed with {} issues", issues.len())
    }
}

fn validate_json_file(schema: &Path, input: &Path) -> Result<()> {
    let issues = validate_json_schema(schema, input)?;
    if issues.is_empty() {
        Ok(())
    } else {
        bail!(
            "{} failed schema validation with {} issues",
            input.display(),
            issues.len()
        )
    }
}

fn golden_check(path: &Path) -> Result<()> {
    if !path.exists() {
        bail!("golden path does not exist: {}", path.display());
    }
    let mut checked = 0usize;
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
    {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
            let _: Value = serde_json::from_slice(&fs::read(path)?)?;
            checked += 1;
        }
    }
    println!("golden ok: {checked} JSON fixture files checked");
    Ok(())
}

fn golden_update(path: &Path) -> Result<()> {
    if !path.exists() {
        bail!("golden path does not exist: {}", path.display());
    }
    println!(
        "golden update currently has no generated fixtures to refresh at {}",
        path.display()
    );
    Ok(())
}

fn project(input: &Path, projection: Option<&str>, out: Option<&Path>) -> Result<()> {
    let ir = load_core_ir(input)?;
    let view = project_view(&ir, projection, input.display().to_string())?;
    let bytes = serde_json::to_vec_pretty(&view)?;
    write_or_stdout(out, &bytes)
}

fn render(
    input: &Path,
    projection: Option<&str>,
    format: RenderFormat,
    out: Option<&Path>,
) -> Result<()> {
    let ir = load_core_ir(input)?;
    let view = project_view(&ir, projection, input.display().to_string())?;
    match out {
        Some(path) if path.extension().is_none() || path.is_dir() => {
            fs::create_dir_all(path)?;
            let stem = input
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or("dslraid");
            let view_path = path.join(format!("{stem}.view.json"));
            let svg_path = path.join(format!("{stem}.svg"));
            let core_path = path.join(
                input
                    .file_name()
                    .ok_or_else(|| anyhow!("input has no file name"))?,
            );
            write_bytes(&view_path, serde_json::to_string_pretty(&view)?.as_bytes())?;
            write_bytes(&svg_path, render_svg(&view).as_bytes())?;
            fs::copy(input, core_path)?;
            println!("rendered {}", path.display());
            Ok(())
        }
        Some(path) => match format {
            RenderFormat::Svg => write_bytes(path, render_svg(&view).as_bytes()),
            RenderFormat::Json => {
                write_bytes(path, serde_json::to_string_pretty(&view)?.as_bytes())
            }
        },
        None => match format {
            RenderFormat::Svg => {
                print!("{}", render_svg(&view));
                Ok(())
            }
            RenderFormat::Json => {
                println!("{}", serde_json::to_string_pretty(&view)?);
                Ok(())
            }
        },
    }
}

fn codegen(input: &Path, target: CliCodegenTarget, out: Option<&Path>) -> Result<()> {
    let ir = load_core_ir(input)?;
    let target: CodegenTarget = target.into();
    let generated = generate_code(&ir, target)?;
    match out {
        Some(path) if path.extension().is_none() || path.is_dir() => {
            fs::create_dir_all(path)?;
            let file = path.join(format!("dslraid_generated.{}", target.extension()));
            write_bytes(&file, generated.as_bytes())?;
            println!("generated {}", file.display());
            Ok(())
        }
        Some(path) => write_bytes(path, generated.as_bytes()),
        None => {
            print!("{generated}");
            Ok(())
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn compat_check(input: &Path) -> Result<()> {
    let ir = load_core_ir(input)?;
    println!(
        "compat ok: ir_version={} project={}",
        ir.ir_version, ir.project.id
    );
    Ok(())
}

fn export(input: &Path, target: CliExportTarget, out: Option<&Path>) -> Result<()> {
    match target {
        CliExportTarget::Json => normalize(input, out),
        CliExportTarget::Svg => render(input, None, RenderFormat::Svg, out),
        CliExportTarget::Mermaid => {
            let ir = load_core_ir(input)?;
            let generated = generate_code(&ir, CodegenTarget::Mermaid)?;
            write_or_stdout(out, generated.as_bytes())
        }
        CliExportTarget::Dot => {
            let ir = load_core_ir(input)?;
            let generated = generate_code(&ir, CodegenTarget::Dot)?;
            write_or_stdout(out, generated.as_bytes())
        }
    }
}

fn print_report_text(report: &ValidationReport) {
    println!("validation {}", report.summary.status);
    println!(
        "assertions: passed={} failed={} warnings={} skipped={} n/a={}",
        report.summary.assertions.passed,
        report.summary.assertions.failed,
        report.summary.assertions.warnings,
        report.summary.assertions.skipped,
        report.summary.assertions.not_applicable
    );
    for assertion in &report.assertions {
        if assertion.status != "passed" && assertion.status != "not_applicable" {
            println!(
                "{} {} {}: {}",
                assertion.severity,
                assertion.code,
                assertion.id,
                assertion.message.clone().unwrap_or_default()
            );
            if let Some(suggestion) = &assertion.suggestion {
                println!("  suggestion: {suggestion}");
            }
        }
    }
}

fn write_or_stdout(out: Option<&Path>, bytes: &[u8]) -> Result<()> {
    match out {
        Some(path) => write_bytes(path, bytes),
        None => {
            std::io::stdout().write_all(bytes)?;
            println!();
            Ok(())
        }
    }
}

fn write_bytes(path: &Path, bytes: &[u8]) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    fs::write(path, bytes).with_context(|| format!("write {}", path.display()))
}

impl From<CliCodegenTarget> for CodegenTarget {
    fn from(value: CliCodegenTarget) -> Self {
        match value {
            CliCodegenTarget::Rust => CodegenTarget::Rust,
            CliCodegenTarget::Go => CodegenTarget::Go,
            CliCodegenTarget::Typescript => CodegenTarget::TypeScript,
        }
    }
}
