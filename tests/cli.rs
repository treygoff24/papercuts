use assert_cmd::Command;
use papercuts::commands::add::AddData;
use papercuts::commands::doctor::DoctorData;
use papercuts::commands::list::ListData;
use papercuts::commands::resolve::ResolveData;
use papercuts::commands::resolve::ResolveManyData;
use papercuts::error::exit_code_map;
use papercuts::output::{ErrorEnvelope, SuccessEnvelope};
use papercuts::{ItemStatus, Severity, compute_id};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Barrier};
use std::thread;
use tempfile::TempDir;

const NOW: &str = "2026-07-09T18:30:00.123456Z";

fn command() -> Command {
    let mut command = assert_cmd::cargo::cargo_bin_cmd!("papercuts");
    command
        .env("PAPERCUTS_NOW", NOW)
        .env_remove("PAPERCUTS_FILE")
        .env_remove("PAPERCUTS_AGENT")
        .env_remove("CLAUDECODE");
    for (key, _) in std::env::vars_os() {
        if key.to_string_lossy().starts_with("CODEX_")
            || key.to_string_lossy().starts_with("CURSOR_")
        {
            command.env_remove(key);
        }
    }
    command
}

fn run(args: &[&str]) -> std::process::Output {
    command().args(args).output().unwrap()
}

fn run_file(file: &Path, args: &[&str]) -> std::process::Output {
    command()
        .arg("--file")
        .arg(file)
        .args(args)
        .output()
        .unwrap()
}

fn temp_has_git_ancestor(temp: &TempDir) -> bool {
    temp.path()
        .ancestors()
        .any(|ancestor| ancestor.join(".git").exists())
}

fn success<T: DeserializeOwned>(output: &std::process::Output) -> SuccessEnvelope<T> {
    assert!(
        output.status.success(),
        "status={:?}\nstderr={}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stderr.is_empty());
    serde_json::from_slice(&output.stdout).unwrap()
}

fn error(output: &std::process::Output, exit: i32, code: &str) -> ErrorEnvelope {
    assert_eq!(output.status.code(), Some(exit));
    assert!(output.stdout.is_empty());
    let envelope: ErrorEnvelope = serde_json::from_slice(&output.stderr).unwrap();
    assert!(!envelope.ok);
    assert_eq!(envelope.error.code, code);
    assert!(!envelope.error.suggested_fix.is_empty());
    assert_eq!(envelope.meta.contract, 1);
    envelope
}

fn add(file: &Path, text: &str) -> SuccessEnvelope<AddData> {
    let output = run_file(file, &["add", text, "--agent", "tester"]);
    success(&output)
}

#[test]
fn add_evidence_flags_are_redacted_bounded_and_optional_fields_are_omitted() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("cuts.jsonl");
    let stderr_file = temp.path().join("stderr.txt");
    std::fs::write(&stderr_file, format!("{}éafter-boundary", "x".repeat(4095))).unwrap();
    let output = command()
        .arg("--file")
        .arg(&file)
        .args([
            "add",
            "tool failed",
            "--agent",
            "tester",
            "--cmd",
            "curl -H 'Authorization: Bearer abc123'",
            "--exit",
            "7",
            "--stderr-file",
        ])
        .arg(&stderr_file)
        .args([
            "--evidence",
            "API_KEY=sk_live_secret token: abc password='hunter2' ghp_AbCdEf0123456789GhIjKlMnOpQrStUv monkey=keep tokenized=keep",
        ])
        .output()
        .unwrap();
    let added: SuccessEnvelope<AddData> = success(&output);
    let evidence = added.data.record.evidence.unwrap();
    assert_eq!(evidence.exit, Some(7));
    assert!(!evidence.cmd.as_deref().unwrap().contains("abc123"));
    assert!(!evidence.note.as_deref().unwrap().contains("sk_live_secret"));
    assert!(
        !evidence
            .note
            .as_deref()
            .unwrap()
            .contains("ghp_AbCdEf0123456789GhIjKlMnOpQrStUv")
    );
    assert!(evidence.note.as_deref().unwrap().contains("monkey=keep"));
    assert!(evidence.note.as_deref().unwrap().contains("tokenized=keep"));
    assert_eq!(evidence.stderr.as_deref().unwrap().len(), 4095);
    assert!(
        !evidence
            .stderr
            .as_deref()
            .unwrap()
            .contains("after-boundary")
    );

    let absent = add(&temp.path().join("absent.jsonl"), "no evidence");
    let absent_json = serde_json::to_value(&absent.data.record).unwrap();
    assert!(absent_json.get("evidence").is_none());

    let missing_stderr = run_file(
        &temp.path().join("missing-stderr.jsonl"),
        &["add", "missing stderr", "--stderr-file", "does-not-exist"],
    );
    let missing = error(&missing_stderr, 66, "not_found");
    assert!(
        missing
            .error
            .message
            .starts_with("stderr evidence file not found:")
    );
    assert!(missing.error.suggested_fix.contains("--stderr-file PATH"));
}

