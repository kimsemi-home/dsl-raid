use anyhow::{anyhow, bail, Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use dslraid_analyzer::{validate_core_ir, ValidateOptions, ValidationReport};
use dslraid_codegen::{generate_code, project_view, render_svg, CodegenTarget};
use dslraid_core::{
    load_core_ir, sha256_json, validate_json_schema, CORE_SCHEMA_PATH, VALIDATION_SCHEMA_PATH,
    VIEW_SCHEMA_PATH,
};
use serde_json::Value;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

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
    },
    Diff {
        base: PathBuf,
        head: PathBuf,
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
enum ArtifactCommand {
    Verify {
        input: PathBuf,
        #[arg(long)]
        lock: Option<PathBuf>,
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
        Command::Validate {
            input,
            schema,
            format,
            deny,
        } => validate(&input, &schema, format, deny),
        Command::Schema { command } => match command {
            SchemaCommand::Validate { schema, input } => schema_validate(&schema, &input),
        },
        Command::Quality => quality(),
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
        } => compose(&input, composition.as_deref(), &materialize),
        Command::Diff { base, head } => diff(&base, &head),
        Command::Artifact { command } => match command {
            ArtifactCommand::Verify { input, lock } => artifact_verify(&input, lock.as_deref()),
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

fn quality() -> Result<()> {
    check_json_syntax("schemas")?;
    check_json_syntax("examples")?;
    schema_validate(
        Path::new(CORE_SCHEMA_PATH),
        Path::new("examples/runscope/runscope.raid.json"),
    )?;
    schema_validate(
        Path::new("schemas/dslraid-assertion.schema.json"),
        Path::new("examples/runscope/runscope.assertions.json"),
    )?;
    schema_validate(
        Path::new(VALIDATION_SCHEMA_PATH),
        Path::new("examples/runscope/runscope.validation.json"),
    )?;
    schema_validate(
        Path::new("schemas/dslraid-lock.schema.json"),
        Path::new("examples/runscope/runscope.lock.json"),
    )?;
    schema_validate(
        Path::new("schemas/dslraid-annotation.schema.json"),
        Path::new("examples/runscope/runscope.annotations.json"),
    )?;
    schema_validate(
        Path::new("schemas/dslraid-sourcemap.schema.json"),
        Path::new("examples/runscope/runscope.sourcemap.json"),
    )?;
    schema_validate(
        Path::new("schemas/dslraid-trace.schema.json"),
        Path::new("examples/runscope/run-001.trace.json"),
    )?;
    schema_validate(
        Path::new("schemas/dslraid-coverage.schema.json"),
        Path::new("examples/runscope/run-001.coverage.json"),
    )?;

    let input = Path::new("examples/runscope/runscope.raid.json");
    let ir = load_core_ir(input)?;
    let report = validation_report(&ir, input, "quality", Vec::new())?;
    if !report.is_success(&[]) {
        print_report_text(&report);
        bail!("semantic quality failed");
    }
    let view = project_view(&ir, Some("view:runtime"), input.display().to_string())?;
    let view_path = std::env::temp_dir().join(format!("dslraid-view-{}.json", std::process::id()));
    write_bytes(&view_path, serde_json::to_string_pretty(&view)?.as_bytes())?;
    schema_validate(Path::new(VIEW_SCHEMA_PATH), &view_path)?;
    fs::remove_file(&view_path).ok();

    let svg = render_svg(&view);
    if !svg.contains("<svg") || svg.len() < 200 {
        bail!("rendered SVG is unexpectedly empty");
    }
    for target in [
        CliCodegenTarget::Rust,
        CliCodegenTarget::Go,
        CliCodegenTarget::Typescript,
    ] {
        let generated = generate_code(&ir, target.into())?;
        if generated.trim().is_empty() {
            bail!("empty codegen output for {target:?}");
        }
    }
    println!("quality ok");
    Ok(())
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

fn compose(input: &Path, composition: Option<&str>, materialize: &str) -> Result<()> {
    let ir = load_core_ir(input)?;
    let selected = composition
        .and_then(|id| ir.compositions.iter().find(|item| item.id == id))
        .or_else(|| ir.compositions.first());
    match selected {
        Some(composition) => {
            let state_space: usize = composition
                .inputs
                .iter()
                .filter_map(|id| ir.find_fsm(id))
                .map(|fsm| fsm.states.len())
                .product();
            println!(
                "composition {} kind={} mode={} lazy_state_space={}",
                composition.id, composition.kind, materialize, state_space
            );
        }
        None => println!("no compositions defined; nothing to compose"),
    }
    Ok(())
}

fn diff(base: &Path, head: &Path) -> Result<()> {
    let base = load_core_ir(base)?;
    let head = load_core_ir(head)?;
    let base_subjects = base.semantic_subjects();
    let head_subjects = head.semantic_subjects();
    for added in head_subjects.difference(&base_subjects) {
        println!("+ {added}");
    }
    for removed in base_subjects.difference(&head_subjects) {
        println!("- {removed}");
    }
    Ok(())
}

fn artifact_verify(input: &Path, _lock: Option<&Path>) -> Result<()> {
    let ir = load_core_ir(input)?;
    let mut errors = Vec::new();
    for artifact in &ir.artifacts {
        if artifact.kind == "generated" {
            match artifact.generated_by.as_deref() {
                Some(derivation) if ir.derivation_by_id(derivation).is_some() => {}
                Some(derivation) => errors.push(format!(
                    "{} references unknown derivation {}",
                    artifact.id, derivation
                )),
                None => errors.push(format!(
                    "{} is generated but has no generated_by",
                    artifact.id
                )),
            }
        }
    }
    if errors.is_empty() {
        println!("artifact provenance ok");
        Ok(())
    } else {
        for error in errors {
            println!("{error}");
        }
        bail!("artifact verification failed")
    }
}

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

fn check_json_syntax(path: impl AsRef<Path>) -> Result<()> {
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
    {
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
            let _: Value = serde_json::from_slice(
                &fs::read(path).with_context(|| format!("read {}", path.display()))?,
            )
            .with_context(|| format!("parse {}", path.display()))?;
        }
    }
    Ok(())
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
