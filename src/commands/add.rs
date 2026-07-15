use crate::cli::AddArgs;
use crate::error::{AppError, AppResult};
use crate::output::{self, Meta};
use crate::store;
use crate::{CutRecord, Evidence, compute_id, format_timestamp, resolve_agent};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{IsTerminal, Read};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct AddData {
    pub changed: bool,
    pub record: CutRecord,
}

pub fn run(args: AddArgs, file: Option<PathBuf>, pretty: bool, now: Timestamp) -> AppResult<i32> {
    let resolved = store::discover(file)?;
    let evidence = build_evidence(&args)?;
    let text = read_text(args.text)?;
    if text.trim().is_empty() {
        return Err(AppError::invalid_input(
            "papercut text cannot be empty or whitespace-only",
            "Pass non-empty TEXT or pipe it on stdin.",
        ));
    }
    if text.len() > 10_000 {
        return Err(AppError::invalid_input(
            format!(
                "papercut text is {} bytes; the maximum is 10000",
                text.len()
            ),
            "Shorten the papercut text to at most 10000 UTF-8 bytes.",
        ));
    }
    if args
        .agent
        .as_deref()
        .is_some_and(|agent| agent.trim().is_empty())
    {
        return Err(AppError::invalid_input(
            "agent name cannot be empty or whitespace-only",
            "Pass a non-empty --agent NAME or omit the flag.",
        ));
    }
    let (agent, source) = resolve_agent(args.agent);
    if agent.trim().is_empty() {
        return Err(AppError::invalid_input(
            "agent name cannot be whitespace-only",
            "Pass a non-empty --agent NAME or set PAPERCUTS_AGENT.",
        ));
    }
    let mut tags = args.tags;
    tags.sort();
    let mut warnings = Vec::new();
    let ts = format_timestamp(now);
    let record = CutRecord {
        kind: "cut".into(),
        id: compute_id(&ts, &agent, &text, args.severity, &tags),
        ts,
        agent,
        text,
        tags,
        severity: args.severity,
        cwd: std::env::current_dir()
            .map_err(|error| AppError::from_io(error, std::path::Path::new(".")))?
            .to_string_lossy()
            .into_owned(),
        repo: resolved
            .repo
            .as_ref()
            .map(|path| path.to_string_lossy().into_owned()),
        evidence,
    };
    let trimmed = record.text.trim_start();
    if trimmed.starts_with("RESOLUTION") || trimmed.starts_with("RESOLVED") {
        warnings.push(
            "resolution_text: this looks like a resolution; use `papercuts resolve <id>` for an existing cut".into(),
        );
    }

    let (changed, record) = if args.dry_run {
        (false, record)
    } else {
        store::with_exclusive(&resolved.path, true, |log| {
            let bytes = store::read_bytes(log, &resolved.path)?;
            if let Some(existing) = store::fold_bytes(&bytes)
                .items
                .into_iter()
                .find(|item| item.cut.id == record.id)
            {
                return Ok((false, existing.cut));
            }
            store::append_json(log, &resolved.path, &bytes, &record)?;
            Ok((true, record))
        })?
    };
    if args.dry_run {
        warnings.push("dry run; no record appended".into());
    } else if !changed {
        warnings
            .push("duplicate_cut: existing record returned; later evidence was not stored".into());
    }
    let mut meta = Meta::new();
    meta.file = Some(resolved.path.to_string_lossy().into_owned());
    meta.agent_source = Some(source.into());
    meta.warnings = warnings;
    output::write_success(AddData { changed, record }, pretty, meta)
        .map_err(|error| AppError::from_io(error, std::path::Path::new("stdout")))?;
    Ok(0)
}

fn build_evidence(args: &AddArgs) -> AppResult<Option<Evidence>> {
    let stderr = args.stderr_file.as_deref().map(read_stderr).transpose()?;
    if args.cmd.is_none() && args.exit_code.is_none() && stderr.is_none() && args.evidence.is_none()
    {
        return Ok(None);
    }
    Ok(Some(Evidence {
        cmd: args.cmd.as_deref().map(redact_evidence),
        exit: args.exit_code,
        stderr: stderr.map(|value| redact_and_truncate(&value, 4096)),
        note: args.evidence.as_deref().map(redact_evidence),
    }))
}

fn read_stderr(path: &std::path::Path) -> AppResult<String> {
    let mut file = File::open(path).map_err(|error| AppError::from_log_open(error, path))?;
    let mut bytes = Vec::new();
    file.by_ref()
        .take(4096 + 4)
        .read_to_end(&mut bytes)
        .map_err(|error| AppError::from_io(error, path))?;
    match String::from_utf8(bytes) {
        Ok(value) => Ok(value),
        Err(error) if error.utf8_error().valid_up_to() >= 4096 => {
            let valid = error.utf8_error().valid_up_to();
            let bytes = error.into_bytes();
            Ok(String::from_utf8(bytes[..valid].to_vec()).expect("valid prefix"))
        }
        Err(_) => Err(AppError::invalid_input(
            format!("stderr file is not valid UTF-8: {}", path.display()),
            "Pass a UTF-8 stderr file with --stderr-file PATH.",
        )),
    }
}

fn redact_and_truncate(value: &str, max_bytes: usize) -> String {
    truncate_utf8(&redact_evidence(value), max_bytes)
}

fn truncate_utf8(value: &str, max_bytes: usize) -> String {
    if value.len() <= max_bytes {
        return value.to_owned();
    }
    let mut end = max_bytes;
    while !value.is_char_boundary(end) {
        end -= 1;
    }
    value[..end].to_owned()
}