#[test]
fn evidence_redaction_handles_boundary_headers_unicode_json_and_cli_options() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("cuts.jsonl");
    let stderr_file = temp.path().join("stderr.txt");
    let secret = "boundary-secret-value";
    let quoted_secret = "quoted-secret-value";
    let header_secret = "header-secret-value";
    let url = "https://example.test/a/AbCdEf0123456789GhIjKlMnOpQrStUv";
    let path = "/tmp/AbCdEf0123456789GhIjKlMnOpQrStUv.log";
    std::fs::write(
        &stderr_file,
        format!(
            "{}Authorization: Basic {secret}\ncontext=kept",
            "x".repeat(4092)
        ),
    )
    .unwrap();
    let output = command()
        .arg("--file")
        .arg(&file)
        .args([
            "add",
            "tool failed",
            "--agent",
            "tester",
            "--stderr-file",
        ])
        .arg(&stderr_file)
        .args([
            "--cmd",
            &format!("tool --api-key {secret} --path /tmp/harmless"),
            "--evidence",
            &format!(
                "\"api_key\"\u{2003}:\u{2002}\"{quoted_secret}\" authorization\u{2003}:\u{2002}Bearer {header_secret} url={url} path={path}"
            ),
        ])
        .output()
        .unwrap();
    let added: SuccessEnvelope<AddData> = success(&output);
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stored = std::fs::read_to_string(&file).unwrap();
    for (label, evidence, forbidden) in [
        (
            "command",
            added
                .data
                .record
                .evidence
                .as_ref()
                .unwrap()
                .cmd
                .as_deref()
                .unwrap(),
            secret,
        ),
        (
            "note",
            added
                .data
                .record
                .evidence
                .as_ref()
                .unwrap()
                .note
                .as_deref()
                .unwrap(),
            quoted_secret,
        ),
        (
            "stderr",
            added
                .data
                .record
                .evidence
                .as_ref()
                .unwrap()
                .stderr
                .as_deref()
                .unwrap(),
            secret,
        ),
        ("stdout", stdout.as_str(), secret),
        ("stored", stored.as_str(), secret),
    ] {
        assert!(
            !evidence.contains(forbidden),
            "redaction failed for {label}"
        );
    }
    let note = added.data.record.evidence.unwrap().note.unwrap();
    assert!(!note.contains(header_secret));
    assert_eq!(note.matches("<redacted>").count(), 2);
    assert!(note.contains(url));
    assert!(note.contains(path));
}

#[cfg(unix)]
#[test]
fn stderr_file_requires_a_regular_file_and_follows_regular_file_symlinks() {
    use std::os::unix::fs::symlink;

    let temp = TempDir::new().unwrap();
    let file = temp.path().join("cuts.jsonl");
    let target = temp.path().join("stderr.txt");
    let link = temp.path().join("stderr-link.txt");
    std::fs::write(&target, "ordinary stderr").unwrap();
    symlink(&target, &link).unwrap();
    let added: SuccessEnvelope<AddData> = success(
        &command()
            .arg("--file")
            .arg(&file)
            .args(["add", "symlink evidence", "--stderr-file"])
            .arg(&link)
            .output()
            .unwrap(),
    );
    assert_eq!(
        added.data.record.evidence.unwrap().stderr.as_deref(),
        Some("ordinary stderr")
    );

    let fifo = temp.path().join("stderr.fifo");
    let made_fifo = std::process::Command::new("mkfifo")
        .arg(&fifo)
        .status()
        .is_ok_and(|status| status.success());
    if made_fifo {
        let rejected = run_file(
            &file,
            &[
                "add",
                "fifo evidence",
                "--stderr-file",
                fifo.to_str().unwrap(),
            ],
        );
        let envelope = error(&rejected, 65, "invalid_input");
        assert!(envelope.error.message.contains("not a regular file"));
        assert!(envelope.error.suggested_fix.contains("FIFOs and devices"));
    }
}

#[test]
fn evidence_and_resolve_response_shapes_are_exactly_compatible() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("cuts.jsonl");
    let added = add(&file, "no evidence");
    let add_json: Value =
        serde_json::from_slice(&run_file(&file, &["add", "another", "--agent", "tester"]).stdout)
            .unwrap();
    let add_data = add_json["data"].as_object().unwrap();
    assert_eq!(
        add_data.keys().map(String::as_str).collect::<Vec<_>>(),
        ["changed", "record"]
    );
    let record = add_data["record"].as_object().unwrap();
    assert_eq!(
        record.keys().map(String::as_str).collect::<Vec<_>>(),
        [
            "agent", "cwd", "id", "kind", "repo", "severity", "tags", "text", "ts"
        ]
    );
    assert!(record.get("evidence").is_none());
    let log_text = std::fs::read_to_string(&file).unwrap();
    let log: Value = serde_json::from_str(log_text.lines().next().unwrap()).unwrap();
    assert_eq!(log, serde_json::to_value(&added.data.record).unwrap());

    let partial: SuccessEnvelope<AddData> = success(&run_file(
        &file,
        &[
            "add",
            "partial evidence",
            "--agent",
            "tester",
            "--exit",
            "1",
        ],
    ));
    assert_eq!(
        serde_json::to_value(partial.data.record.evidence.unwrap())
            .unwrap()
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["exit"]
    );

    let one: Value =
        serde_json::from_slice(&run_file(&file, &["resolve", &added.data.record.id]).stdout)
            .unwrap();
    assert_eq!(
        one["data"]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["changed", "record"]
    );
    let second = partial.data.record.id;
    let third: SuccessEnvelope<AddData> =
        success(&run_file(&file, &["add", "third", "--agent", "tester"]));
    let many: Value = serde_json::from_slice(
        &run_file(&file, &["resolve", &second, &third.data.record.id]).stdout,
    )
    .unwrap();
    assert_eq!(
        many["data"]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["changed", "records"]
    );
}

#[test]
fn duplicate_add_warns_that_later_evidence_was_cut() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("cuts.jsonl");
    let first = command()
        .arg("--file")
        .arg(&file)
        .args(["add", "same", "--agent", "tester", "--evidence", "first"])
        .output()
        .unwrap();
    let first: SuccessEnvelope<AddData> = success(&first);
    let second = command()
        .arg("--file")
        .arg(&file)
        .args(["add", "same", "--agent", "tester", "--evidence", "later"])
        .output()
        .unwrap();
    let second: SuccessEnvelope<AddData> = success(&second);
    assert!(!second.data.changed);
    assert_eq!(second.data.record.id, first.data.record.id);
    assert_eq!(second.meta.warnings.len(), 1);
    assert!(second.meta.warnings[0].starts_with("duplicate_cut:"));
    assert!(second.meta.warnings[0].contains("later evidence was not stored"));
    assert_eq!(
        second.data.record.evidence.unwrap().note.as_deref(),
        Some("first")
    );
    assert_eq!(std::fs::read_to_string(&file).unwrap().lines().count(), 1);

    let no_evidence: SuccessEnvelope<AddData> = success(
        &command()
            .arg("--file")
            .arg(&file)
            .args(["add", "same", "--agent", "tester"])
            .output()
            .unwrap(),
    );
    assert_eq!(
        no_evidence.meta.warnings,
        ["duplicate_cut: existing record returned"]
    );
}

