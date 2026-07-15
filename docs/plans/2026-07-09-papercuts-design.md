# papercuts — design doc

2026-07-09. Coordinator-authored. Status: r4 — amended for Wave 2 evidence and multi-resolve behavior. See Amendments for full triage provenance.

## Thesis and provenance

Agents hit friction constantly — dead-end tool calls, broken links, missing helpers, footgun configs — and silently push through without telling anyone. The signal evaporates. `papercuts` is a tiny agent-first CLI that gives agents a one-line way to file the complaint at the moment they hit it, and gives humans (and other agents) a way to review and burn down the backlog.

Provenance: Steve Ruiz shipped a private version of this inside his repo (X post, 2026-07-09, 39K views / 770 bookmarks in hours) and reported it immediately surfaced real workflow defects his agents had been eating silently: unquoted zsh globs breaking `rg`, wrong test cwd in a yarn workspace, tab-indented YAML breaking deploys, stale Supabase CLI ambiguity. Every one is an actionable fix a human would never have heard about otherwise. This is a validated behavior pattern, not a speculative product.

Why a CLI and not an MCP server or harness feature: every agent harness (Claude Code, Codex, Cursor, Droid, anything) can shell out. A single static binary with a JSON contract is the lowest common denominator and needs zero per-harness integration. One line in an AGENTS.md/CLAUDE.md activates it.

## External contract

Binary and crate: `papercuts` (crates.io name verified free 2026-07-09; bare `papercut` is taken by an image tool). Repo: `treygoff24/papercuts`. License: MIT.

### Commands

```text
papercuts add <TEXT | ->        # file a papercut ('-' reads text from stdin)
papercuts list                  # read papercuts (default: open only, severity-first then newest)
papercuts resolve <ID>...       # mark one or more papercuts resolved (append-only events)
papercuts schema [all|record|error|exit-codes]   # machine contract, self-orientation
papercuts doctor                # validate the log file (diagnose-only)
```

