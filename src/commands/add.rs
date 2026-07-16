use crate::cli::AddArgs;
use crate::error::{AppError, AppResult};
use crate::output::{self, Meta};
use crate::store;
use crate::{CutRecord, Evidence, compute_id, format_timestamp, resolve_agent};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
#[cfg(not(unix))]
use std::fs::File;
#[cfg(unix)]
use std::fs::OpenOptions;
use std::io::{IsTerminal, Read};
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
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
                "duplicate papercut; existing record returned"
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
    // Opening first makes the handle, rather than a path lookup, the object we validate.
    // OpenOptions follows symlinks, preserving the accepted symlink-to-regular-file policy.
    #[cfg(unix)]
    let mut file = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NONBLOCK)
        .open(path)
        .map_err(|error| AppError::from_evidence_file(error, path))?;
    #[cfg(not(unix))]
    let mut file = File::open(path).map_err(|error| AppError::from_evidence_file(error, path))?;
    let metadata = file
        .metadata()
        .map_err(|error| AppError::from_evidence_file(error, path))?;
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
        let Some((end, key, option, sensitive_named_path)) = sensitive_key(input, start) else {
            continue;
        };
        let mut value_start = skip_layout(input, end);
        let assignment = assignment_separator(input, value_start);
        if let Some(separator_length) = assignment {
            value_start += separator_length;
            value_start = skip_layout(input, value_start);
        } else if !option && key != "bearer" {
            continue;
        }
        value_start = skip_layout(input, value_start);
        let span = if key == "authorization" {
            authorization_value_span(input, value_start)
        } else {
            value_span(input, value_start)
        };
        if let Some(span) = span
            && !(sensitive_named_path && option && looks_like_path_or_url(&input[span.0..span.1]))
        {
            spans.push(span);
        }
    }
    for (start, end) in high_entropy_spans(input) {
        spans.push((start, end));
    }
    spans.extend(url_userinfo_spans(input));
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

fn url_userinfo_spans(input: &str) -> Vec<(usize, usize)> {
    let lower = input.to_ascii_lowercase();
    let mut spans = Vec::new();
    let mut search_start = 0;
    while let Some(offset) = lower[search_start..].find("http") {
        let start = search_start + offset;
        let scheme_len = if lower[start..].starts_with("https://") {
            "https://".len()
        } else if lower[start..].starts_with("http://") {
            "http://".len()
        } else {
            search_start = start + "http".len();
            continue;
        };
        let authority_start = start + scheme_len;
        let authority_end = input[authority_start..]
            .char_indices()
            .find(|(_, character)| "/?#\"' \t\r\n".contains(*character))
            .map_or(input.len(), |(offset, _)| authority_start + offset);
        if let Some(at) = input[authority_start..authority_end].rfind('@') {
            spans.push((authority_start, authority_start + at));
        }
        search_start = authority_end.max(authority_start);
    }
    spans
}

fn sensitive_key(input: &str, start: usize) -> Option<(usize, &'static str, bool, bool)> {
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
    let normalized = raw.to_ascii_lowercase();
    let segments = key_segments(raw);
    let sensitive_named_path = segments.last().is_some_and(|segment| {
        segment.eq_ignore_ascii_case("file") || segment.eq_ignore_ascii_case("path")
    }) && segments.iter().any(|segment| is_sensitive_segment(segment));
    let delimiter_name = raw.contains(['_', '-']);
    let key = match normalized.as_str() {
        "authorization" => "authorization",
        "bearer" => "bearer",
        "apikey" => "key",
        "password" | "passwd" | "secret" | "token" | "key" => "key",
        _ if [
            "access", "api", "auth", "client", "consumer", "db", "private", "refresh", "session",
            "ssh",
        ]
        .iter()
        .any(|prefix| {
            normalized
                .strip_prefix(prefix)
                .is_some_and(is_sensitive_segment)
        }) =>
        {
            "key"
        }
        _ if delimiter_name
            && segments
                .iter()
                .any(|segment| segment.eq_ignore_ascii_case("authorization")) =>
        {
            "authorization"
        }
        _ if segments.len() > 1 && segments.iter().any(|segment| is_sensitive_segment(segment)) => {
            "key"
        }
        _ => return None,
    };
    Some((end, key, option, sensitive_named_path))
}