#[test]
fn add_resolution_text_warns_without_blocking() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("cuts.jsonl");
    let added: SuccessEnvelope<AddData> = success(
        &command()
            .arg("--file")
            .arg(&file)
            .args(["add", "  RESOLVED: fixed", "--agent", "tester"])
            .output()
            .unwrap(),
    );
    assert!(added.data.changed);
    assert!(added.meta.warnings.iter().any(|warning| {
        warning.starts_with("resolution_text:") && warning.contains("papercuts resolve <id>")
    }));
}

#[test]
fn every_command_success_envelope_deserializes() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("cuts.jsonl");
    let added = add(&file, "first cut");
    assert!(added.ok);
    assert!(added.data.changed);
    assert_eq!(added.data.record.ts, "2026-07-09T18:30:00.123Z");
    assert_eq!(added.meta.agent_source.as_deref(), Some("flag"));

    let listed: SuccessEnvelope<ListData> = success(&run_file(&file, &["list", "--status", "all"]));
    assert_eq!(listed.data.count, 1);

    let resolved: SuccessEnvelope<ResolveData> = success(&run_file(
        &file,
        &[
            "resolve",
            &added.data.record.id,
            "--agent",
            "fixer",
            "--note",
            "fixed",
        ],
    ));
    assert!(resolved.data.changed);
    assert_eq!(resolved.data.record.status, ItemStatus::Resolved);
    assert_eq!(
        resolved.data.record.resolution.unwrap().note.as_deref(),
        Some("fixed")
    );

    let doctor_output = run_file(&file, &["doctor"]);
    let doctor: SuccessEnvelope<DoctorData> = success(&doctor_output);
    assert!(doctor.data.healthy);
    assert_eq!(doctor.data.checked_lines, 2);

    let schema: SuccessEnvelope<Value> = success(&run(&["schema"]));
    assert_eq!(schema.data["contract"], 1);
    assert_eq!(schema.data["exit_codes"]["74"], "I/O error");
    assert_eq!(schema.data["commands"]["doctor"]["read_only"], true);
    assert!(
        schema.data["commands"]["add"]["flags"]["--stderr-file"]
            .as_str()
            .unwrap()
            .contains("4096")
    );
    assert_eq!(
        schema.data["commands"]["resolve"]["output"]["two_or_more"],
        "{changed,records:[...]}; IDs are canonicalized, sorted, and duplicate inputs collapse"
    );

    let expected = serde_json::to_value(exit_code_map()).unwrap();
    assert_eq!(schema.data["exit_codes"], expected);
}

#[test]
fn add_stdin_validation_duplicate_and_exact_id() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("cuts.jsonl");
    let mut stdin = command();
    let output = stdin
        .arg("--file")
        .arg(&file)
        .args([
            "add",
            "-",
            "--agent",
            "tester",
            "--severity",
            "major",
            "--tag",
            "z",
            "--tag",
            "a",
        ])
        .write_stdin("ouch\n")
        .output()
        .unwrap();
    let first: SuccessEnvelope<AddData> = success(&output);
    assert_eq!(first.data.record.id, "pc_6d26611bad4c");
    assert_eq!(first.data.record.tags, ["a", "z"]);

    let second: SuccessEnvelope<AddData> = success(
        &command()
            .arg("--file")
            .arg(&file)
            .args([
                "add",
                "ouch",
                "--agent",
                "tester",
                "--severity",
                "major",
                "--tag",
                "z",
                "--tag",
                "a",
            ])
            .output()
            .unwrap(),
    );
    assert!(!second.data.changed);
    assert_eq!(second.meta.warnings.len(), 1);
    assert_eq!(std::fs::read_to_string(&file).unwrap().lines().count(), 1);

    let blank = command()
        .arg("--file")
        .arg(&file)
        .arg("add")
        .write_stdin(" \n")
        .output()
        .unwrap();
    error(&blank, 65, "invalid_input");
    let large = "x".repeat(10_001);
    error(&run_file(&file, &["add", &large]), 65, "invalid_input");
}

#[test]
fn list_filters_sorts_limits_since_and_markdown() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("cuts.jsonl");
    let cases = [
        ("2026-07-01T00:00:00Z", "old blocker", "blocker", "ops"),
        ("2026-07-09T17:00:00Z", "new minor", "minor", "shell"),
        ("2026-07-09T18:00:00Z", "new major", "major", "ops"),
    ];
    for (now, text, severity, tag) in cases {
        let output = command()
            .env("PAPERCUTS_NOW", now)
            .arg("--file")
            .arg(&file)
            .args([
                "add",
                text,
                "--agent",
                "tester",
                "--severity",
                severity,
                "--tag",
                tag,
            ])
            .output()
            .unwrap();
        success::<AddData>(&output);
    }
    let limited: SuccessEnvelope<ListData> = success(&run_file(&file, &["list", "--limit", "1"]));
    assert_eq!(limited.data.items[0].cut.text, "old blocker");
    assert_eq!(limited.data.total, 3);
    assert!(limited.data.truncated);

    let since: SuccessEnvelope<ListData> = success(
        &command()
            .env("PAPERCUTS_NOW", "2026-07-09T19:00:00Z")
            .arg("--file")
            .arg(&file)
            .args(["list", "--since", "2h", "--tag", "ops"])
            .output()
            .unwrap(),
    );
    assert_eq!(since.data.items.len(), 1);
    assert_eq!(since.data.items[0].cut.text, "new major");

    let markdown = run_file(&file, &["list", "--format", "md", "--severity", "major"]);
    assert!(markdown.status.success());
    assert!(markdown.stderr.is_empty());
    let markdown = String::from_utf8(markdown.stdout).unwrap();
    assert!(markdown.starts_with("## Major\n"));
    assert!(markdown.contains("new major — tester"));
    assert!(serde_json::from_str::<Value>(&markdown).is_err());
    error(
        &run_file(&file, &["list", "--since", "2026-07-09"]),
        2,
        "invalid_argument",
    );
}

