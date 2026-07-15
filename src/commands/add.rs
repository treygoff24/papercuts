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

const STDERR_INPUT_LIMIT: u64 = 1024 * 1024;

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

    let supplied_evidence = record.evidence.is_some();
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
        warnings.push(
            if supplied_evidence {
                "duplicate_cut: existing record returned; later evidence was not stored"
            } else {
                "duplicate_cut: existing record returned"
            }
            .into(),
        );
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
    // `metadata` follows a symlink, so a symlink to a regular file is accepted and a
    // symlink to a FIFO/device is rejected before opening the target for reading.
    let metadata =
        std::fs::metadata(path).map_err(|error| AppError::from_evidence_file(error, path))?;
    if !metadata.is_file() {
        return Err(AppError::invalid_input(
            format!(
                "stderr evidence path is not a regular file: {}",
                path.display()
            ),
            "Pass a regular UTF-8 file to --stderr-file PATH; FIFOs and devices are not accepted.",
        ));
    }
    if metadata.len() > STDERR_INPUT_LIMIT {
        return Err(AppError::invalid_input(
            format!(
                "stderr evidence file exceeds the {}-byte read limit: {}",
                STDERR_INPUT_LIMIT,
                path.display()
            ),
            "Pass a smaller stderr file to --stderr-file PATH; stored sanitized stderr is capped at 4096 bytes.",
        ));
    }
    let mut file = File::open(path).map_err(|error| AppError::from_evidence_file(error, path))?;
    let mut bytes = Vec::new();
    file.by_ref()
        .take(STDERR_INPUT_LIMIT + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| AppError::from_evidence_file(error, path))?;
    if bytes.len() > STDERR_INPUT_LIMIT as usize {
        return Err(AppError::invalid_input(
            format!(
                "stderr evidence file exceeds the {}-byte read limit: {}",
                STDERR_INPUT_LIMIT,
                path.display()
            ),
            "Pass a smaller stderr file to --stderr-file PATH; stored sanitized stderr is capped at 4096 bytes.",
        ));
    }
    String::from_utf8(bytes).map_err(|_| {
        AppError::invalid_input(
            format!("stderr file is not valid UTF-8: {}", path.display()),
            "Pass a UTF-8 stderr file with --stderr-file PATH.",
        )
    })
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
    for (start, _) in input.char_indices() {
        let Some((end, key, option)) = sensitive_key(input, start) else {
            continue;
        };
        let mut value_start = skip_spaces(input, end);
        let assignment = input
            .get(value_start..)
            .and_then(|rest| rest.chars().next())
            .is_some_and(|character| character == '=' || character == ':');
        if assignment {
            value_start += input[value_start..]
                .chars()
                .next()
                .expect("checked above")
                .len_utf8();
            value_start = skip_spaces(input, value_start);
        } else if !option && key != "bearer" {
            continue;
        }
        value_start = skip_spaces(input, value_start);
        let span = if key == "authorization" && assignment {
            authorization_value_span(input, value_start)
        } else {
            value_span(input, value_start)
        };
        if let Some(span) = span {
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

fn sensitive_key(input: &str, start: usize) -> Option<(usize, &'static str, bool)> {
    let prior = input[..start].chars().next_back();
    if prior.is_some_and(|character| {
        character.is_alphanumeric() || character == '_' || character == '-'
    }) {
        return None;
    }
    let rest = &input[start..];
    let (raw, end, option) = if let Some(rest) = rest.strip_prefix("--") {
        let length = rest
            .char_indices()
            .find(|(_, character)| {
                !character.is_alphanumeric() && *character != '_' && *character != '-'
            })
            .map_or(rest.len(), |(index, _)| index);
        (&rest[..length], start + 2 + length, true)
    } else if let Some(stripped) = rest.strip_prefix('"') {
        let close = stripped.find('"')?;
        (&stripped[..close], start + close + 2, false)
    } else {
        let length = rest
            .char_indices()
            .find(|(_, character)| {
                !character.is_alphanumeric() && *character != '_' && *character != '-'
            })
            .map_or(rest.len(), |(index, _)| index);
        (&rest[..length], start + length, false)
    };
    let normalized = raw
        .chars()
        .filter(|character| *character != '_' && *character != '-')
        .flat_map(char::to_lowercase)
        .collect::<String>();
    let key = match normalized.as_str() {
        "authorization" => "authorization",
        "password" => "password",
        "secret" => "secret",
        "token" => "token",
        "bearer" => "bearer",
        "key" | "apikey" => "key",
        _ => return None,
    };
    Some((end, key, option))
}

fn skip_spaces(input: &str, mut index: usize) -> usize {
    while input[index..]
        .chars()
        .next()
        .is_some_and(char::is_whitespace)
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

fn authorization_value_span(input: &str, start: usize) -> Option<(usize, usize)> {
    let scheme_end = input[start..]
        .char_indices()
        .find(|(_, character)| character.is_whitespace())
        .map_or(input.len(), |(offset, _)| start + offset);
    let scheme = &input[start..scheme_end];
    if scheme.eq_ignore_ascii_case("basic") || scheme.eq_ignore_ascii_case("bearer") {
        let value_start = skip_spaces(input, scheme_end);
        let (_, value_end) = value_span(input, value_start)?;
        return Some((start, value_end));
    }
    value_span(input, start)
}

fn high_entropy_spans(input: &str) -> Vec<(usize, usize)> {
    let mut spans = Vec::new();
    let mut start = None;
    for (index, character) in input
        .char_indices()
        .chain(std::iter::once((input.len(), ' ')))
    {
        let token_character = character.is_ascii_alphanumeric() || "_-./+=\\".contains(character);
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
            if !token.contains('/')
                && !token.contains('\\')
                && token.len() >= 24
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

#[cfg(test)]
mod tests {
    use super::{redact_evidence, sensitive_key};

    #[test]
    fn redacts_quoted_json_keys_with_unicode_whitespace() {
        let value = "quoted-secret-value";
        let header = "header-secret-value";
        assert_eq!(
            sensitive_key("\"api_key\"\u{2003}:\u{2002}value", 0),
            Some((9, "key", false))
        );
        let input = format!("\"api_key\"\u{2003}:\u{2002}\"{value}\"");
        assert_eq!(
            redact_evidence(&input),
            "\"api_key\"\u{2003}:\u{2002}\"<redacted>\""
        );
        let output = redact_evidence(&format!(
            "\"api_key\"\u{2003}:\u{2002}\"{value}\" authorization\u{2003}:\u{2002}Bearer {header} url=https://example.test/a/b"
        ));
        assert!(!output.contains(value));
        assert!(!output.contains(header));
    }
}
