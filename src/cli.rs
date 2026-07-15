use crate::Severity;
use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version,
    about,
    long_about = None,
    arg_required_else_help = true,
    subcommand_required = true,
    color = clap::ColorChoice::Never,
    rename_all = "kebab-case"
)]
pub struct Cli {
    #[arg(long, global = true, value_name = "PATH")]
    pub file: Option<PathBuf>,

    #[arg(long, global = true)]
    pub pretty: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(alias = "log")]
    Add(AddArgs),
    List(ListArgs),
    Resolve(ResolveArgs),
    Schema {
        #[arg(value_enum, default_value_t = SchemaTarget::All)]
        target: SchemaTarget,
    },
    Doctor,
}

#[derive(Debug, Args)]
pub struct AddArgs {
    #[arg(value_name = "TEXT")]
    pub text: Option<String>,
    #[arg(long)]
    pub agent: Option<String>,
    #[arg(long = "tag")]
    pub tags: Vec<String>,
    #[arg(long, value_enum, default_value_t = Severity::Minor)]
    pub severity: Severity,
    #[arg(
        long,
        allow_hyphen_values = true,
        value_name = "TEXT",
        help = "Command that failed"
    )]
    pub cmd: Option<String>,
    #[arg(long = "exit", value_name = "N", help = "Command exit status")]
    pub exit_code: Option<i32>,
    #[arg(
        long,
        value_name = "PATH",
        help = "Read regular UTF-8 PATH (<=1 MiB); best-effort redaction; store sanitized value <=4096 bytes"
    )]
    pub stderr_file: Option<PathBuf>,
    #[arg(
        long,
        allow_hyphen_values = true,
        value_name = "TEXT",
        help = "Additional evidence or filing note"
    )]
    pub evidence: Option<String>,
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(Debug, Args)]
pub struct ListArgs {
    #[arg(long, value_enum, default_value_t = StatusFilter::Open)]
    pub status: StatusFilter,
    #[arg(long)]
    pub agent: Option<String>,
    #[arg(long)]
    pub tag: Option<String>,
    #[arg(long, value_enum)]
    pub severity: Option<Severity>,
    #[arg(long)]
    pub since: Option<String>,
    #[arg(long, default_value_t = 50)]
    pub limit: usize,
    #[arg(long, value_enum, default_value_t = OutputFormat::Json)]
    pub format: OutputFormat,
}

#[derive(Debug, Args)]
pub struct ResolveArgs {
    #[arg(
        value_name = "ID",
        num_args = 1..,
        required = true,
        help = "One or more IDs or unique prefixes"
    )]
    pub ids: Vec<String>,
    #[arg(long, allow_hyphen_values = true)]
    pub note: Option<String>,
    #[arg(long)]
    pub agent: Option<String>,
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum StatusFilter {
    Open,
    Resolved,
    All,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    Json,
    Md,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum SchemaTarget {
    All,
    Record,
    Error,
    ExitCodes,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_covers_defaults_aliases_and_globals() {
        let cli =
            Cli::try_parse_from(["papercuts", "--file", "x", "log", "ouch", "--pretty"]).unwrap();
        assert!(cli.pretty);
        assert_eq!(cli.file, Some(PathBuf::from("x")));
        let Command::Add(args) = cli.command else {
            panic!("expected add")
        };
        assert_eq!(args.text.as_deref(), Some("ouch"));
        assert_eq!(args.severity, Severity::Minor);

        let cli = Cli::try_parse_from(["papercuts", "list"]).unwrap();
        let Command::List(args) = cli.command else {
            panic!("expected list")
        };
        assert_eq!(args.status, StatusFilter::Open);
        assert_eq!(args.limit, 50);
        assert_eq!(args.format, OutputFormat::Json);
    }

    #[test]
    fn parser_rejects_bad_values_and_missing_required_id() {
        assert!(Cli::try_parse_from(["papercuts", "list", "--format", "jsonl"]).is_err());
        assert!(Cli::try_parse_from(["papercuts", "add", "x", "--severity", "critical"]).is_err());
        assert!(Cli::try_parse_from(["papercuts", "resolve"]).is_err());
        assert!(Cli::try_parse_from(["papercuts"]).is_err());
    }

    #[test]
    fn parser_accepts_every_command_and_stdin_marker() {
        for args in [
            vec!["papercuts", "add", "-"],
            vec!["papercuts", "list", "--status", "all"],
            vec!["papercuts", "resolve", "abcd"],
            vec!["papercuts", "schema", "record"],
            vec!["papercuts", "doctor"],
        ] {
            assert!(Cli::try_parse_from(args).is_ok());
        }
    }

    #[test]
    fn parser_accepts_leading_hyphen_text_values_without_swallowing_following_options() {
        let cli = Cli::try_parse_from([
            "papercuts",
            "add",
            "text",
            "--cmd",
            "-tool arg",
            "--evidence",
            "--detail note",
            "--agent",
            "tester",
        ])
        .unwrap();
        let Command::Add(args) = cli.command else {
            panic!("expected add")
        };
        assert_eq!(args.cmd.as_deref(), Some("-tool arg"));
        assert_eq!(args.evidence.as_deref(), Some("--detail note"));
        assert_eq!(args.agent.as_deref(), Some("tester"));

        let cli = Cli::try_parse_from([
            "papercuts",
            "resolve",
            "abcd1234",
            "--note",
            "--retry after timeout",
            "--agent",
            "fixer",
        ])
        .unwrap();
        let Command::Resolve(args) = cli.command else {
            panic!("expected resolve")
        };
        assert_eq!(args.note.as_deref(), Some("--retry after timeout"));
        assert_eq!(args.agent.as_deref(), Some("fixer"));
    }
}