fn redact_evidence(input: &str) -> String {
    let mut spans = Vec::new();
    let keywords = [
        "authorization",
        "password",
        "secret",
        "token",
        "bearer",
        "key",
    ];
    for (start, _) in input.char_indices() {
        let Some(keyword) = keywords.iter().find(|keyword| {
            input[start..].len() >= keyword.len()
                && input
                    .get(start..start + keyword.len())
                    .is_some_and(|candidate| candidate.eq_ignore_ascii_case(keyword))
                && boundary_before(input, start)
                && boundary_after(input, start + keyword.len())
        }) else {
            continue;
        };
        let end = start + keyword.len();
        let mut value_start = skip_spaces(input, end);
        let assignment = input
            .as_bytes()
            .get(value_start)
            .is_some_and(|byte| *byte == b'=' || *byte == b':');
        if assignment {
            value_start = skip_spaces(input, value_start + 1);
        } else if *keyword != "bearer"
            && input
                .get(start.saturating_sub(2)..start)
                .is_none_or(|prefix| prefix != "--")
        {
            continue;
        }
        value_start = skip_spaces(input, value_start);
        if input[value_start..]
            .to_ascii_lowercase()
            .starts_with("bearer ")
        {
            value_start = skip_spaces(input, value_start + "bearer".len());
        }
        if let Some(span) = value_span(input, value_start) {
            spans.push(span);
        }
    }
    for (start, end) in high_entropy_spans(input) {
        spans.push((start, end));
    }
    spans.sort_unstable();
    let mut merged = Vec::new();
    for (start, end) in spans {
        if let Some((_, last_end)) = merged.last_mut()
            && start <= *last_end
        {
            *last_end = (*last_end).max(end);
        } else {
            merged.push((start, end));
        }
    }
    let mut output = String::with_capacity(input.len());
    let mut cursor = 0;
    for (start, end) in merged {
        output.push_str(&input[cursor..start]);
        output.push_str("<redacted>");
        cursor = end;
    }
    output.push_str(&input[cursor..]);
    output
}

fn boundary_before(input: &str, start: usize) -> bool {
    input[..start]
        .chars()
        .next_back()
        .is_none_or(|character| !character.is_ascii_alphanumeric())
}

fn boundary_after(input: &str, end: usize) -> bool {
    input[end..]
        .chars()
        .next()
        .is_none_or(|character| !character.is_ascii_alphanumeric())
}

fn skip_spaces(input: &str, mut index: usize) -> usize {
    while input[index..]
        .chars()
        .next()
        .is_some_and(|character| character.is_ascii_whitespace())
    {
        index += input[index..]
            .chars()
            .next()
            .expect("checked above")
            .len_utf8();
    }
    index
}

fn value_span(input: &str, start: usize) -> Option<(usize, usize)> {
    let first = input[start..].chars().next()?;
    if first == '\'' || first == '"' {
        let content_start = start + first.len_utf8();
        let content_end = input[content_start..]
            .find(first)
            .map_or(input.len(), |offset| content_start + offset);
        return (content_start < content_end).then_some((content_start, content_end));
    }
    let end = input[start..]
        .char_indices()
        .find(|(_, character)| character.is_ascii_whitespace() || ",;)]}".contains(*character))
        .map_or(input.len(), |(offset, _)| start + offset);
    (start < end).then_some((start, end))
}

fn high_entropy_spans(input: &str) -> Vec<(usize, usize)> {
    let mut spans = Vec::new();
    let mut start = None;
    for (index, character) in input
        .char_indices()
        .chain(std::iter::once((input.len(), ' ')))
    {
        let token_character = character.is_ascii_alphanumeric() || "_-./+=".contains(character);
        if token_character && start.is_none() {
            start = Some(index);
        } else if !token_character && let Some(token_start) = start.take() {
            let token = &input[token_start..index];
            let unique = token
                .bytes()
                .collect::<std::collections::HashSet<_>>()
                .len();
            let categories = [
                token.bytes().any(|byte| byte.is_ascii_lowercase()),
                token.bytes().any(|byte| byte.is_ascii_uppercase()),
                token.bytes().any(|byte| byte.is_ascii_digit()),
                token.bytes().any(|byte| !byte.is_ascii_alphanumeric()),
            ]
            .into_iter()
            .filter(|present| *present)
            .count();
            if token.len() >= 24
                && unique >= 12
                && (categories >= 3
                    || (categories >= 2 && token.bytes().any(|byte| byte.is_ascii_digit())))
            {
                spans.push((token_start, index));
            }
        }
    }
    spans
}

fn read_text(text: Option<String>) -> AppResult<String> {
    let use_stdin =
        text.as_deref() == Some("-") || (text.is_none() && !std::io::stdin().is_terminal());
    let mut text = if use_stdin {
        let mut input = Vec::new();
        std::io::stdin()
            .lock()
            .read_to_end(&mut input)
            .map_err(|error| AppError::from_io(error, std::path::Path::new("stdin")))?;
        String::from_utf8(input).map_err(|_| {
            AppError::invalid_input(
                "papercut text from stdin is not valid UTF-8",
                "Pipe UTF-8 text to `papercuts add -`.",
            )
        })?
    } else {
        text.ok_or_else(|| {
            AppError::invalid_argument(
                "add requires TEXT when stdin is a terminal",
                "Run `papercuts add \"what went wrong\"` or pipe text to `papercuts add -`.",
            )
        })?
    };
    while text.ends_with('\n') || text.ends_with('\r') {
        text.pop();
    }
    Ok(text)
}
