//! main.rs — Snip CLI entry point.
//! Standard CLI flags: -h/--help, -V/--version, --format, -o/--output, -q/--quiet, -v/--verbose.

use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;
use std::process;
use snip::{check, emit_cursor_markdown, emit_generic_text, emit_json, Diff, Policy, Verdict};

const VERSION: &str = "0.2.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputFormat {
    Text,
    Json,
    Cursor,
}

#[derive(Debug)]
struct CliConfig {
    input_file: Option<PathBuf>,
    format: OutputFormat,
    output_file: Option<PathBuf>,
    quiet: bool,
    verbose: bool,
}

impl Default for CliConfig {
    fn default() -> Self {
        CliConfig {
            input_file: None,
            format: OutputFormat::Text,
            output_file: None,
            quiet: false,
            verbose: false,
        }
    }
}

fn print_help() {
    println!(
        "snip {} — Vibe-code security gate for AI diffs\n\
        \n\
        USAGE:\n\
          git diff | snip [OPTIONS]\n\
          snip [OPTIONS] [FILE]\n\
        \n\
        SUBCOMMANDS / VERBS:\n\
          check, audit    Audit diff content (default)\n\
        \n\
        OPTIONS:\n\
          -h, --help              Print help information\n\
          -V, --version           Print version information\n\
          --format <fmt>          Output format: text, json, cursor [default: text]\n\
          -o, --output <file>     Write report to file instead of stdout\n\
          -q, --quiet             Quiet mode; exit code only\n\
          -v, --verbose           Verbose diagnostic logging to stderr\n\
        \n\
        EXAMPLES:\n\
          git diff | snip\n\
          git diff --staged | snip --format cursor\n\
          snip -o report.json --format json patch.diff\n",
        VERSION
    );
}

fn parse_args(args: &[String]) -> Result<Option<CliConfig>, String> {
    let mut config = CliConfig::default();
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_help();
                return Ok(None);
            }
            "-V" | "--version" => {
                println!("snip {}", VERSION);
                return Ok(None);
            }
            "-q" | "--quiet" => config.quiet = true,
            "-v" | "--verbose" => config.verbose = true,
            "--format" => {
                i += 1;
                if i >= args.len() {
                    return Err("Missing argument for --format".to_string());
                }
                config.format = match args[i].to_lowercase().as_str() {
                    "json" => OutputFormat::Json,
                    "cursor" | "markdown" | "md" => OutputFormat::Cursor,
                    "text" => OutputFormat::Text,
                    other => return Err(format!("Unknown format: {}", other)),
                };
            }
            "-o" | "--output" => {
                i += 1;
                if i >= args.len() {
                    return Err("Missing argument for -o/--output".to_string());
                }
                config.output_file = Some(PathBuf::from(&args[i]));
            }
            "check" | "audit" => {}
            arg if !arg.starts_with('-') => {
                config.input_file = Some(PathBuf::from(arg));
            }
            other => return Err(format!("Unknown option: {}", other)),
        }
        i += 1;
    }
    Ok(Some(config))
}

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
        fs::write(path, content)
    } else {
        print!("{}", content);
        Ok(())
    }
}

fn run() -> Result<i32, String> {
    let args: Vec<String> = env::args().collect();
    let config = match parse_args(&args)? {
        Some(c) => c,
        None => return Ok(0),
    };

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
        write_output(&output, config.output_file.as_ref())
            .map_err(|e| format!("Write failed: {}", e))?;
    }

    Ok(match result.verdict {
        Verdict::Ship => 0,
        Verdict::Fix | Verdict::Block => 1,
    })
}

fn main() {
    match run() {
        Ok(code) => process::exit(code),
        Err(err) => {
            eprintln!("error: {}", err);
            process::exit(2);
        }
    }
}
