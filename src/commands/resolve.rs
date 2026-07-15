use crate::cli::ResolveArgs;
use crate::error::{AppError, AppResult};
use crate::output::{self, Meta};
use crate::store;
use crate::{ItemStatus, ListItem, Resolution, ResolveRecord, format_timestamp, resolve_agent};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct ResolveData {
    pub changed: bool,
    pub record: ListItem,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResolveManyData {
    pub changed: bool,
    pub records: Vec<ListItem>,
}

pub fn run(
    args: ResolveArgs,
    file: Option<PathBuf>,
    pretty: bool,
    now: Timestamp,
) -> AppResult<i32> {
    let prefixes: Vec<_> = args
        .ids
        .iter()
        .map(|id| normalize_prefix(id))
        .collect::<AppResult<_>>()?;
    let resolved = store::discover(file)?;
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
    let ts = format_timestamp(now);
    let note = args.note;
    let many = args.ids.len() > 1;
    let action = |log: &mut std::fs::File| -> AppResult<(bool, Vec<String>, Vec<ListItem>)> {
        let bytes = store::read_bytes(log, &resolved.path)?;
        let folded = store::fold_bytes(&bytes);
        let mut ids = prefixes
            .iter()
            .map(|prefix| match_id(prefix, &folded.items))
            .collect::<AppResult<Vec<_>>>()?;
        ids.sort();
        ids.dedup();
        let mut items = ids
            .iter()
            .map(|id| {
                folded
                    .items
                    .iter()
                    .find(|item| item.cut.id == *id)
                    .cloned()
                    .ok_or_else(|| {
                        AppError::internal("matched papercut disappeared during resolution")
                    })
            })
            .collect::<AppResult<Vec<_>>>()?;
        let already_resolved_ids: Vec<_> = ids
            .iter()
            .zip(&items)
            .filter(|(_, item)| item.status == ItemStatus::Resolved)
            .map(|(id, _)| id.clone())
            .collect();
        let mut changed = false;
        if !args.dry_run {
            let mut events = Vec::new();
            for (id, item) in ids.iter().zip(items.iter_mut()) {
                if item.status == ItemStatus::Resolved {
                    continue;
                }
                item.status = ItemStatus::Resolved;
                item.resolution = Some(Resolution {
                    ts: ts.clone(),
                    agent: agent.clone(),
                    note: note.clone(),
                });
                events.push(ResolveRecord {
                    kind: "resolve".into(),
                    id: id.clone(),
                    ts: ts.clone(),
                    agent: agent.clone(),
                    note: note.clone(),
                });
            }
            if !events.is_empty() {
                store::append_json_batch(log, &resolved.path, &bytes, &events)?;
                changed = true;
            }
        } else {
            for item in &mut items {
                if item.status == ItemStatus::Open {
                    item.status = ItemStatus::Resolved;
                    item.resolution = Some(Resolution {
                        ts: ts.clone(),
                        agent: agent.clone(),
                        note: note.clone(),
                    });
                }
            }
        }
        Ok((changed, already_resolved_ids, items))
    };
    let (changed, already_resolved_ids, records) = if args.dry_run {
        store::with_shared(&resolved.path, action)
    } else {
        store::with_exclusive(&resolved.path, false, action)
    }?;
    let mut meta = Meta::new();
    meta.file = Some(resolved.path.to_string_lossy().into_owned());
    meta.agent_source = Some(source.into());
    if already_resolved_ids.len() == records.len() {
        meta.warnings.push("already resolved".into());
    } else if !already_resolved_ids.is_empty() {
        let noun = if already_resolved_ids.len() == 1 {
            "ID"
        } else {
            "IDs"
        };
        meta.warnings.push(format!(
            "already resolved: {} {noun} ({})",
            already_resolved_ids.len(),
            already_resolved_ids.join(", ")
        ));
    } else if args.dry_run {
        meta.warnings
            .push("dry run; no resolve event appended".into());
    }
    if !many {
        output::write_success(
            ResolveData {
                changed,
                record: records.into_iter().next().expect("one record"),
            },
            pretty,
            meta,
        )
        .map_err(|error| AppError::from_io(error, std::path::Path::new("stdout")))?;
    } else {
        output::write_success(ResolveManyData { changed, records }, pretty, meta)
            .map_err(|error| AppError::from_io(error, std::path::Path::new("stdout")))?;
    }
    Ok(0)
}

fn normalize_prefix(input: &str) -> AppResult<String> {
    let prefix = input
        .get(..3)
        .filter(|prefix| prefix.eq_ignore_ascii_case("pc_"))
        .map_or(input, |_| &input[3..]);
    if prefix.len() < 4 || !prefix.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(AppError::invalid_argument(
            format!("invalid papercut ID prefix '{input}'"),
            "Use `papercuts list --status all` and pass at least 4 hexadecimal digits, with optional pc_ prefix.",
        ));
    }
    Ok(prefix.to_ascii_lowercase())
}

fn match_id(prefix: &str, items: &[ListItem]) -> AppResult<String> {
    let mut candidates: Vec<_> = items
        .iter()
        .map(|item| item.cut.id.clone())
        .filter(|id| {
            id.get(..3)
                .filter(|id_prefix| id_prefix.eq_ignore_ascii_case("pc_"))
                .and_then(|_| id.get(3..))
                .is_some_and(|hex| hex.to_ascii_lowercase().starts_with(prefix))
        })
        .collect();
    candidates.sort();
    match candidates.as_slice() {
        [] => Err(AppError::not_found(
            format!("no papercut matches ID prefix '{prefix}'"),
            "Run `papercuts list --status all` and retry with a listed ID.",
        )),
        [id] => Ok(id.clone()),
        _ => Err(AppError::ambiguous_id(prefix, candidates)),
    }
}
