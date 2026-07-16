#!/bin/sh
set -eu

ROOT=$(cd -- "$(dirname -- "$0")/.." && pwd)
REPORT=${REPORT:-"$ROOT/docs/papercuts-diagnostic-report-2026-07-15.md"}
MANIFEST=${MANIFEST:-"$ROOT/docs/plans/papercuts-remediation-manifest.md"}
AFTER_WAVE=0
DIAGNOSTIC_ONLY=0
LOGS=
ATTESTATIONS=
HARNESS_OUTCOMES=

usage() {
    cat <<'EOF'
Usage: scripts/check-manifest.sh --after-wave WAVE --log PATH [--log PATH ...] [conditions]
       scripts/check-manifest.sh --diagnostic-only [--log PATH ...]

WAVE: 0, 1, 2, 3, 4a, 5, 4b, 6, 7, 8.
Conditions: --accept-harness claude|codex|delegate,
            --defer-harness claude|codex|delegate, --verify-id ID,
            --verify-opm-complete-part-set, --task-x-key, --task-data-gov-key.

State gates require all 132 frozen IDs. --diagnostic-only accepts partial
coverage and never prints PASS. Wave 4a has no cut-status delta; run its
separate doctor/framework gate as well.
EOF
}

add_log() {
    LOGS="${LOGS}${LOGS:+
}$1"
}

add_attestation() {
    ATTESTATIONS="${ATTESTATIONS}${ATTESTATIONS:+
}$1"
}

add_harness_outcome() {
    HARNESS_OUTCOMES="${HARNESS_OUTCOMES}${HARNESS_OUTCOMES:+
}$1|$2"
}

wave_rank() {
    case "$1" in
        0) echo 0 ;; 1) echo 1 ;; 2) echo 2 ;; 3) echo 3 ;; 4a) echo 4 ;;
        5) echo 5 ;; 4b) echo 6 ;; 6) echo 7 ;; 7) echo 8 ;; 8) echo 9 ;;
        *) return 1 ;;
    esac
}

while [ "$#" -gt 0 ]; do
    case "$1" in
        --after-wave)
            [ "$#" -ge 2 ] || { usage >&2; exit 64; }
            AFTER_WAVE=$2
            shift 2
            ;;
        --log)
            [ "$#" -ge 2 ] || { usage >&2; exit 64; }
            add_log "$2"
            shift 2
            ;;
        --diagnostic-only) DIAGNOSTIC_ONLY=1; shift ;;
        --condition)
            [ "$#" -ge 2 ] || { usage >&2; exit 64; }
            case "$2" in shell:*)
                printf 'shell conditions require --accept-harness or --defer-harness\n' >&2; exit 64 ;;
            esac
            add_attestation "$2"
            shift 2
            ;;
        --accept-harness)
            [ "$#" -ge 2 ] || { usage >&2; exit 64; }
            case "$2" in claude|codex|delegate) add_attestation "shell:$2" ;; *)
                printf 'unknown harness: %s\n' "$2" >&2; exit 64 ;;
            esac
            add_harness_outcome "$2" accept
            shift 2
            ;;
        --defer-harness)
            [ "$#" -ge 2 ] || { usage >&2; exit 64; }
            case "$2" in claude|codex|delegate) ;; *)
                printf 'unknown harness: %s\n' "$2" >&2; exit 64 ;;
            esac
            add_harness_outcome "$2" defer
            shift 2
            ;;
        --verify-id)
            [ "$#" -ge 2 ] || { usage >&2; exit 64; }
            add_attestation "verify:$2"
            shift 2
            ;;
        --verify-opm-complete-part-set)
            add_attestation 'verify:opm-complete-part-set'
            shift
            ;;
        --task-x-key) add_attestation 'task:x-key'; shift ;;
        --task-data-gov-key) add_attestation 'task:data-gov-key'; shift ;;
        -h|--help) usage; exit 0 ;;
        *) printf 'unknown argument: %s\n' "$1" >&2; usage >&2; exit 64 ;;
    esac
