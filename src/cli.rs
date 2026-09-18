//! cli.rs — Standard CLI arguments and subcommand parser for snip.
use std::fmt;
use std::path::PathBuf;

pub const VERSION: &str = "0.2.3";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Text,
    Json,
    Cursor,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Subcommand {
    Audit,
    Serve,
    Doctor,
    Update,
    Help,
    Version,
}

#[derive(Debug)]
pub struct CliConfig {
    pub subcommand: Subcommand,
    pub input_file: Option<PathBuf>,
    pub format: OutputFormat,
    pub output_file: Option<PathBuf>,
    pub quiet: bool,
    pub verbose: bool,
}

impl Default for CliConfig {
    fn default() -> Self {
        CliConfig {
            subcommand: Subcommand::Audit,
            input_file: None,
            format: OutputFormat::Text,
            output_file: None,
            quiet: false,
            verbose: false,
        }
    }
}

#[derive(Debug)]
pub enum CliError {
    Parse(String),
    Runtime(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::Parse(s) | CliError::Runtime(s) => write!(f, "{}", s),
        }
    }
}

impl From<String> for CliError {
    fn from(s: String) -> Self {
        CliError::Runtime(s)
    }
}

impl From<&str> for CliError {
    fn from(s: &str) -> Self {
        CliError::Runtime(s.to_string())
    }
}

impl From<std::io::Error> for CliError {
    fn from(e: std::io::Error) -> Self {
        CliError::Runtime(e.to_string())
    }
}

pub fn print_help() {
    println!(
        "snip {} — Vibe-code security gate for AI diffs\n\
        \n\
        USAGE:\n\
          git diff | snip [OPTIONS]\n\
          snip [SUBCOMMAND] [OPTIONS] [FILE]\n\
        \n\
        SUBCOMMANDS:\n\
          audit, check     Audit diff content for vibe-code hazards (default)\n\
          serve            Start MCP (Model Context Protocol) JSON-RPC server\n\
          doctor           Inspect system health and environment\n\
          update, upgrade  Update binary to latest release\n\
          help             Print help information\n\
          version          Print version information\n\
        \n\
        OPTIONS:\n\
          -h, --help              Print help information\n\
          -V, --version           Print version information\n\
          -f, --format <fmt>      Output format: text, json, cursor [default: text]\n\
          -o, --output <file>     Write report to file instead of stdout\n\
          -q, --quiet             Quiet mode; exit code only\n\
          -v, --verbose           Verbose diagnostic logging to stderr\n\
          --mcp                   Run server in MCP mode\n\
        \n\
        EXAMPLES:\n\
          git diff | snip\n\
          git diff --staged | snip -f cursor\n\
          snip serve --mcp\n\
          snip doctor\n",
        VERSION
    );
}

pub fn parse_args(args: &[String]) -> Result<CliConfig, CliError> {
    let mut config = CliConfig::default();
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" | "help" => {
                config.subcommand = Subcommand::Help;
                return Ok(config);
            }
            "-V" | "--version" | "version" => {
                config.subcommand = Subcommand::Version;
                return Ok(config);
            }
            "-q" | "--quiet" => config.quiet = true,
            "-v" | "--verbose" => config.verbose = true,
            "--mcp" => {
                config.subcommand = Subcommand::Serve;
            }
            "-f" | "--format" => {
                i += 1;
                if i >= args.len() {
                    return Err(CliError::Parse("Missing argument for format option".into()));
                }
                config.format = match args[i].to_lowercase().as_str() {
                    "json" => OutputFormat::Json,
                    "cursor" | "markdown" | "md" => OutputFormat::Cursor,
                    "text" => OutputFormat::Text,
                    other => return Err(CliError::Parse(format!("Unknown format: {}", other))),
                };
            }
            "-o" | "--output" => {
                i += 1;
                if i >= args.len() {
                    return Err(CliError::Parse("Missing argument for output option".into()));
                }
                config.output_file = Some(PathBuf::from(&args[i]));
            }
            "audit" | "check" => {
                config.subcommand = Subcommand::Audit;
            }
            "serve" => {
                config.subcommand = Subcommand::Serve;
            }
            "doctor" => {
                config.subcommand = Subcommand::Doctor;
            }
            "update" | "upgrade" => {
                config.subcommand = Subcommand::Update;
            }
            arg if !arg.starts_with('-') => {
                config.input_file = Some(PathBuf::from(arg));
            }
            other => return Err(CliError::Parse(format!("Unknown option: {}", other))),
        }
        i += 1;
    }
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_flags() {
        let args = vec!["snip".into(), "-q".into(), "-f".into(), "json".into()];
        let cfg = parse_args(&args).unwrap();
        assert!(cfg.quiet);
        assert_eq!(cfg.format, OutputFormat::Json);
    }

    #[test]
    fn test_subcommands() {
        let args = vec!["snip".into(), "serve".into(), "--mcp".into()];
        assert_eq!(parse_args(&args).unwrap().subcommand, Subcommand::Serve);
        let args = vec!["snip".into(), "doctor".into()];
        assert_eq!(parse_args(&args).unwrap().subcommand, Subcommand::Doctor);
    }

    #[test]
    fn test_parse_errors() {
        let args = vec!["snip".into(), "--invalid".into()];
        assert!(matches!(parse_args(&args), Err(CliError::Parse(_))));
        let args = vec!["snip".into(), "-f".into()];
        assert!(matches!(parse_args(&args), Err(CliError::Parse(_))));
    }
}