`log` is an alias of `add` (the verb people will guess from Steve's post); `add` is canonical.

Global flags: `--file <PATH>` (explicit log file, overrides discovery; relative paths resolve against cwd) and `--pretty` (pretty-print JSON output; no-op for `--format md`). No `--quiet` in v1 (cut — its interaction with data output was underspecified; warnings ride in `meta`, which is already machine-skippable). No color anywhere, ever (agent-only tool; there is nothing to colorize — output is JSON).

### `add`

- **Duplicate-safe, not retry-idempotent** (r3 correction): the content-addressed ID exists for determinism and merge self-healing, NOT as an exactly-once mechanism — a retry at a later wall-clock second produces a new ts, hence a new ID and a second cut, and that is accepted v1 behavior. If the computed ID already exists in the log (fixed-clock tests, post-merge duplicate lines, byte-identical racing adds), nothing is appended and the existing record is returned with `data.changed: false` plus a warning. A caller-supplied idempotency key is an explicit non-goal for v1.
- The ID check and the append happen inside one exclusive-lock critical section (lock → read+fold → decide → append) — two racing identical adds cannot both append.
- `--dry-run`: full discovery, agent resolution, validation, and record construction; reports the would-be record with `data.changed: false`; creates no file and no directory.
- Text is bounded: max 10,000 bytes after trailing-newline strip; larger is `invalid_input` (exit 65).
- Positional `TEXT` (or `-` for stdin; stdin also used when text is omitted and stdin is non-TTY).
- `--agent <NAME>`: reporter identity. Resolution order: flag → `PAPERCUTS_AGENT` env → harness detection (`CLAUDECODE`→`claude-code`, `CODEX_*`→`codex`, `CURSOR_*`→`cursor`) → `"unknown"`. The resolved value AND its source (`flag|env|detected|default`) are echoed in output meta — no silent ambient inference.
- `--tag <TAG>` (repeatable), `--severity minor|major|blocker` (default `minor`).
- Evidence flags are optional: `--cmd TEXT`, `--exit N`, `--stderr-file PATH` (read at filing time and stored as at most 4096 UTF-8 bytes), and `--evidence TEXT`. Evidence is best-effort redacted at write time; never feed raw environment dumps. Evidence is not part of the ID.
- A missing stderr path is `not_found` (66), permission failures are `permission_denied` (77), other read failures are `io_error` (74), and invalid UTF-8 is `invalid_input` (65); no private error codes are emitted.
- A text value beginning with optional whitespace followed by `RESOLUTION` or `RESOLVED` succeeds with a `resolution_text` warning suggesting `papercuts resolve <id>`; it is never blocked.
- Captures `cwd` and repo root automatically (filesystem walk for `.git`; no libgit2).
- Output: success envelope containing the full record + `meta.file` (resolved log path) + `meta.agent_source`.

### `list`

- Filters: `--status open|resolved|all` (default `open`), `--agent`, `--tag`, `--severity`, `--since <RFC3339 | Nd | Nh>`. All filters inspect **cut** fields (`--since` compares the cut's `ts`, never the resolve's).
- `--limit N` (default 50) — bounded output by default; envelope carries `count` (items returned), `total` (matches before limit), `truncated` (`total > count`). The limit slices AFTER the normative sort, so `--limit 1` returns the highest-severity-then-newest match.
- `--format json|md` (default `json`; `jsonl` cut in r3 — one envelope is the contract). `md` is the **sole raw-output exception** in the tool: raw Markdown on stdout, no envelope, warnings as a trailing `> note:` blockquote.
- Empty result is exit 0 with an empty array and a hint in `meta.warnings` — never exit 1. A **missing log file at a discovered default location** is virtual empty state (exit 0, warning `"no papercuts file yet; papercuts add creates it"`); a missing file at an **explicit** `--file`/`PAPERCUTS_FILE` path is exit 66.

### `resolve`

- `papercuts resolve <ID>... [--note <TEXT>] [--agent <NAME>] [--dry-run]`. One ID keeps the `{changed,record}` output. Two or more IDs return `{changed,records:[...]}` in canonical ID order, with duplicate inputs collapsed. Appends one `resolve` event per open ID; never rewrites history. `--dry-run` reports what would be appended without writing. Output includes `data.changed: bool`.
- All IDs and prefixes are normalized and validated under one exclusive lock before any event is appended. Any invalid, ambiguous, or missing ID aborts the whole command with no partial append.
- The existence/status check and the append run inside one exclusive-lock critical section — two racing resolves of the same cut yield one `changed:true` and one already-resolved.
- Unknown ID → structured `not_found` error, exit 66, with a hint naming `papercuts list --status all`.
- Already-resolved ID → **idempotent success**, `data.changed: false`, `meta.warnings: ["already resolved"]`.
- ID prefix matching (normative): candidates are the distinct folded cut IDs (first-wins, including resolved cuts; orphan resolves are never candidates). A prefix is `pc_` optional + ≥4 hex digits, matched case-insensitively. Unique → resolves; ambiguous → `ambiguous_id` error listing full candidate IDs sorted ascending; <4 hex digits → `invalid_argument`.

### `schema`

Prints the full machine contract as JSON: contract version, every command/flag, record schemas, error codes, exit-code dictionary. This is the self-orientation surface; an agent that has never seen the tool runs `papercuts schema` and knows everything.

### `doctor` (v1: diagnose-only)

- Validates the log file: every line parses as a known event, IDs verified by **recomputation** (id must equal the hash of the record's fields); reports torn last line, git conflict-marker lines, unknown kinds, orphan resolves, duplicate cut lines — each as a structured finding `{line, kind, message}` with line numbers. A missing file at a discovered default is healthy-empty (exit 0 + note).
- Conflict-marker detection matches only complete physical marker lines (`<<<<<<< `/`>>>>>>> ` prefixes) — a cut whose *text* mentions conflict markers parses as valid JSON and is never flagged.
- Byte-identical duplicate cut lines are a **warning, not an error** (expected after git concat-merges; `list` folds them first-wins). Same-ID lines with **different payloads** (or an ID that fails recomputation) are an `id_conflict` finding — corruption, not a benign duplicate.
- If the `git` binary is available and the log lives in a repo, warns when the log path is gitignored (the diff-visibility feature silently off).
- **No `--fix` in v1** (review finding: an unguarded quarantine that eats a mis-judged line is worse than no fix; a safe fix path needs backup/undo/dry-run — v2). Exit dictionary: 0 healthy / 1 findings, published in `schema`.

### Envelope and exit codes

Success: `{"ok":true,"data":{…},"meta":{…}}` on stdout, single line (or pretty with `--pretty`).
Error: `{"ok":false,"error":{"code":"…","message":"…","details":{…},"retryable":bool,"suggested_fix":"paste-ready command"},"meta":{"contract":1}}` on **stderr** — the `meta` block (with `contract`) rides on error envelopes too.

Clap integration: `try_parse`; parse failures are rewritten into the error envelope (code `invalid_argument`, exit 2, carrying clap's did-you-mean hint when present). The two documented plaintext exceptions: explicit `--help` and `--version` print clap's human text on stdout, exit 0.

Exit codes follow the rust-agent-cli skill dictionary: 0 success/empty, 2 usage, 65 bad input data, 66 missing file / not-found ID, 70 internal, 75 lock timeout (`retryable:true`), 77 permission denied, 78 config — plus **74 (I/O error) as a documented extension** to the skill table (deliberate deviation, published in `schema`; implementer must not "fix" this back). `std::io::ErrorKind::PermissionDenied` maps to 77/`permission_denied`; other I/O failures to 74/`io_error`. Doctor uses its own published dictionary (0 healthy / 1 findings).

Every envelope (success and error) carries `meta.contract: 1` so consumers can detect contract skew. `schema` output includes an env-var inventory (`PAPERCUTS_FILE`, `PAPERCUTS_AGENT`, `PAPERCUTS_NOW`) and per-command `read_only`/`appends`/`destructive` annotations.

### Record shapes (contract v1)

Cut event:

```json
{"kind":"cut","id":"pc_a1b2c3d4e5f6","ts":"2026-07-09T18:30:00.123Z","agent":"claude-code","text":"rg failed: unquoted zsh glob expanded before rg ran; quote globs or use --files","tags":["shell","rg"],"severity":"minor","cwd":"/Users/x/proj/apps/web","repo":"/Users/x/proj"}
```

Resolve event:

```json
{"kind":"resolve","id":"pc_a1b2c3d4e5f6","ts":"2026-07-10T09:00:00.000Z","agent":"trey","note":"added rg wrapper to CLAUDE.md"}
```

- `id` = `pc_` + first 12 lowercase hex of SHA-256 over the **length-prefixed** field sequence `len(ts) ts len(agent) agent len(text) text len(severity) severity len(tags.join(","))  tags.join(",")` (each len a u32-LE of the UTF-8 byte count; tags sorted) — content-addressed and unambiguous (no delimiter injection), covering every user-supplied field so two same-instant records differing only in severity/tags get distinct IDs.

### Materialized output shapes (normative)

`add` data: `{"changed":bool,"record":{cut fields}}`. `resolve` with one ID returns `{"changed":bool,"record":{cut plus resolution}}`; with two or more IDs it returns `{"changed":bool,"records":[cut plus resolution...]}`.
`list` data: `{"items":[ListItem…],"count":N,"total":M,"truncated":bool}` where `ListItem` = all cut fields + `"status":"open"|"resolved"` + `"resolution":{"ts","agent","note"}` (present only when resolved; `note` null when absent).
`doctor` data: `{"healthy":bool,"findings":[{"line":N,"kind":"torn_line|malformed|unknown_kind|orphan_resolve|duplicate_cut|id_conflict|conflict_marker|gitignored","message":"…"}],"checked_lines":N}`.
`schema` data: the contract object (version, commands with `read_only`/`appends`/`destructive` flags, env vars, error codes, exit codes, record + ListItem shapes). Representative instances of every shape are pinned by deserialization tests.
- `ts` = UTC RFC3339 milliseconds. `PAPERCUTS_NOW` env (RFC3339) overrides the clock for reproducible tests — documented, not hidden.
- Unknown `kind` values are skipped by `list` with a `meta.warnings` count (forward compatibility) but flagged by `doctor`.

## Storage

**Append-only JSONL, event-sourced.** Per the state-and-persistence reference: the check-then-act each mutation needs is serialized by the exclusive file lock (see Concurrency), so JSONL beats SQLite here. `resolve` is an appended event, not a rewrite; **nothing rewrites the file in v1** (the only in-place bytes ever added are appends, including the tear-healing `\n`). `list` folds cut+resolve events into current state at read time — trivial at the scale of a papercuts log (thousands of lines, single-digit ms).

File discovery order:

1. `--file PATH` flag
2. `PAPERCUTS_FILE` env
3. Walk up from cwd to the git repo root; use `<repo-root>/.papercuts.jsonl` (created on first `add`)
4. No repo → `~/.papercuts/log.jsonl`

The per-repo default is the point: the log travels with the repo, and every `add` shows up in `git diff` — exactly how Steve's screenshot surfaced (the green block IS the diff). Teams see papercuts in review for free. This is deliberately committed-by-default (owner decision, review risk acknowledged); the README documents the opt-out (`echo .papercuts.jsonl >> .gitignore` + `PAPERCUTS_FILE`) and recommends `.papercuts.jsonl merge=union` in `.gitattributes` so branch merges concat instead of conflicting. The fold rules below make concat-merges (including duplicated lines) safe.

Repo-root detection treats `.git` as a root marker whether it is a **directory or a file** (worktrees and submodules use a `.git` file).

Concurrency (r3-hardened): mutations open read+append, acquire an exclusive `std::fs::File` lock via **bounded `try_lock` retries** (50 × 100ms; exhaustion → `lock_timeout`, exit 75, `retryable:true`), and run the whole read → fold → decide → append sequence inside that one critical section. The append serializes the full line to one buffer and lands it with `write_all`; on a mid-write error the file is truncated back to its pre-append length (we hold the lock and captured the length). If the file is nonempty and its last byte is not `\n`, the writer first appends a lone `\n` — terminating a previously torn fragment so it becomes one skippable malformed line and the new record stays intact (self-healing, never wedged). Reads take a shared lock with the same bounded retries. Durability is best-effort (no fsync per append — documented; a papercut lost to a power cut is acceptable). Advisory locks are only claimed for **local filesystems**; network mounts (NFS/SMB) are documented as unsupported. First `add` creates the file (and `~/.papercuts/` when at the home fallback) race-safely via `create_dir_all` + open `create|read|append`. Empty `PAPERCUTS_FILE`/`PAPERCUTS_AGENT` env values are treated as unset; an unresolvable home directory is a config error (78).

### `list` fold algorithm (normative)

1. Read lines in file order. A final line without a trailing `\n` is **torn**: skip it, count it in `meta.warnings`, never fail the whole read.
2. Lines that fail to parse, or parse to an unknown `kind`, are skipped and counted in `meta.warnings` (forward compatibility; `doctor` reports them with line numbers).
3. `cut` events: **first occurrence of an ID wins**; later duplicates are ignored (this is what makes git concat-merges and idempotent-add races self-healing). Evidence is excluded from the ID, so duplicate-ID adds keep the first cut and do not store later evidence.
4. `resolve` events: mark the ID resolved, recording the **first** resolve's `ts`/`agent`/`note`. A resolve whose ID has not been seen *by end of file* is an **orphan**: counted in `meta.warnings`, otherwise ignored (a resolve line may legitimately precede its cut line after a merge, so resolution status is computed after the full scan).
5. Sort for output: severity rank (blocker > major > minor), then `ts` descending, then `id` ascending; tags sorted within each record. Same ordering for every format — `md` output is deterministic.

`--since` semantics: relative durations (`Nd`/`Nh`) are computed against the effective now (`PAPERCUTS_NOW` if set, else wall clock UTC). Absolute values must be full RFC3339 with offset (`Z` accepted); date-only input is rejected with a `suggested_fix` showing both forms (ambiguous timezone — reject, don't guess).

## Dependencies (each justified)

- `clap` 4 (derive) — parser, per skill.
- `serde` + `serde_json` — every output shape is a struct.
- `thiserror` — typed public error contract.
- `jiff` — RFC3339 UTC timestamps, parsing `--since`. (Frozen choice — implementer must not substitute.)
- `sha2` — content-addressed IDs.
- Dev: `assert_cmd`, `predicates`, `tempfile`.

Nothing else. No tokio, no color crates, no config-file crate, no git library.

## Testing strategy

- Parser unit tests via `Cli::try_parse_from` (conflicts, defaults, bad values).
- Black-box CLI tests via `assert_cmd`: every command's success shape deserialized into its envelope struct; every error path asserts code + exit code + that the `suggested_fix` hint survives (pinned per the error-rewriting craft).
- **Table-driven fold matrix**: adversarial event orderings — resolve-before-cut, orphan resolve, duplicate cuts (identical and id-conflicting), duplicate resolves, torn tail, unknown kinds, interleavings of all of the above — each row asserting folded state + warning counts.
- Concurrency tests: (a) N threads × M distinct `add`s against one file → exactly N×M valid lines; (b) racing **identical** adds → exactly one line, one `changed:true`; (c) racing resolves of one cut → one `changed:true`, one already-resolved.
- Torn-tail self-heal test: truncated final line, then `add` → fragment terminated, new record intact, `list` shows it with one malformed-line warning.
- Discovery precedence tests: `--file` beats env beats walk-up beats home; explicit-missing = 66 vs discovered-missing = virtual empty; `.git`-as-file root.
- Determinism test: two identical invocations with `PAPERCUTS_NOW` fixed against **identical fresh state** produce byte-identical stdout; the fixed-clock retry case is asserted separately (`changed:true` then `changed:false`).
- Quality gate: `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test`, `cargo build --release`. 5x test sweep before any commit.
- Live acceptance (coordinator-driven): drive the real binary through the full agent lifecycle including empty states, malformed file, ambiguous prefixes, concurrent adds, stdin path.

## Distribution / ship plan

- Public GitHub repo `treygoff24/papercuts`, README written for two audiences: the human installing it, and the agent using it (an AGENTS.md-ready snippet to paste into any repo's agent instructions).
- `cargo install papercuts` as the v0.1.0 install path; `cargo publish` at ship.
- cargo-dist/homebrew/curl-installer deferred to a follow-up release (lens playbook exists; not v0.1 scope).

## Non-goals (v1)

- No server, sync, or telemetry — the file is the product.
- No TUI, no interactive anything.
- No dedup/clustering/AI summarization of cuts (the reviewing agent can do that; this tool is the substrate).
- No Windows CI (nothing platform-specific in the design; just untested).
- No `edit`/`delete` of history — append-only is a feature; nothing rewrites the file in v1 (`doctor --fix` deferred to v2 with backup/undo/dry-run).
- No config file.
- No `--correlation-id` (single-shot local CLI with no logs to correlate — echo-only ceremony; revisit if a long-running mode ever exists).

## Amendments (r2, from adversarial review 2026-07-09)

Reviewers: Cursor (Grok 4.5) `safe`, delivered; Codex GPT-5.6 Sol xhigh attempted twice (work-account quota exhaustion, then expired personal token) — re-run post re-auth or substituted per lane availability. Triage of all Cursor findings:

**Accepted (folded into the doc above):** torn-last-line handling (single-write append + skip-with-warning on read); idempotent `add` resolving the duplicate-ID/determinism contradiction; normative fold algorithm (first-cut-wins, orphan resolves, post-scan status); `.git`-file root detection (worktrees); exit-74-as-documented-extension; `meta.contract` version on every envelope; `--dry-run` + `changed:bool` on mutations; doctor demoted to diagnose-only (cut `--fix`); doctor gitignore check; `--since` semantics pinned; deterministic md sort; jiff frozen; local-fs-only locking note; best-effort durability note; `merge=union` README guidance.

**Accepted-reduced:** NFS handling = documentation only, no runtime network-fs detection (unreliable heuristics); prefix-resolve stays ≥4 chars but all emitted examples use full IDs.

**Rejected with reasons:** `--correlation-id` (see non-goals); `meta.ignored_by_git` on every `add`/`list` (spawning `git check-ignore` per invocation buys little — doctor covers it); runtime `tempfile` dep (moot — no rewrite path in v1); Windows lock-behavior work (stays a documented non-goal).

## Amendments (r3, from Codex GPT-5.6 Sol xhigh review 2026-07-09)

Second decorrelated review: 1 blocker, 12 major, 1 minor. Triage:

**Accepted (folded in):** F2 mutations lock-then-fold-then-append in one critical section + race tests; F3 `write_all` + truncate-on-error + tear-healing `\n` + 10KB text bound; F4 bounded `try_lock` (50×100ms) → exit 75 retryable; F5 purged all `doctor --fix` ghosts; F6 `add --dry-run`; F7 cut `--format jsonl` and `--quiet`, md documented as sole raw-output exception; F8 normative materialized shapes (ListItem, doctor findings, mutation data) + filter/limit semantics; F9 `meta.contract` on error envelopes, clap `try_parse` rewriting + help/version exceptions, PermissionDenied → 77; F10 severity-first-then-newest is the one normative sort (synopsis fixed, limit-slice pinned); F11 normative prefix candidate set (folded distinct cuts, ≥4 hex after optional `pc_`, case-insensitive); F12 virtual-empty for discovered-missing vs 66 for explicit-missing, race-safe creation, env-empty=unset, no-home=78; F13 conflict markers as physical lines only, `id_conflict` for same-ID-different-payload, doctor recomputes IDs; F14 fold matrix + the concurrency/discovery/format test additions.

**Accepted-reduced:** F1 (the blocker) — the retry-idempotence *claim* was wrong and is retracted; the fix is honest reframing (duplicate-safe content addressing for determinism + merge self-healing), a length-prefixed all-field hash (kills delimiter injection and the fixed-clock severity-collapse), and NOT a caller idempotency key (explicit v1 non-goal — complaint logging does not need exactly-once, and a key registry contradicts append-only simplicity). F3's "refuse appends on missing final newline" reduced to tear-healing (a wedged log with no rewrite path would be strictly worse).

**Rejected:** none — every finding drew blood. Round 2 of plan review closes here (two decorrelated rounds, findings converged from design contradictions to contract precision; residual risk moves to the code-review wave).

## Amendments (r4, Wave 2 implementation 2026-07-15)

Wave 2 adds optional cut evidence without changing the v1 identity or fold rules. A cut may carry `evidence` with optional `cmd`, integer `exit`, `stderr`, and `note` fields; absent fields are omitted during serialization, and stderr is read and redacted in full before its sanitized value is capped at 4096 valid UTF-8 bytes. To keep memory bounded, `--stderr-file` rejects regular files over 1 MiB rather than raw-truncating them before redaction. `metadata()` follows symlinks: a symlink to a regular file is accepted, while a FIFO, device, directory, or a symlink resolving to one is rejected before opening. Evidence strings pass a deterministic best-effort redactor for assignment/header forms involving key, token, secret, password, authorization, and bearer, plus long high-entropy token shapes; this does not make raw environment dumps safe to submit.

Evidence is not included in the content-addressed ID. Duplicate cut events remain first-cut-wins, and duplicate-ID `add` returns the first record with a `duplicate_cut` warning stating that later evidence was not stored. Resolve events remain first-resolve-wins. Multi-ID `resolve` validates every argument and prefix under one exclusive-lock critical section before appending; one ID retains `{changed,record}`, while two or more IDs return `{changed,records:[...]}` in canonical ID order. Validation failures append nothing.

## Wave plan

Slimmed foundry (reduced config: Codex authors and fixes, cross-family review via Cursor/Grok, coordinator independently gates and reads riskiest files).

- **Plan review** (this doc): `delegate codex safe --model sol --reasoning-effort xhigh` + `delegate cursor safe` in parallel; coordinator triages all findings in writing; doc amended.
- **Wave 1 — the whole CLI, one lane** (task-clustering: ~1000 LOC sharing one design; splitting would fragment coherence): `delegate codex work --model sol --reasoning-effort high`. Layout per skill: `main.rs`/`cli.rs`/`commands/`/`output.rs`/`error.rs`/`lib.rs`/`tests/`.
- **Review wave**: `delegate cursor safe` adversarial review of the diff + coordinator riskiest-file read (locking/append path, ID fold logic in `list`, torn-line handling). Triage → Codex fix round → coordinator verifies every fix landed → re-review until dry (3-round cap).
- **Acceptance**: coordinator drives the real binary. Zero unexplained failures.
- **Ship**: README/AGENTS.md, GitHub repo + push, tag v0.1.0, `cargo publish`.

Budget: subscription lanes only (Codex + Cursor); zero metered spend expected.
