//! main.rs — Snip CLI entry point.
//! Standard CLI flags: -h/--help, -V/--version, -f/--format, -o/--output, -q/--quiet, -v/--verbose.

mod cli;
mod doctor;
mod serve;
mod update;
mod xdg;

use cli::{parse_args, print_help, CliConfig, CliError, OutputFormat, Subcommand, VERSION};
use snip::{check, emit_cursor_markdown, emit_generic_text, emit_json, Diff, Policy, Verdict};
use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process;

fn read_input(file: Option<&PathBuf>) -> Result<String, String> {
    if let Some(path) = file {
        fs::read_to_string(path).map_err(|e| format!("Failed to read {}: {}", path.display(), e))
    } else {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .map_err(|e| format!("Failed to read stdin: {}", e))?;
        Ok(buffer)
    }
}

fn write_output(content: &str, target: Option<&PathBuf>) -> Result<(), std::io::Error> {
    if let Some(path) = target {
        fs::write(path, content)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o644));
        }
        Ok(())
    } else {
        print!("{}", content);
        Ok(())
    }
}

fn run_audit(config: &CliConfig) -> Result<i32, CliError> {
    let raw_diff = read_input(config.input_file.as_ref())?;
    if config.verbose {
        eprintln!("snip: processing {} bytes of diff input", raw_diff.len());
    }

    let diff = Diff::from_raw(&raw_diff);
    let policy = Policy::default();
    let result = check(&diff, &policy);

    if !config.quiet {
        let output = match config.format {
            OutputFormat::Json => emit_json(&result),
            OutputFormat::Cursor => emit_cursor_markdown(&result),
            OutputFormat::Text => emit_generic_text(&result),
        };
        write_output(&output, config.output_file.as_ref())?;
    }

    Ok(match result.verdict {
        Verdict::Ship => 0,
        Verdict::Fix | Verdict::Block => 1,
    })
}

fn run() -> Result<i32, CliError> {
    let args: Vec<String> = env::args().collect();
    let config = parse_args(&args)?;

    match config.subcommand {
        Subcommand::Help => {
            print_help();
            Ok(0)
        }
        Subcommand::Version => {
            println!("snip {}", VERSION);
            Ok(0)
        }
        Subcommand::Doctor => {
            let (code, output) = doctor::run_doctor("snip", VERSION, config.format);
            if !config.quiet || config.output_file.is_some() {
                write_output(&output, config.output_file.as_ref())?;
            }
            Ok(code)
        }
        Subcommand::Update => update::run_update("snip", VERSION).map_err(CliError::Runtime),
        Subcommand::Serve => serve::run_server().map_err(CliError::Runtime),
        Subcommand::Audit => run_audit(&config),
    }
}

fn main() {
    match run() {
        Ok(code) => process::exit(code),
        Err(CliError::Parse(err)) => {
            eprintln!("error: {}", err);
            process::exit(2);
        }
        Err(CliError::Runtime(err)) => {
            eprintln!("error: {}", err);
            process::exit(1);
        }
    }
}