done

wave_rank "$AFTER_WAVE" >/dev/null || { printf 'invalid wave: %s\n' "$AFTER_WAVE" >&2; exit 64; }
[ -f "$REPORT" ] || { printf 'missing report: %s\n' "$REPORT" >&2; exit 66; }
[ -f "$MANIFEST" ] || { printf 'missing manifest: %s\n' "$MANIFEST" >&2; exit 66; }
command -v jq >/dev/null 2>&1 || { printf 'jq is required\n' >&2; exit 69; }
command -v trash >/dev/null 2>&1 || { printf 'trash is required before temporary files are created\n' >&2; exit 69; }

if [ -n "${PAPERCUTS_BIN:-}" ]; then
    if [ -x "$PAPERCUTS_BIN" ]; then
        PAPERCUTS_CMD=$PAPERCUTS_BIN
    else
        PAPERCUTS_CMD=$(command -v "$PAPERCUTS_BIN" 2>/dev/null) || {
            printf 'PAPERCUTS_BIN is not executable: %s\n' "$PAPERCUTS_BIN" >&2; exit 69;
        }
    fi
elif command -v papercuts >/dev/null 2>&1; then
    PAPERCUTS_CMD=$(command -v papercuts)
elif [ -x "$ROOT/target/release/papercuts" ]; then
    PAPERCUTS_CMD=$ROOT/target/release/papercuts
elif [ -x "$ROOT/target/debug/papercuts" ]; then
    PAPERCUTS_CMD=$ROOT/target/debug/papercuts
else
    printf 'papercuts is required (set PAPERCUTS_BIN, install it, or build target/release/papercuts)\n' >&2
    exit 69
fi

if [ -z "$LOGS" ] && [ "$DIAGNOSTIC_ONLY" -eq 1 ]; then
    add_log "${PAPERCUTS_FILE:-"$ROOT/.papercuts.jsonl"}"
fi
[ -n "$LOGS" ] || { printf 'a state gate requires one or more --log PATH arguments\n' >&2; usage >&2; exit 64; }

TMPDIR_PATH=$(mktemp -d "${TMPDIR:-/tmp}/papercuts-manifest.XXXXXX")
trap 'trash "$TMPDIR_PATH" >/dev/null 2>&1 || true' EXIT HUP INT TERM
printf '%s\n' "$LOGS" > "$TMPDIR_PATH/logs"
printf '%s\n' "$ATTESTATIONS" | sed '/^$/d' | sort > "$TMPDIR_PATH/attestations"
printf '%s\n' "$HARNESS_OUTCOMES" | sed '/^$/d' | sort > "$TMPDIR_PATH/harness-outcomes"
duplicates=$(uniq -d "$TMPDIR_PATH/attestations")
[ -z "$duplicates" ] || { printf 'duplicate attestations:\n%s\n' "$duplicates" >&2; exit 64; }
duplicates=$(cut -d'|' -f1 "$TMPDIR_PATH/harness-outcomes" | uniq -d)
[ -z "$duplicates" ] || { printf 'duplicate or conflicting harness outcomes:\n%s\n' "$duplicates" >&2; exit 64; }