#[test]
fn list_sorts_rfc3339_offsets_by_instant_not_text() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("offsets.jsonl");
    let earlier = json!({"kind":"cut","id":"pc_111111111111","ts":"2026-07-09T10:00:00+02:00","agent":"a","text":"earlier","tags":[],"severity":"minor","cwd":"/tmp","repo":null});
    let later = json!({"kind":"cut","id":"pc_222222222222","ts":"2026-07-09T09:00:00Z","agent":"a","text":"later","tags":[],"severity":"minor","cwd":"/tmp","repo":null});
    std::fs::write(&file, format!("{earlier}\n{later}\n")).unwrap();
    let listed: SuccessEnvelope<ListData> = success(&run_file(&file, &["list"]));
    assert_eq!(listed.data.items[0].cut.text, "later");
}

#[test]
fn resolve_prefix_errors_and_idempotence_are_structured() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("cuts.jsonl");
    let added = add(&file, "resolve me");
    let id = added.data.record.id;
    let prefix = &id[3..7];
    let first: SuccessEnvelope<ResolveData> = success(&run_file(
        &file,
        &["resolve", &prefix.to_ascii_uppercase(), "--agent", "fixer"],
    ));
    assert!(first.data.changed);
    let second: SuccessEnvelope<ResolveData> =
        success(&run_file(&file, &["resolve", &id, "--agent", "fixer"]));
    assert!(!second.data.changed);
    assert_eq!(second.meta.warnings, ["already resolved"]);

    error(&run_file(&file, &["resolve", "abc"]), 2, "invalid_argument");
    error(&run_file(&file, &["resolve", "deadbeef"]), 66, "not_found");

    let ambiguous = temp.path().join("ambiguous.jsonl");
    let lines = ["pc_abcd00000000", "pc_abcd11111111"]
        .map(|id| {
            json!({"kind":"cut","id":id,"ts":"2026-07-09T00:00:00.000Z","agent":"a","text":id,"tags":[],"severity":"minor","cwd":"/tmp","repo":null}).to_string()
        })
        .join("\n")
        + "\n";
    std::fs::write(&ambiguous, lines).unwrap();
    let envelope = error(
        &run_file(&ambiguous, &["resolve", "abcd"]),
        65,
        "ambiguous_id",
    );
    assert_eq!(
        envelope.error.details["candidates"],
        json!(["pc_abcd00000000", "pc_abcd11111111"])
    );
}

#[test]
fn multi_resolve_is_atomic_deterministic_and_idempotent() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("cuts.jsonl");
    let first = add(&file, "multi first").data.record.id;
    let second = add(&file, "multi second").data.record.id;
    let before = std::fs::read(&file).unwrap();

    let invalid = run_file(&file, &["resolve", &first, "deadbeef", "--agent", "fixer"]);
    error(&invalid, 66, "not_found");
    assert_eq!(std::fs::read(&file).unwrap(), before);

    let resolved: SuccessEnvelope<ResolveManyData> = success(&run_file(
        &file,
        &[
            "resolve", &second, &first, "--agent", "fixer", "--note", "batch",
        ],
    ));
    assert!(resolved.data.changed);
    assert_eq!(resolved.data.records.len(), 2);
    let mut expected = vec![first.clone(), second.clone()];
    expected.sort();
    assert_eq!(
        resolved
            .data
            .records
            .iter()
            .map(|record| record.cut.id.clone())
            .collect::<Vec<_>>(),
        expected
    );
    assert_eq!(std::fs::read_to_string(&file).unwrap().lines().count(), 4);

    let events: Vec<Value> = std::fs::read_to_string(&file)
        .unwrap()
        .lines()
        .skip(2)
        .map(serde_json::from_str)
        .collect::<Result<_, _>>()
        .unwrap();
    assert_eq!(
        events
            .iter()
            .map(|event| event["kind"].as_str())
            .collect::<Vec<_>>(),
        [Some("resolve"), Some("resolve")]
    );
    assert_eq!(
        events
            .iter()
            .map(|event| event["id"].as_str())
            .collect::<Vec<_>>(),
        expected
            .iter()
            .map(|id| Some(id.as_str()))
            .collect::<Vec<_>>()
    );
    for event in &events {
        assert_eq!(event["agent"], "fixer");
        assert_eq!(event["note"], "batch");
    }
    let listed: SuccessEnvelope<ListData> =
        success(&run_file(&file, &["list", "--status", "resolved"]));
    assert_eq!(listed.data.items.len(), 2);
    assert!(listed.data.items.iter().all(|item| {
        item.resolution.as_ref().is_some_and(|resolution| {
            resolution.agent == "fixer" && resolution.note.as_deref() == Some("batch")
        })
    }));

    let duplicate: SuccessEnvelope<ResolveManyData> = success(&run_file(
        &file,
        &["resolve", &first, &first, "--agent", "fixer"],
    ));
    assert!(!duplicate.data.changed);
    assert_eq!(duplicate.data.records.len(), 1);
    assert_eq!(duplicate.meta.warnings, ["already resolved"]);
    assert_eq!(std::fs::read_to_string(&file).unwrap().lines().count(), 4);
}