fn key_segments(raw: &str) -> Vec<&str> {
    let mut segments = Vec::new();
    let mut start = 0;
    let mut previous = None;
    let mut characters = raw.char_indices().peekable();
    while let Some((index, character)) = characters.next() {
        if matches!(character, '_' | '-') {
            if start < index {
                segments.push(&raw[start..index]);
            }
            start = index + character.len_utf8();
            previous = None;
        } else {
            if previous.is_some_and(|previous: char| {
                previous.is_ascii_lowercase() && character.is_ascii_uppercase()
            }) || (previous.is_some_and(|previous: char| previous.is_ascii_uppercase())
                && character.is_ascii_uppercase()
                && characters
                    .peek()
                    .is_some_and(|(_, next)| next.is_ascii_lowercase()))
            {
                segments.push(&raw[start..index]);
                start = index;
            }
            previous = Some(character);
        }
    }
    if start < raw.len() {
        segments.push(&raw[start..]);
    }
    segments
}

fn is_sensitive_segment(segment: &str) -> bool {
    matches!(
        segment.to_ascii_lowercase().as_str(),
        "password" | "passwd" | "secret" | "token" | "key"
    )
}

fn assignment_separator(input: &str, index: usize) -> Option<usize> {
    input[index..]
        .chars()
        .next()
        .filter(|character| matches!(character, '=' | ':' | '＝' | '：'))
        .map(char::len_utf8)
}

fn skip_layout(input: &str, mut index: usize) -> usize {
    while input[index..]
        .chars()
        .next()
        .is_some_and(|character| character.is_whitespace() || character == '\u{200b}')
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
            .char_indices()
            .find_map(|(offset, character)| {
                (character == first && !is_escaped_quote(input, content_start + offset))
                    .then_some(content_start + offset)
            })
            .unwrap_or(input.len());
        return (content_start < content_end).then_some((content_start, content_end));
    }
    let end = input[start..]
        .char_indices()
        .find(|(_, character)| character.is_ascii_whitespace() || ",;)]}&#\"'".contains(*character))
        .map_or(input.len(), |(offset, _)| start + offset);
    (start < end).then_some((start, end))
}

fn is_escaped_quote(input: &str, quote: usize) -> bool {
    input[..quote]
        .bytes()
        .rev()
        .take_while(|byte| *byte == b'\\')
        .count()
        % 2
        == 1
}

fn authorization_value_span(input: &str, start: usize) -> Option<(usize, usize)> {
    let first = input[start..].chars().next()?;
    if matches!(first, '\'' | '"') {
        return value_span(input, start);
    }
    let scheme_end = input[start..]
        .char_indices()
        .find(|(_, character)| character.is_whitespace() || *character == '\u{200b}')
        .map_or(input.len(), |(offset, _)| start + offset);
    let credential_start = skip_layout(input, scheme_end);
    if credential_start == scheme_end {
        return value_span(input, start);
    }
    let credential_quote = input[credential_start..].chars().next()?;
    let (_, credential_end) = value_span(input, credential_start)?;
    let end = if matches!(credential_quote, '\'' | '"')
        && input[credential_end..].starts_with(credential_quote)
    {
        credential_end + credential_quote.len_utf8()
    } else {
        credential_end
    };
    Some((start, end))
}

