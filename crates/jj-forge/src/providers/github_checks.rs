use super::checks::{check_state, timestamp, web_url};
use crate::{
    client::{encode, number, text},
    *,
};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

fn workflow(v: &Value) -> String {
    let id = number(v, "workflow_id");
    if id == 0 {
        format!("run:{}", number(v, "id"))
    } else {
        format!("workflow:{id}")
    }
}
fn rank(v: &Value) -> (u64, u64, u64) {
    (
        number(v, "run_number"),
        number(v, "run_attempt"),
        number(v, "id"),
    )
}
fn relevant(v: &Value, pr: &PullRequest) -> bool {
    v.get("pull_requests")
        .and_then(Value::as_array)
        .is_none_or(|prs| prs.is_empty() || prs.iter().any(|p| number(p, "number") == pr.number))
}
fn display_name(check: &Value, run: Option<&Value>) -> String {
    let name = text(check, "name");
    let Some(run) = run else {
        return name;
    };
    let workflow = text(run, "name");
    let event = text(run, "event");
    let label = if workflow.is_empty() || workflow == name {
        name
    } else {
        format!("{workflow} / {name}")
    };
    if event.is_empty() {
        label
    } else {
        format!("{label} ({event})")
    }
}

/// Present current check contexts, not workflow aggregates plus every historical job.
fn current_checks(
    pr: &PullRequest,
    runs: &[Value],
    leaves: &[Value],
    statuses: &[Value],
) -> Checks {
    let runs: Vec<_> = runs
        .iter()
        .filter(|v| text(v, "head_sha") == pr.sha && relevant(v, pr))
        .collect();
    let mut latest = BTreeMap::<String, &Value>::new();
    for run in &runs {
        let entry = latest.entry(workflow(run)).or_insert(run);
        if rank(run) > rank(entry) {
            *entry = run;
        }
    }
    let mut contexts = BTreeMap::<(u64, String, String), (&Value, Option<&Value>)>::new();
    for leaf in leaves {
        if text(leaf, "head_sha") != pr.sha || !relevant(leaf, pr) {
            continue;
        }
        let suite = number(&leaf["check_suite"], "id");
        let run = runs
            .iter()
            .copied()
            .find(|r| suite != 0 && number(r, "check_suite_id") == suite);
        if let Some(run) = run {
            if latest
                .get(&workflow(run))
                .is_some_and(|current| number(current, "id") != number(run, "id"))
            {
                continue;
            }
        }
        let key = (
            number(&leaf["app"], "id"),
            run.map(workflow).unwrap_or_default(),
            text(leaf, "name"),
        );
        let entry = contexts.entry(key).or_insert((leaf, run));
        if (text(leaf, "started_at"), number(leaf, "id"))
            > (text(entry.0, "started_at"), number(entry.0, "id"))
        {
            *entry = (leaf, run);
        }
    }
    let mut represented = BTreeSet::new();
    let mut checks = vec![];
    for (_, (v, run)) in contexts {
        if let Some(run) = run {
            represented.insert(number(run, "id"));
        }
        checks.push(Check {
            id: format!("check:{}", number(v, "id")),
            name: display_name(v, run),
            state: check_state(&text(v, "status"), &text(v, "conclusion")),
            sha: pr.sha.clone(),
            url: web_url(text(v, "details_url")),
            run_id: run.map(|r| number(r, "id").to_string()),
            log_id: None,
            started_at: timestamp(v, "started_at"),
            completed_at: timestamp(v, "completed_at"),
        });
    }
    // A queued workflow may not have reported any check jobs yet. Keep that fact visible.
    for run in latest.values() {
        let id = number(run, "id");
        if represented.contains(&id) {
            continue;
        }
        checks.push(Check {
            id: id.to_string(),
            name: display_name(run, Some(run)),
            state: check_state(&text(run, "status"), &text(run, "conclusion")),
            sha: pr.sha.clone(),
            url: web_url(text(run, "html_url")),
            run_id: Some(id.to_string()),
            log_id: None,
            started_at: timestamp(run, "run_started_at"),
            completed_at: None,
        });
    }
    let mut names = BTreeSet::new();
    for v in statuses {
        let name = text(v, "context");
        if !names.insert(name.clone()) {
            continue;
        }
        checks.push(Check {
            id: format!("status:{}", number(v, "id")),
            name,
            state: check_state(&text(v, "state"), ""),
            sha: pr.sha.clone(),
            url: web_url(text(v, "target_url")),
            run_id: None,
            log_id: None,
            started_at: None,
            completed_at: None,
        });
    }
    Checks {
        sha: pr.sha.clone(),
        complete: true,
        checks,
    }
}
impl Client {
    pub(super) async fn github_checks(&self, pr: &PullRequest) -> Result<Checks> {
        let project = self.project();
        let sha = encode(&pr.sha);
        let runs = self
            .pages(&format!("{project}/actions/runs?head_sha={sha}"))
            .await?;
        let leaves = self
            .pages(&format!("{project}/commits/{sha}/check-runs?filter=latest"))
            .await?;
        let statuses = self
            .pages(&format!("{project}/commits/{sha}/statuses"))
            .await?;
        Ok(current_checks(pr, &runs, &leaves, &statuses))
    }
}