awk -F'|' '
function trim(s) { gsub(/^[ \t]+|[ \t]+$/, "", s); gsub(/`/, "", s); return s }
/^\| [0-9]+ \| `pc_/ { print trim($3) "|" trim($6) }
' "$REPORT" | sort -t'|' -k1,1 > "$TMPDIR_PATH/report.tsv"

awk -F'|' '
$0 == "## ID manifest" { inside=1; next }
inside && /^\| `pc_[0-9a-f]+` \|/ {
    id=$2; disposition=$3; wave=$4; end_state=$5
    gsub(/^[ \t]+|[ \t]+$/, "", id); gsub(/^[ \t]+|[ \t]+$/, "", disposition)
    gsub(/^[ \t]+|[ \t]+$/, "", wave); gsub(/^[ \t]+|[ \t]+$/, "", end_state)
    gsub(/`/, "", id); gsub(/`/, "", disposition); gsub(/`/, "", wave); gsub(/`/, "", end_state)
    print id "|" disposition "|" wave "|" end_state
}' "$MANIFEST" | sort -t'|' -k1,1 > "$TMPDIR_PATH/manifest.tsv"

awk -F'|' '
$0 == "## Named resolution conditions" { inside=1; next }
inside && /^## / { inside=0 }
inside && /^\| `pc_[0-9a-f]+` \|/ {
    id=$2; condition=$3
    gsub(/^[ \t]+|[ \t]+$/, "", id); gsub(/^[ \t]+|[ \t]+$/, "", condition)
    gsub(/`/, "", id); gsub(/`/, "", condition)
    print id "|" condition
}' "$MANIFEST" | sort -t'|' -k1,1 > "$TMPDIR_PATH/conditions.tsv"

report_rows=$(wc -l < "$TMPDIR_PATH/report.tsv" | tr -d ' ')
manifest_rows=$(wc -l < "$TMPDIR_PATH/manifest.tsv" | tr -d ' ')
[ "$report_rows" -eq 132 ] || { printf 'report rows: expected 132, got %s\n' "$report_rows" >&2; exit 1; }
[ "$manifest_rows" -eq 132 ] || { printf 'manifest rows: expected 132, got %s\n' "$manifest_rows" >&2; exit 1; }

for source in report manifest; do
    duplicates=$(cut -d'|' -f1 "$TMPDIR_PATH/$source.tsv" | uniq -d)
    [ -z "$duplicates" ] || { printf 'duplicate %s IDs:\n%s\n' "$source" "$duplicates" >&2; exit 1; }
done
duplicates=$(cut -d'|' -f1 "$TMPDIR_PATH/conditions.tsv" | uniq -d)
[ -z "$duplicates" ] || { printf 'duplicate condition rows:\n%s\n' "$duplicates" >&2; exit 1; }

cut -d'|' -f1 "$TMPDIR_PATH/report.tsv" > "$TMPDIR_PATH/report.ids"
cut -d'|' -f1 "$TMPDIR_PATH/manifest.tsv" > "$TMPDIR_PATH/manifest.ids"
missing=$(comm -23 "$TMPDIR_PATH/report.ids" "$TMPDIR_PATH/manifest.ids")
extra=$(comm -13 "$TMPDIR_PATH/report.ids" "$TMPDIR_PATH/manifest.ids")
[ -z "$missing" ] || { printf 'diagnostic IDs missing from manifest:\n%s\n' "$missing" >&2; exit 1; }
[ -z "$extra" ] || { printf 'manifest IDs absent from frozen diagnostic:\n%s\n' "$extra" >&2; exit 1; }

awk -F'|' '
NR == FNR { report[$1]=$2; next }
!( $1 in report ) { print "unknown diagnostic ID: " $1; bad=1; next }
report[$1] != $2 { print "disposition mismatch for " $1 ": report=" report[$1] ", manifest=" $2; bad=1 }
END { exit bad }
' "$TMPDIR_PATH/report.tsv" "$TMPDIR_PATH/manifest.tsv" || exit 1

awk -F'|' '
function rank(w) { return w == 1 ? 1 : w == 2 ? 2 : w == 3 ? 3 : w == "4a" ? 4 : w == 5 ? 5 : w == "4b" ? 6 : w == 6 ? 7 : w == 7 ? 8 : w == 8 ? 9 : 0 }
{
    if (rank($3) == 0) { print "invalid wave for " $1 ": " $3; bad=1 }
    if ($4 !~ /^(resolve-on-(1|2|3|4a|4b|5|6|7|8|verification)|stays-open-(needs-repro|external|trey-task)|already-resolved)$/) { print "invalid end state for " $1 ": " $4; bad=1 }
    if ($4 ~ /^resolve-on-/ && $4 != "resolve-on-verification") { split($4, p, "-"); if (p[3] != $3) { print "resolution wave mismatch for " $1; bad=1 } }
    if ($2 == "resolved" && $4 != "already-resolved") { print "resolved disposition must be already-resolved: " $1; bad=1 }
    if ($2 == "needs-repro" && $4 != "stays-open-needs-repro") { print "needs-repro must stay open: " $1; bad=1 }
    if ($2 == "external-upstream" && $4 != "stays-open-external") { print "external-upstream must stay open: " $1; bad=1 }
    if ($2 == "already-fixed" && $4 != "resolve-on-verification") { print "already-fixed requires live verification: " $1; bad=1 }
    if (($2 == "fix" || $2 == "instruction-only") && ($4 == "already-resolved" || $4 ~ /^stays-open-(needs-repro|external)$/)) { print "incompatible disposition/end state for " $1; bad=1 }
}
END { exit bad }
' "$TMPDIR_PATH/manifest.tsv" || exit 1

cat > "$TMPDIR_PATH/expected-assertion-keys" <<'EOF'
total
disposition.already-fixed
disposition.external-upstream
disposition.fix
disposition.instruction-only
disposition.needs-repro
disposition.resolved
wave.1
wave.2
wave.3
wave.4a
wave.5
wave.4b
wave.6
wave.7
wave.8
end_state.already-resolved
end_state.resolve-on-1
end_state.resolve-on-2
end_state.resolve-on-3
end_state.resolve-on-4a
end_state.resolve-on-5
end_state.resolve-on-4b
end_state.resolve-on-6
end_state.resolve-on-7
end_state.resolve-on-8
end_state.resolve-on-verification
end_state.stays-open-external
end_state.stays-open-needs-repro
end_state.stays-open-trey-task
EOF
sort "$TMPDIR_PATH/expected-assertion-keys" -o "$TMPDIR_PATH/expected-assertion-keys"
awk '/^<!-- checker assertions$/{inside=1; next} /^-->$/{inside=0} inside && /^[a-z_.0-9-]+=[0-9]+$/ { print }' "$MANIFEST" | sort > "$TMPDIR_PATH/assertions.tsv"
cut -d= -f1 "$TMPDIR_PATH/assertions.tsv" > "$TMPDIR_PATH/assertion.keys"
duplicates=$(uniq -d "$TMPDIR_PATH/assertion.keys")
[ -z "$duplicates" ] || { printf 'duplicate assertion keys:\n%s\n' "$duplicates" >&2; exit 1; }
diff -u "$TMPDIR_PATH/expected-assertion-keys" "$TMPDIR_PATH/assertion.keys" >/dev/null || { printf 'assertion keys are not the exact required set\n' >&2; exit 1; }

while IFS='=' read -r key expected; do
    case "$key" in
        total) actual=$manifest_rows ;;
        disposition.*) actual=$(awk -F'|' -v v="${key#disposition.}" '$2 == v {n++} END {print n+0}' "$TMPDIR_PATH/manifest.tsv") ;;
        wave.*) actual=$(awk -F'|' -v v="${key#wave.}" '$3 == v {n++} END {print n+0}' "$TMPDIR_PATH/manifest.tsv") ;;
        end_state.*) actual=$(awk -F'|' -v v="${key#end_state.}" '$4 == v {n++} END {print n+0}' "$TMPDIR_PATH/manifest.tsv") ;;
        *) printf 'unknown assertion key: %s\n' "$key" >&2; exit 1 ;;
    esac
    [ "$actual" = "$expected" ] || { printf 'assertion %s: expected %s, got %s\n' "$key" "$expected" "$actual" >&2; exit 1; }
