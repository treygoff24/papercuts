# AGENTS.md — papercuts

Machine-facing contract for agents working in this repo.

## What this is

`papercuts` is a Rust CLI (clap 4 derive) that lets AI agents log friction into an append-only JSONL file. Agent-only tool: JSON envelopes on stdout, structured errors on stderr, stable exit codes. The normative contract is `docs/plans/2026-07-09-papercuts-design.md` (r3) — treat it as law; its Amendments sections record review provenance and deliberate deviations from the rust-agent-cli skill (exit 74 extension, diagnose-only doctor, no --quiet).

## Build and gate

```bash
cargo build --release
cargo test --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
```

All four must pass before any commit. Run the test suite 5x when touching store.rs or anything concurrency-adjacent — a single green run proves nothing about races.

For focused Rust tests, `cargo test` accepts one positional filter: use one shared substring or separate invocations. Report-generation transforms must pass verbatim replacement text to `re.sub`/`re.subn` through a callable such as `lambda _: replacement`; use `seen.update(tokens)` or `seen |= tokens` for a Python set union, never `set.add(other_set)`.

## Layout

- `src/store.rs` — file discovery, locking (bounded try_lock → exit 75), append (write_all + tear-heal + rollback), the normative fold. The riskiest file; change with care and tests.
- `src/commands/*.rs` — one file per subcommand. Mutations run read→fold→decide→append inside one exclusive-lock critical section.
- `src/error.rs` — the public error contract (codes ↔ exit codes). Never add an undocumented code.
- `src/output.rs` — envelope types. Every output shape is a serde struct.
- `tests/cli.rs` — black-box assert_cmd tests. Env via `Command::env` only, never `std::env::set_var` (parallel-test races).

## Invariants (do not break)

- Append-only: nothing rewrites the log file, ever. The only bytes added are appends (including the tear-healing `\n`).
- stdout = data only, one envelope; stderr = errors only. `--format md` is the sole raw-output exception.
- Deterministic: same input + `PAPERCUTS_NOW` → byte-identical output.
- Empty results are exit 0. Not-found IDs are 66. Lock timeout is 75 + `retryable:true`.
- Dogfood: when you hit friction working here, `cargo run -- add "..."`.