#[test]
fn multi_resolve_heals_a_torn_tail_and_keeps_first_resolution() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("cuts.jsonl");
    let first = add(&file, "first torn batch").data.record.id;
    let second = add(&file, "second torn batch").data.record.id;
    let mut torn = OpenOptions::new().append(true).open(&file).unwrap();
    write!(torn, "{{\"kind\":").unwrap();
    drop(torn);
    let _: SuccessEnvelope<ResolveManyData> = success(&run_file(
        &file,
        &[
            "resolve", &second, &first, "--agent", "fixer", "--note", "first",
        ],
    ));
    let log = std::fs::read_to_string(&file).unwrap();
    assert!(log.ends_with('\n'));
    let listed: SuccessEnvelope<ListData> =
        success(&run_file(&file, &["list", "--status", "resolved"]));
    assert_eq!(listed.data.items.len(), 2);
    assert!(listed.data.items.iter().all(|item| {
        item.resolution
            .as_ref()
            .is_some_and(|resolution| resolution.note.as_deref() == Some("first"))
    }));
    let first_resolution = json!({"kind":"resolve","id":first,"ts":"2026-07-09T18:30:00.123Z","agent":"later","note":"later"});
    std::fs::write(&file, format!("{log}{first_resolution}\n")).unwrap();
    let listed: SuccessEnvelope<ListData> =
        success(&run_file(&file, &["list", "--status", "resolved"]));
    let first_item = listed
        .data
        .items
        .iter()
        .find(|item| item.cut.id == first)
        .unwrap();
    assert_eq!(
        first_item.resolution.as_ref().unwrap().note.as_deref(),
        Some("first")
    );
}

#[test]
fn concurrent_multi_resolves_share_one_critical_section() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("cuts.jsonl");
    let first = add(&file, "concurrent multi first").data.record.id;
    let second = add(&file, "concurrent multi second").data.record.id;
    let barrier = Arc::new(Barrier::new(4));
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let file = file.clone();
            let first = first.clone();
            let second = second.clone();
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                barrier.wait();
                let output = run_file(&file, &["resolve", &first, &second, "--agent", "race"]);
                let envelope: SuccessEnvelope<ResolveManyData> = success(&output);
                envelope.data.changed
            })
        })
        .collect();
    let changed = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .filter(|changed| *changed)
        .count();
    assert_eq!(changed, 1);
    assert_eq!(std::fs::read_to_string(&file).unwrap().lines().count(), 4);
}

#[test]
fn structured_error_exit_matrix_and_help_exceptions() {
    let temp = TempDir::new().unwrap();
    let missing = temp.path().join("missing.jsonl");
    error(&run_file(&missing, &["list"]), 66, "not_found");
    error(&run(&["list", "--format", "jsonl"]), 2, "invalid_argument");
    error(
        &command()
            .env("PAPERCUTS_NOW", "not-a-time")
            .args(["schema"])
            .output()
            .unwrap(),
        78,
        "config_error",
    );
    error(
        &run_file(&missing, &["add", " ", "--agent", "tester"]),
        65,
        "invalid_input",
    );
    let invalid_utf8 = command()
        .arg("--file")
        .arg(&missing)
        .args(["add", "-", "--agent", "tester"])
        .write_stdin(vec![0xff])
        .output()
        .unwrap();
    error(&invalid_utf8, 65, "invalid_input");
    let directory_error = run_file(temp.path(), &["list"]);
    error(&directory_error, 74, "io_error");

    let help = run(&["--help"]);
    assert!(help.status.success());
    assert!(help.stderr.is_empty());
    assert!(String::from_utf8_lossy(&help.stdout).contains("Usage:"));
    let version = run(&["--version"]);
    assert!(version.status.success());
    assert_eq!(
        String::from_utf8_lossy(&version.stdout),
        "papercuts 0.1.0\n"
    );
}

#[test]
fn agent_resolution_order_and_sources_are_pinned() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("unused.jsonl");
    let invoke = |command: &mut Command| -> SuccessEnvelope<AddData> {
        success(
            &command
                .arg("--file")
                .arg(&file)
                .args(["add", "x", "--dry-run"])
                .output()
                .unwrap(),
        )
    };

    let default = invoke(&mut command());
    assert_eq!(default.data.record.agent, "unknown");
    assert_eq!(default.meta.agent_source.as_deref(), Some("default"));

    let claude = invoke(command().env("CLAUDECODE", "1"));
    assert_eq!(claude.data.record.agent, "claude-code");
    assert_eq!(claude.meta.agent_source.as_deref(), Some("detected"));

    let codex = invoke(command().env("CODEX_TEST", "1").env("CURSOR_TEST", "1"));
    assert_eq!(codex.data.record.agent, "codex");

    let cursor = invoke(command().env("CURSOR_TEST", "1"));
    assert_eq!(cursor.data.record.agent, "cursor");

    let env = invoke(
        command()
            .env("PAPERCUTS_AGENT", "from-env")
            .env("CLAUDECODE", "1"),
    );
    assert_eq!(env.data.record.agent, "from-env");
    assert_eq!(env.meta.agent_source.as_deref(), Some("env"));

    let flag: SuccessEnvelope<AddData> = success(
        &command()
            .env("PAPERCUTS_AGENT", "from-env")
            .arg("--file")
            .arg(&file)
            .args(["add", "x", "--agent", "from-flag", "--dry-run"])
            .output()
            .unwrap(),
    );
    assert_eq!(flag.data.record.agent, "from-flag");
    assert_eq!(flag.meta.agent_source.as_deref(), Some("flag"));
    assert!(!file.exists());
}

#[test]
fn mutation_dry_runs_do_not_write() {
    let temp = TempDir::new().unwrap();
    let dry_add = temp.path().join("nested/cuts.jsonl");
    let added: SuccessEnvelope<AddData> = success(&run_file(
        &dry_add,
        &["add", "preview", "--agent", "a", "--dry-run"],
    ));
    assert!(!added.data.changed);
    assert!(!dry_add.exists());

    let file = temp.path().join("cuts.jsonl");
    let id = add(&file, "resolve preview").data.record.id;
    let before = std::fs::read(&file).unwrap();
    let resolved: SuccessEnvelope<ResolveData> = success(&run_file(
        &file,
        &["resolve", &id, "--agent", "a", "--dry-run"],
    ));
    assert!(!resolved.data.changed);
    assert_eq!(resolved.data.record.status, ItemStatus::Resolved);
    assert_eq!(std::fs::read(&file).unwrap(), before);
}

