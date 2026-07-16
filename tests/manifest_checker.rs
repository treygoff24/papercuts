#![cfg(unix)]

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use tempfile::TempDir;

const LOST_IDS: [&str; 4] = [
    "pc_944d374ac9c4",
    "pc_8c2350511589",
    "pc_df6af25a100a",
    "pc_f8eb38d950f5",
];

struct ManifestRow {
    id: String,
    wave: String,
    end_state: String,
}

#[test]
fn manifest_lost_wave_6_resolutions_require_explicit_attestation() {
    let temp = TempDir::new().unwrap();
    let log = write_state_log(&temp, "6", None);
    let unattested = run_gate(&log, "6", &[]);
    let stderr = String::from_utf8_lossy(&unattested.stderr);
    assert!(!unattested.status.success());
    assert!(stderr.contains("state mismatch for pc_df6af25a100a: expected=resolved, actual=open"));
    assert!(stderr.contains("state mismatch for pc_f8eb38d950f5: expected=resolved, actual=open"));

    let output = run_gate(
        &log,
        "6",
        &[
            "--attest-lost-resolved",
            "pc_df6af25a100a",
            "--attest-lost-resolved",
            "pc_f8eb38d950f5",
        ],
    );

    assert!(
        output.status.success(),
        "stdout={}\nstderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("ledger-lost ID counted at attested status (resolved): pc_df6af25a100a")
    );
    assert!(
        stdout.contains("ledger-lost ID counted at attested status (resolved): pc_f8eb38d950f5")
    );
    assert!(stdout.contains(
        "manifest PASS: 132 unique diagnostic IDs; live snapshot coverage=132/132; after-wave=6"
    ));
}

#[test]
fn manifest_unattested_condition_requires_open_state() {
    let temp = TempDir::new().unwrap();
    let log = write_state_log(&temp, "5", Some("pc_b8fe2e571b1f"));
    let output = run_gate(&log, "5", &[]);

    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr)
            .contains("state mismatch for pc_b8fe2e571b1f: expected=open, actual=resolved"),
        "stdout={}\nstderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn run_gate(log: &Path, wave: &str, extra: &[&str]) -> Output {
    Command::new("sh")
        .arg(repo_root().join("scripts/check-manifest.sh"))
        .args(["--after-wave", wave, "--log"])
        .arg(log)
        .args(["--accept-harness", "claude", "--defer-harness", "codex"])
        .args(extra)
        .env("PAPERCUTS_BIN", env!("CARGO_BIN_EXE_papercuts"))
        .output()
        .unwrap()
}

fn write_state_log(temp: &TempDir, wave: &str, force_resolved: Option<&str>) -> PathBuf {
    let manifest =
        std::fs::read_to_string(repo_root().join("docs/plans/papercuts-remediation-manifest.md"))
            .unwrap();
    let rows = manifest_rows(&manifest);
    let conditions = manifest_conditions(&manifest);
    let lost: HashSet<_> = LOST_IDS.into_iter().collect();
    let mut lines = Vec::new();

    for row in rows.iter().filter(|row| !lost.contains(row.id.as_str())) {
        lines.push(
            serde_json::json!({
                "kind": "cut",
                "id": row.id,
                "ts": "2026-07-16T00:00:00.000Z",
                "agent": "manifest-test",
                "text": "fixture",
                "tags": [],
                "severity": "minor",
                "cwd": "/tmp",
                "repo": null
            })
            .to_string(),
        );
        if row_is_resolved(row, conditions.get(&row.id), wave)
            || force_resolved == Some(row.id.as_str())
        {
            lines.push(
                serde_json::json!({
                    "kind": "resolve",
                    "id": row.id,
                    "ts": "2026-07-16T00:01:00.000Z",
                    "agent": "manifest-test",
                    "note": null
                })
                .to_string(),
            );
        }
    }

    let log = temp.path().join("state.jsonl");
    std::fs::write(&log, format!("{}\n", lines.join("\n"))).unwrap();
    log
}

fn row_is_resolved(row: &ManifestRow, condition: Option<&String>, target_wave: &str) -> bool {
    if row.end_state == "already-resolved" {
        return true;
    }
    if row.end_state.starts_with("stays-open-") || condition.is_some() {
        return condition.is_some_and(|value| value == "shell:claude");
    }
    wave_rank(target_wave) >= wave_rank(&row.wave)
}

fn manifest_rows(manifest: &str) -> Vec<ManifestRow> {
    manifest
        .lines()
        .skip_while(|line| *line != "## ID manifest")
        .skip(1)
        .take_while(|line| !line.starts_with("## "))
        .filter(|line| line.starts_with("| `pc_"))
        .map(|line| {
            let fields = table_fields(line);
            ManifestRow {
                id: fields[1].clone(),
                wave: fields[3].clone(),
                end_state: fields[4].clone(),
            }
        })
        .collect()
}

fn manifest_conditions(manifest: &str) -> HashMap<String, String> {
    manifest
        .lines()
        .skip_while(|line| *line != "## Named resolution conditions")
        .skip(1)
        .take_while(|line| !line.starts_with("## "))
        .filter(|line| line.starts_with("| `pc_"))
        .map(|line| {
            let fields = table_fields(line);
            (fields[1].clone(), fields[2].clone())
        })
        .collect()
}

fn table_fields(line: &str) -> Vec<String> {
    line.split('|')
        .map(|field| field.trim().trim_matches('`').to_owned())
        .collect()
}

fn wave_rank(wave: &str) -> u8 {
    match wave {
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4a" => 4,
        "5" => 5,
        "4b" => 6,
        "6" => 7,
        "7" => 8,
        "8" => 9,
        _ => 0,
    }
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