fn high_entropy_spans(input: &str) -> Vec<(usize, usize)> {
    let mut spans = Vec::new();
    let mut start = None;
    for (index, character) in input
        .char_indices()
        .chain(std::iter::once((input.len(), ' ')))
    {
        let token_character = character.is_ascii_alphanumeric() || "_-./+=\\:".contains(character);
        if token_character && start.is_none() {
            start = Some(index);
        } else if !token_character && let Some(token_start) = start.take() {
            let token = &input[token_start..index];
            let (value_start, value) = token
                .split_once('=')
                .filter(|(name, value)| {
                    !name.is_empty()
                        && name.bytes().all(|byte| {
                            byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-'
                        })
                        && value.bytes().any(|byte| byte != b'=')
                })
                .map_or((token_start, token), |(name, value)| {
                    (token_start + name.len() + 1, value)
                });
            let unique = value
                .bytes()
                .collect::<std::collections::HashSet<_>>()
                .len();
            let unpadded = value.trim_end_matches('=');
            let core_unique = unpadded
                .bytes()
                .collect::<std::collections::HashSet<_>>()
                .len();
            let core_categories = [
                unpadded.bytes().any(|byte| byte.is_ascii_lowercase()),
                unpadded.bytes().any(|byte| byte.is_ascii_uppercase()),
                unpadded.bytes().any(|byte| byte.is_ascii_digit()),
                unpadded.bytes().any(|byte| !byte.is_ascii_alphanumeric()),
            ]
            .into_iter()
            .filter(|present| *present)
            .count();
            let categories = [
                value.bytes().any(|byte| byte.is_ascii_lowercase()),
                value.bytes().any(|byte| byte.is_ascii_uppercase()),
                value.bytes().any(|byte| byte.is_ascii_digit()),
                value.bytes().any(|byte| !byte.is_ascii_alphanumeric()),
            ]
            .into_iter()
            .filter(|present| *present)
            .count();
            let not_structural = !looks_like_path_or_url(value);
            if not_structural
                && ((value.len() >= 24
                    && unique >= 12
                    && (categories >= 3
                        || (categories >= 2 && value.bytes().any(|byte| byte.is_ascii_digit()))))
                    || (value.len() >= 32 && core_unique >= 20 && core_categories == 1))
            {
                spans.push((value_start, index));
            }
        }
    }
    spans
}

fn looks_like_path_or_url(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.starts_with("http://")
        || lower.starts_with("https://")
        || lower.starts_with("file://")
        || value.starts_with('/')
        || value.starts_with("./")
        || value.starts_with("../")
        || value.starts_with("\\\\")
        || value.as_bytes().get(0..3).is_some_and(|prefix| {
            prefix[0].is_ascii_alphabetic()
                && prefix[1] == b':'
                && matches!(prefix[2], b'/' | b'\\')
        })
        || looks_like_relative_path(value)
        || looks_like_schemeless_url(value)
}

fn looks_like_relative_path(value: &str) -> bool {
    let components: Vec<_> = value
        .split('/')
        .filter(|component| !component.is_empty())
        .collect();
    let Some(last) = components.last() else {
        return false;
    };
    components.len() >= 2
        && (components.iter().any(|component| {
            matches!(
                *component,
                "app"
                    | "apps"
                    | "assets"
                    | "cache"
                    | "config"
                    | "configs"
                    | "docs"
                    | "lib"
                    | "scripts"
                    | "src"
                    | "test"
                    | "tests"
            )
        }) || plausible_extension(last))
}

fn looks_like_schemeless_url(value: &str) -> bool {
    let Some((host, _)) = value.split_once('/') else {
        return false;
    };
    let mut labels = host.split('.');
    let Some(first) = labels.next() else {
        return false;
    };
    !first.is_empty()
        && first
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
        && labels.clone().all(|label| {
            !label.is_empty()
                && label
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
        })
        && labels.count() > 0
}

fn plausible_extension(component: &str) -> bool {
    component.rsplit_once('.').is_some_and(|(_, extension)| {
        (2..=8).contains(&extension.len())
            && extension.bytes().all(|byte| byte.is_ascii_alphanumeric())
    })
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
            Some((9, "key", false, false))
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