#[cfg(unix)]
#[test]
fn permission_denied_is_exit_77() {
    use std::os::unix::fs::PermissionsExt;
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("cuts.jsonl");
    std::fs::write(&file, "{}\n").unwrap();
    std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o000)).unwrap();
    let output = run_file(&file, &["list"]);
    std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o600)).unwrap();
    error(&output, 77, "permission_denied");
}

#[test]
fn lock_timeout_is_retryable_exit_75() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("cuts.jsonl");
    add(&file, "locked");
    let locked = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&file)
        .unwrap();
    locked.lock().unwrap();
    let output = run_file(&file, &["list"]);
    locked.unlock().unwrap();
    let envelope = error(&output, 75, "lock_timeout");
    assert!(envelope.error.retryable);
}

#[test]
fn doctor_reports_all_core_findings_and_recomputed_ids() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("cuts.jsonl");
    let good = add(&file, "valid").data.record;
    let good_line = std::fs::read_to_string(&file).unwrap();
    let bad_id = json!({"kind":"cut","id":"pc_000000000000","ts":good.ts,"agent":"tester","text":"bad","tags":[],"severity":"minor","cwd":"/tmp","repo":null});
    let mut writer = OpenOptions::new().append(true).open(&file).unwrap();
    writeln!(writer, "{good_line}{}", bad_id).unwrap();
    writeln!(writer, "{{\"kind\":\"future\"}}").unwrap();
    writeln!(writer, "{{\"kind\":\"resolve\",\"id\":\"pc_deadbeef0000\",\"ts\":\"2026-07-09T00:00:00.000Z\",\"agent\":\"a\",\"note\":null}}").unwrap();
    writeln!(writer, "<<<<<<< HEAD").unwrap();
    write!(writer, "{{\"kind\":").unwrap();
    drop(writer);
    let output = run_file(&file, &["doctor"]);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stderr.is_empty());
    let envelope: SuccessEnvelope<DoctorData> = serde_json::from_slice(&output.stdout).unwrap();
    let kinds: Vec<_> = envelope
        .data
        .findings
        .iter()
        .map(|finding| finding.kind.as_str())
        .collect();
    for kind in [
        "duplicate_cut",
        "id_conflict",
        "unknown_kind",
        "orphan_resolve",
        "conflict_marker",
        "torn_line",
    ] {
        assert!(kinds.contains(&kind), "missing {kind}: {kinds:?}");
    }
    assert!(!envelope.data.healthy);
}

#[test]
fn torn_tail_self_heals_on_add() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("cuts.jsonl");
    std::fs::write(&file, b"{\"kind\":\"cut\"").unwrap();
    let added = add(&file, "after tear");
    assert!(added.data.changed);
    let bytes = std::fs::read(&file).unwrap();
    assert!(bytes.ends_with(b"\n"));
    assert_eq!(bytes.split(|byte| *byte == b'\n').count(), 3);
    let listed: SuccessEnvelope<ListData> = success(&run_file(&file, &["list"]));
    assert_eq!(listed.data.items.len(), 1);
    assert_eq!(listed.data.items[0].cut.text, "after tear");
    assert!(
        listed
            .meta
            .warnings
            .iter()
            .any(|warning| warning.contains("malformed"))
    );
}

#[test]
fn doctor_finding_counts_match_fold_bytes_warning_counts() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("cuts.jsonl");
    let valid_id = compute_id(
        "2026-07-09T00:00:00.000Z",
        "a",
        "valid",
        Severity::Minor,
        &[],
    );
    let malformed = json!({
        "kind": "cut",
        "id": "pc_000000000000",
        "ts": "not-a-time",
        "agent": "a",
        "text": "malformed",
        "tags": [],
        "severity": "minor",
        "cwd": "/tmp",
        "repo": null
    })
    .to_string();
    let valid = json!({
        "kind": "cut",
        "id": valid_id,
        "ts": "2026-07-09T00:00:00.000Z",
        "agent": "a",
        "text": "valid",
        "tags": [],
        "severity": "minor",
        "cwd": "/tmp",
        "repo": null
    })
    .to_string();
    let orphan = json!({
        "kind": "resolve",
        "id": "pc_deadbeef0000",
        "ts": "2026-07-09T00:00:00.000Z",
        "agent": "a",
        "note": null
    })
    .to_string();
    let unknown = json!({"kind": "future"}).to_string();
    let fixture = format!("{malformed}\n{valid}\n{orphan}\n{valid}\n{unknown}\n{{\"kind\":");
    std::fs::write(&file, fixture).unwrap();

    let folded = papercuts::store::fold_bytes(&std::fs::read(&file).unwrap());
    let doctor_output = run_file(&file, &["doctor"]);
    assert_eq!(doctor_output.status.code(), Some(1));
    assert!(doctor_output.stderr.is_empty());
    let doctor: SuccessEnvelope<DoctorData> =
        serde_json::from_slice(&doctor_output.stdout).unwrap();

    let fold_counts = fold_warning_counts(&folded.warnings);
    let doctor_counts = doctor_finding_counts(&doctor.data.findings);
    let expected: HashMap<String, usize> = [
        ("malformed", 1),
        ("unknown", 1),
        ("duplicate_cut", 1),
        ("orphan_resolve", 1),
        ("torn", 1),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v))
    .collect();
    assert_eq!(
        fold_counts, expected,
        "fold warnings: {:?}",
        folded.warnings
    );
    assert_eq!(
        doctor_counts, expected,
        "doctor findings: {:?}",
        doctor.data.findings
    );
}