done < "$TMPDIR_PATH/assertions.tsv"

awk -F'|' '
function trim(s) { gsub(/^[ \t]+|[ \t]+$/, "", s); gsub(/`|\*\*/, "", s); return s }
/^## / { section=""; next }
/^\| Disposition \| Count \|/ { section="disposition"; next }
/^\| Wave \| Count \|/ { section="wave"; next }
/^\| Expected end state \| Count \|/ { section="end_state"; next }
section != "" && /^\| `/ { print section "." trim($2) "=" trim($3) }
' "$MANIFEST" | sort > "$TMPDIR_PATH/visible-counts.tsv"
grep -v '^total=' "$TMPDIR_PATH/assertions.tsv" > "$TMPDIR_PATH/assertion-counts.tsv"
diff -u "$TMPDIR_PATH/assertion-counts.tsv" "$TMPDIR_PATH/visible-counts.tsv" >/dev/null || { printf 'visible count tables do not match assertions\n' >&2; exit 1; }

awk -F'|' '
NR == FNR { disposition[$1]=$2; wave[$1]=$3; end_state[$1]=$4; next }
{
    id=$1; condition=$2
    if (!(id in disposition)) { print "condition for unknown ID: " id; bad=1; next }
    if (condition !~ /^(shell:(claude|codex|delegate)|verify:pc_[0-9a-f]+|verify:opm-complete-part-set|task:(x-key|data-gov-key))$/) { print "invalid condition for " id; bad=1 }
    condition_for[id]=condition
}
END {
    for (id in disposition) {
        c=condition_for[id]; e=end_state[id]
        if (e == "resolve-on-verification") {
            if (disposition[id] == "already-fixed" && c != "verify:" id) { print "already-fixed condition mismatch for " id; bad=1 }
            else if (disposition[id] != "already-fixed" && !(wave[id] == 1 && c ~ /^shell:/)) { print "shell verification condition missing for " id; bad=1 }
        } else if (id == "pc_b8fe2e571b1f") {
            if (c != "verify:opm-complete-part-set") { print "OPM condition missing"; bad=1 }
        } else if (id == "pc_02430da9ef6d" || id == "pc_88e09fdfbb7f") {
            if (!(wave[id] == 3 && e == "resolve-on-3" && c ~ /^shell:/)) { print "Wave 1 plus Wave 3 condition missing for " id; bad=1 }
        }
        else if (id == "pc_3d8f55856fe6" || id == "pc_b66efae3997d") { if (!(e == "stays-open-trey-task" && wave[id] == 7 && c == "task:x-key")) { print "X task condition mismatch for " id; bad=1 } }
        else if (id == "pc_828f1dfa2edc") { if (!(e == "stays-open-trey-task" && wave[id] == 7 && c == "task:data-gov-key")) { print "DATA.gov task condition mismatch"; bad=1 } }
        else if (c != "") { print "unexpected condition for " id; bad=1 }
        if (e == "resolve-on-verification" && c == "") { print "missing verification condition for " id; bad=1 }
    }
    exit bad
}
' "$TMPDIR_PATH/manifest.tsv" "$TMPDIR_PATH/conditions.tsv" || exit 1

cut -d'|' -f2 "$TMPDIR_PATH/conditions.tsv" | sort -u > "$TMPDIR_PATH/condition.names"
sed -n 's/^shell://p' "$TMPDIR_PATH/condition.names" | sort -u > "$TMPDIR_PATH/shell-harnesses"
cut -d'|' -f1 "$TMPDIR_PATH/harness-outcomes" > "$TMPDIR_PATH/outcome-harnesses"
unknown_outcomes=$(comm -23 "$TMPDIR_PATH/outcome-harnesses" "$TMPDIR_PATH/shell-harnesses")
[ -z "$unknown_outcomes" ] || { printf 'unknown harness outcomes:\n%s\n' "$unknown_outcomes" >&2; exit 64; }
if [ "$(wave_rank "$AFTER_WAVE")" -ge 1 ]; then
    missing_outcomes=$(comm -23 "$TMPDIR_PATH/shell-harnesses" "$TMPDIR_PATH/outcome-harnesses")
    [ -z "$missing_outcomes" ] || { printf 'missing required harness outcomes:\n%s\n' "$missing_outcomes" >&2; exit 1; }
    claude_outcome=$(awk -F'|' '$1 == "claude" { print $2 }' "$TMPDIR_PATH/harness-outcomes")
    [ "$claude_outcome" = accept ] || { printf 'Claude must be accepted at Wave 1\n' >&2; exit 1; }
fi
while IFS= read -r attestation; do
    grep -Fx "$attestation" "$TMPDIR_PATH/condition.names" >/dev/null || { printf 'unknown attestation: %s\n' "$attestation" >&2; exit 64; }
done < "$TMPDIR_PATH/attestations"
printf '%s\n' '__no-attestation__' >> "$TMPDIR_PATH/attestations"

: > "$TMPDIR_PATH/live-all.tsv"
log_index=0
while IFS= read -r log || [ -n "$log" ]; do
    [ -f "$log" ] || { printf 'missing log: %s\n' "$log" >&2; exit 66; }
    log_index=$((log_index + 1))
    output="$TMPDIR_PATH/live-$log_index.json"
    PAPERCUTS_FILE="$log" "$PAPERCUTS_CMD" list --status all --limit 100000 > "$output"
    envelope_issues=$(jq -r '
        [
            if .ok != true then "ok is not true" else empty end,
            if (.data | type) != "object" then "data is not an object" else empty end,
            if (.data.items? | type) != "array" then "items is not an array" else empty end,
            if (.data.count? | type) != "number" then "count is not numeric" else empty end,
            if (.data.total? | type) != "number" then "total is not numeric" else empty end,
            if (.data.truncated? | type) != "boolean" then "truncated is not boolean" else empty end,
            if ((.data.items? | type) == "array" and (.data.count? | type) == "number" and .data.count != (.data.items | length)) then "count does not match items" else empty end,
            if ((.data.count? | type) == "number" and (.data.total? | type) == "number" and .data.count != .data.total) then "count/total inconsistency" else empty end,
            if .data.truncated? == true then "truncated list" else empty end,
            if (.meta | type) != "object" then "meta is not an object" else empty end,
            if ((.meta.warnings? // []) | type) != "array" then "warnings is not an array" else empty end,
            if ((.meta.warnings? // []) | type) == "array" and ((.meta.warnings? // []) | length) > 0 then "warnings: " + ((.meta.warnings? // []) | map(tostring) | join(", ")) else empty end,
            if ((.data.items? | type) == "array") and any(.data.items[]; (type != "object") or ((.id | type) != "string") or ((.id | test("^pc_[0-9a-f]{12}$")) | not) or (.status != "open" and .status != "resolved")) then "malformed item ID or status" else empty end
        ] | join("; ")
    ' "$output") || { printf 'papercuts list returned non-JSON output for %s\n' "$log" >&2; exit 1; }
    if [ -n "$envelope_issues" ]; then
        if [ "$DIAGNOSTIC_ONLY" -eq 1 ]; then
            printf 'log diagnostic for %s: %s\n' "$log" "$envelope_issues"
        else
            printf 'papercuts list returned unsafe envelope for %s: %s\n' "$log" "$envelope_issues" >&2
            exit 1
        fi
    fi
    jq -r 'if (.data.items? | type) == "array" then .data.items[] | select(type == "object" and (.id | type) == "string" and (.status == "open" or .status == "resolved")) | [.id, .status] | @tsv else empty end' "$output" > "$TMPDIR_PATH/live-$log_index.tsv"
    duplicates=$(cut -f1 "$TMPDIR_PATH/live-$log_index.tsv" | sort | uniq -d)
    [ -z "$duplicates" ] || { printf 'duplicate live IDs in %s:\n%s\n' "$log" "$duplicates" >&2; exit 1; }
    cat "$TMPDIR_PATH/live-$log_index.tsv" >> "$TMPDIR_PATH/live-all.tsv"
done < "$TMPDIR_PATH/logs"

sort -t "$(printf '\t')" -k1,1 -k2,2 "$TMPDIR_PATH/live-all.tsv" > "$TMPDIR_PATH/live-sorted.tsv"
awk -F'\t' '
{
    if (!($1 in status) || $2 == "resolved") status[$1]=$2
}
END { for (id in status) print id "\t" status[id] }
' "$TMPDIR_PATH/live-sorted.tsv" | sort -t "$(printf '\t')" -k1,1 > "$TMPDIR_PATH/live.tsv"
cut -f1 "$TMPDIR_PATH/live.tsv" | sort > "$TMPDIR_PATH/live.ids"

# Ledger-lost IDs (manifest "Amendments 2026-07-16"): these four lived in
# .papercuts.jsonl files inside delegate worktrees that were deleted after the
# diagnostic snapshot. Their filings survive verbatim in the diagnostic report;
# they are counted here at their last attested status instead of from a live
# ledger. Loudly disclosed on every run. Do not add IDs without a matching
# manifest amendment entry.
while IFS=' ' read -r lost_id lost_status; do
    [ -n "$lost_id" ] || continue
    if ! grep -q "^$lost_id	" "$TMPDIR_PATH/live.tsv"; then
        printf '%s\t%s\n' "$lost_id" "$lost_status" >> "$TMPDIR_PATH/live.tsv"
        printf 'ledger-lost ID counted at attested status (%s): %s\n' "$lost_status" "$lost_id"
    fi
done <<'LOST'
pc_944d374ac9c4 resolved
pc_8c2350511589 open
pc_df6af25a100a open
pc_f8eb38d950f5 open
LOST
sort -t "$(printf '\t')" -k1,1 -o "$TMPDIR_PATH/live.tsv" "$TMPDIR_PATH/live.tsv"
cut -f1 "$TMPDIR_PATH/live.tsv" | sort > "$TMPDIR_PATH/live.ids"

post_snapshot=$(comm -13 "$TMPDIR_PATH/report.ids" "$TMPDIR_PATH/live.ids")
[ -z "$post_snapshot" ] || printf 'post-snapshot live-log IDs ignored: %s\n' "$(printf '%s' "$post_snapshot" | tr '\n' ' ')"
live_snapshot=$(comm -12 "$TMPDIR_PATH/report.ids" "$TMPDIR_PATH/live.ids")
live_count=$(printf '%s\n' "$live_snapshot" | sed '/^$/d' | wc -l | tr -d ' ')
awk -F'\t' 'NR == FNR { known[$1]=1; next } $1 in known { print }' "$TMPDIR_PATH/manifest.ids" "$TMPDIR_PATH/live.tsv" > "$TMPDIR_PATH/live-snapshot.tsv"
open_count=$(awk -F'\t' '$2 == "open" {n++} END {print n+0}' "$TMPDIR_PATH/live-snapshot.tsv")
resolved_count=$(awk -F'\t' '$2 == "resolved" {n++} END {print n+0}' "$TMPDIR_PATH/live-snapshot.tsv")

if [ "$DIAGNOSTIC_ONLY" -eq 1 ]; then
    printf 'manifest DIAGNOSTIC: 132 unique diagnostic IDs; live snapshot coverage=%s/132; observed open=%s resolved=%s\n' "$live_count" "$open_count" "$resolved_count"
    exit 0
fi
[ "$live_count" -eq 132 ] || { printf 'incomplete snapshot coverage: got %s/132; use --diagnostic-only for partial inspection\n' "$live_count" >&2; exit 1; }

target_rank=$(wave_rank "$AFTER_WAVE")
awk -F'|' 'NR == FNR { condition[$1]=$2; next } { print $0 "|" (condition[$1] == "" ? "-" : condition[$1]) }' \
    "$TMPDIR_PATH/conditions.tsv" "$TMPDIR_PATH/manifest.tsv" > "$TMPDIR_PATH/state-model.tsv"
tr '\t' '|' < "$TMPDIR_PATH/live-snapshot.tsv" > "$TMPDIR_PATH/live-snapshot.psv"
awk -F'|' 'NR == FNR { actual[$1]=$2; next } { print $0 "|" actual[$1] }' \
    "$TMPDIR_PATH/live-snapshot.psv" "$TMPDIR_PATH/state-model.tsv" > "$TMPDIR_PATH/state-input.tsv"
awk -F'|' -v target="$target_rank" '
FILENAME == ARGV[1] { attested[$1]=1; next }
FILENAME == ARGV[2] { outcome[$1]=$2; next }
{
    id=$1; actual=$6; e=$4; due=0; conditional=0
    if (e == "already-resolved") { expected="resolved" }
    else if (e == "stays-open-needs-repro" || e == "stays-open-external") { expected="open" }
    else {
        due=(target >= rank($3))
        conditional=($5 != "-")
        if (!due) expected="either"
        else if (!conditional) expected="resolved"
        else if ($5 ~ /^shell:/) {
            harness=$5; sub(/^shell:/, "", harness)
            if (outcome[harness] == "accept") expected="resolved"
            else if (outcome[harness] == "defer") expected="open"
            else expected="either"
        } else if (($5) in attested) expected="resolved"
        else expected="either"
    }
    if (expected == "resolved") { required_resolved++ }
    if (expected == "either") { optional_resolved++ }
    if ((expected == "resolved" && actual != "resolved") || (expected == "open" && actual != "open")) { print "state mismatch for " id ": expected=" expected ", actual=" actual; bad=1 }
    if (actual == "resolved") observed_resolved++; else observed_open++
}
END {
    min_resolved=required_resolved+0; max_resolved=min_resolved+optional_resolved
    min_open=132-max_resolved; max_open=132-min_resolved
    if (observed_resolved < min_resolved || observed_resolved > max_resolved || observed_open < min_open || observed_open > max_open) { print "observed totals outside expected bounds"; bad=1 }
    printf "expected open=%s..%s resolved=%s..%s; observed open=%s resolved=%s\n", min_open, max_open, min_resolved, max_resolved, observed_open+0, observed_resolved+0
    exit bad
}
function rank(w) { return w == 1 ? 1 : w == 2 ? 2 : w == 3 ? 3 : w == "4a" ? 4 : w == 5 ? 5 : w == "4b" ? 6 : w == 6 ? 7 : w == 7 ? 8 : w == 8 ? 9 : 0 }
' "$TMPDIR_PATH/attestations" "$TMPDIR_PATH/harness-outcomes" "$TMPDIR_PATH/state-input.tsv" > "$TMPDIR_PATH/state-summary" || { cat "$TMPDIR_PATH/state-summary" >&2; exit 1; }
cat "$TMPDIR_PATH/state-summary"
if [ "$target_rank" -eq 9 ]; then
    awk -F'|' '
    FILENAME == ARGV[1] { actual[$1]=$2; next }
    FILENAME == ARGV[2] { condition[$1]=$2; next }
    FILENAME == ARGV[3] { outcome[$1]=$2; next }
    END {
        for (id in condition) {
            c=condition[id]
            if (c ~ /^shell:/) {
                harness=c; sub(/^shell:/, "", harness)
                if (outcome[harness] != "accept") print c " (" outcome[harness] ")"
            } else if (actual[id] != "resolved") print c
        }
    }
    ' "$TMPDIR_PATH/live-snapshot.psv" "$TMPDIR_PATH/conditions.tsv" "$TMPDIR_PATH/harness-outcomes" | sort -u > "$TMPDIR_PATH/final-pending"
    if [ -s "$TMPDIR_PATH/final-pending" ]; then
        printf 'final conditions remaining:\n'
        sed 's/^/  /' "$TMPDIR_PATH/final-pending"
    else
        printf 'final conditions remaining: none; exact open=21 requires the due-wave state above\n'
    fi
fi
printf 'manifest PASS: 132 unique diagnostic IDs; live snapshot coverage=132/132; after-wave=%s\n' "$AFTER_WAVE"