fn fold_warning_counts(warnings: &[String]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for warning in warnings {
        let parts: Vec<_> = warning.splitn(3, ' ').collect();
        let count: usize = parts[1].parse().unwrap();
        let label = parts[2].trim_end_matches('s');
        let key = if label.starts_with("malformed line") {
            "malformed"
        } else if label.starts_with("torn final line") {
            "torn"
        } else if label.starts_with("unknown event") {
            "unknown"
        } else if label.starts_with("duplicate cut") {
            "duplicate_cut"
        } else if label.starts_with("duplicate resolve") {
            "duplicate_resolve"
        } else if label.starts_with("orphan resolve") {
            "orphan_resolve"
        } else {
            panic!("unknown fold warning label: {label}")
        };
        counts.insert(key.to_string(), count);
    }
    counts
}

fn doctor_finding_counts(
    findings: &[papercuts::commands::doctor::Finding],
) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for finding in findings {
        let key = match finding.kind.as_str() {
            "malformed" => "malformed",
            "torn_line" => "torn",
            "unknown_kind" => "unknown",
            "duplicate_cut" => "duplicate_cut",
            "orphan_resolve" => "orphan_resolve",
            _ => continue,
        };
        *counts.entry(key.to_string()).or_insert(0) += 1;
    }
    counts
}

#[test]
fn discovery_precedence_virtual_empty_and_git_file_root() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("repo");
    let nested = root.join("a/b");
    std::fs::create_dir_all(&nested).unwrap();
    std::fs::write(root.join(".git"), "gitdir: elsewhere\n").unwrap();
    let env_file = temp.path().join("env.jsonl");
    let flag_file = temp.path().join("flag.jsonl");

    let walk: SuccessEnvelope<AddData> = success(
        &command()
            .current_dir(&nested)
            .args(["add", "x", "--agent", "a", "--dry-run"])
            .output()
            .unwrap(),
    );
    let canonical_root = root.canonicalize().unwrap();
    assert_eq!(
        walk.meta.file.as_deref(),
        Some(canonical_root.join(".papercuts.jsonl").to_str().unwrap())
    );
    let empty_env: SuccessEnvelope<AddData> = success(
        &command()
            .current_dir(&nested)
            .env("PAPERCUTS_FILE", "")
            .args(["add", "x", "--agent", "a", "--dry-run"])
            .output()
            .unwrap(),
    );
    assert_eq!(empty_env.meta.file, walk.meta.file);

    let env: SuccessEnvelope<AddData> = success(
        &command()
            .current_dir(&nested)
            .env("PAPERCUTS_FILE", &env_file)
            .args(["add", "x", "--agent", "a", "--dry-run"])
            .output()
            .unwrap(),
    );
    assert_eq!(env.meta.file.as_deref(), Some(env_file.to_str().unwrap()));

    let flag: SuccessEnvelope<AddData> = success(
        &command()
            .current_dir(&nested)
            .env("PAPERCUTS_FILE", &env_file)
            .arg("--file")
            .arg(&flag_file)
            .args(["add", "x", "--agent", "a", "--dry-run"])
            .output()
            .unwrap(),
    );
    assert_eq!(flag.meta.file.as_deref(), Some(flag_file.to_str().unwrap()));

    let empty: SuccessEnvelope<ListData> =
        success(&command().current_dir(&nested).arg("list").output().unwrap());
    assert!(empty.data.items.is_empty());
    assert!(
        empty
            .meta
            .warnings
            .iter()
            .any(|warning| warning.contains("no papercuts file"))
    );

    if !temp_has_git_ancestor(&temp) {
        let outside = temp.path().join("outside");
        let home = temp.path().join("home");
        std::fs::create_dir_all(&outside).unwrap();
        let home_result: SuccessEnvelope<AddData> = success(
            &command()
                .current_dir(&outside)
                .env("HOME", &home)
                .args(["add", "x", "--agent", "a", "--dry-run"])
                .output()
                .unwrap(),
        );
        assert_eq!(
            home_result.meta.file.as_deref(),
            Some(home.join(".papercuts/log.jsonl").to_str().unwrap())
        );
        assert!(
            !home.exists(),
            "dry run must not create the home fallback directory"
        );
        let no_home = command()
            .current_dir(&outside)
            .env_remove("HOME")
            .arg("list")
            .output()
            .unwrap();
        error(&no_home, 78, "config_error");
    } else {
        eprintln!(
            "skipping home-fallback assertions because the temporary directory is inside a git checkout"
        );
    }
}

#[test]
fn fixed_clock_fresh_state_is_byte_deterministic_and_retry_is_duplicate_safe() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("cuts.jsonl");
    let first = run_file(&file, &["add", "same", "--agent", "tester"]);
    assert!(first.status.success());
    std::fs::remove_file(&file).unwrap();
    let fresh = run_file(&file, &["add", "same", "--agent", "tester"]);
    assert_eq!(first.stdout, fresh.stdout);
    let retry: SuccessEnvelope<AddData> =
        success(&run_file(&file, &["add", "same", "--agent", "tester"]));
    assert!(!retry.data.changed);
}

#[test]
fn eight_way_distinct_add_race_loses_no_lines() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("cuts.jsonl");
    let barrier = Arc::new(Barrier::new(8));
    let handles: Vec<_> = (0..8)
        .map(|thread_id| {
            let file = file.clone();
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                barrier.wait();
                for item in 0..4 {
                    let text = format!("thread-{thread_id}-item-{item}");
                    let output = run_file(&file, &["add", &text, "--agent", "race"]);
                    assert!(
                        output.status.success(),
                        "{}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    let contents = std::fs::read_to_string(&file).unwrap();
    assert_eq!(contents.lines().count(), 32);
    for line in contents.lines() {
        serde_json::from_str::<Value>(line).unwrap();
    }
}

#[test]
fn eight_way_identical_add_race_appends_once() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("cuts.jsonl");
    let barrier = Arc::new(Barrier::new(8));
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let file = file.clone();
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                barrier.wait();
                let envelope: SuccessEnvelope<AddData> =
                    success(&run_file(&file, &["add", "identical", "--agent", "race"]));
                envelope.data.changed
            })
        })
        .collect();
    let changed = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .filter(|changed| *changed)
        .count();
    assert_eq!(changed, 1);
    assert_eq!(std::fs::read_to_string(&file).unwrap().lines().count(), 1);
}

#[test]
fn eight_way_resolve_race_appends_once() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("cuts.jsonl");
    let id = add(&file, "resolve race").data.record.id;
    let barrier = Arc::new(Barrier::new(8));
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let file = file.clone();
            let id = id.clone();
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                barrier.wait();
                let envelope: SuccessEnvelope<ResolveData> =
                    success(&run_file(&file, &["resolve", &id, "--agent", "race"]));
                envelope.data.changed
            })
        })
        .collect();
    let changed = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .filter(|changed| *changed)
        .count();
    assert_eq!(changed, 1);
    assert_eq!(std::fs::read_to_string(&file).unwrap().lines().count(), 2);
}

#[test]
fn hash_length_prefix_and_tag_sort_are_pinned() {
    let a = compute_id(
        "2026-07-09T18:30:00.123Z",
        "tester",
        "ouch",
        Severity::Major,
        &["a".into(), "z".into()],
    );
    let b = compute_id(
        "2026-07-09T18:30:00.123Z",
        "tester",
        "ouc",
        Severity::Major,
        &["z".into(), "ha".into()],
    );
    let unsorted = compute_id(
        "2026-07-09T18:30:00.123Z",
        "tester",
        "ouch",
        Severity::Major,
        &["z".into(), "a".into()],
    );
    assert_eq!(a, "pc_6d26611bad4c");
    assert_eq!(a, unsorted);
    assert_ne!(a, b);
}

#[test]
fn env_papercuts_file_nonexistent_returns_not_found() {
    let temp = TempDir::new().unwrap();
    let missing = temp.path().join("missing.jsonl");
    let output = command()
        .env("PAPERCUTS_FILE", &missing)
        .arg("list")
        .output()
        .unwrap();
    error(&output, 66, "not_found");
}

#[test]
fn relative_file_resolves_against_cwd() {
    let temp = TempDir::new().unwrap();
    let output = command()
        .current_dir(temp.path())
        .arg("--file")
        .arg("rel/path.jsonl")
        .args(["add", "x", "--agent", "a", "--dry-run"])
        .output()
        .unwrap();
    let envelope: SuccessEnvelope<AddData> = success(&output);
    let temp_canonical = temp.path().canonicalize().unwrap();
    assert!(
        Path::new(envelope.meta.file.as_deref().unwrap()).starts_with(&temp_canonical),
        "meta.file = {:?}",
        envelope.meta.file
    );
}

#[test]
fn markdown_format_is_byte_deterministic() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("cuts.jsonl");
    let added = add(&file, "determinism");
    let first = run_file(&file, &["list", "--format", "md"]);
    assert!(first.status.success());
    assert!(!first.stdout.is_empty());
    let first_text = String::from_utf8_lossy(&first.stdout);
    assert!(first_text.contains("determinism"));
    assert!(first_text.contains(&added.data.record.id));
    let second = run_file(&file, &["list", "--format", "md"]);
    assert!(second.status.success());
    assert_eq!(first.stdout, second.stdout);
}

#[test]
fn doctor_reports_gitignored_finding() {
    let git_available = std::process::Command::new("git")
        .arg("--version")
        .output()
        .is_ok_and(|output| output.status.success());
    if !git_available {
        return;
    }

    let temp = TempDir::new().unwrap();
    let repo = temp.path().join("repo");
    std::fs::create_dir_all(&repo).unwrap();
    assert!(
        std::process::Command::new("git")
            .arg("-C")
            .arg(&repo)
            .arg("init")
            .output()
            .unwrap()
            .status
            .success()
    );
    std::fs::write(repo.join(".gitignore"), ".papercuts.jsonl\n").unwrap();

    let output = command()
        .current_dir(&repo)
        .args(["add", "gitignored cut", "--agent", "a"])
        .output()
        .unwrap();
    success::<AddData>(&output);

    let doctor_output = command().current_dir(&repo).arg("doctor").output().unwrap();
    assert_eq!(doctor_output.status.code(), Some(1));
    assert!(doctor_output.stderr.is_empty());
    let doctor: SuccessEnvelope<DoctorData> =
        serde_json::from_slice(&doctor_output.stdout).unwrap();
    assert!(!doctor.data.healthy);
    assert!(
        doctor
            .data
            .findings
            .iter()
            .any(|finding| finding.kind == "gitignored")
    );
}

#[test]
fn error_envelope_matrix() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("cuts.jsonl");
    let missing = temp.path().join("missing.jsonl");
    let outside = temp.path().join("outside");
    std::fs::create_dir_all(&outside).unwrap();

    let ambiguous = temp.path().join("ambiguous.jsonl");
    let lines = ["pc_abcd00000000", "pc_abcd11111111"]
        .map(|id| {
            json!({"kind":"cut","id":id,"ts":"2026-07-09T00:00:00.000Z","agent":"a","text":id,"tags":[],"severity":"minor","cwd":"/tmp","repo":null}).to_string()
        })
        .join("\n")
        + "\n";
    std::fs::write(&ambiguous, lines).unwrap();

    error(&run(&["list", "--format", "jsonl"]), 2, "invalid_argument");
    error(
        &run_file(&file, &["add", " ", "--agent", "tester"]),
        65,
        "invalid_input",
    );
    error(&run_file(&missing, &["list"]), 66, "not_found");
    if temp_has_git_ancestor(&temp) {
        eprintln!(
            "skipping HOME/config-78 assertion because the temporary directory is inside a git checkout"
        );
    } else {
        error(
            &command()
                .current_dir(&outside)
                .env("HOME", "")
                .arg("list")
                .output()
                .unwrap(),
            78,
            "config_error",
        );
    }
    error(
        &run_file(&ambiguous, &["resolve", "abcd"]),
        65,
        "ambiguous_id",
    );
}
