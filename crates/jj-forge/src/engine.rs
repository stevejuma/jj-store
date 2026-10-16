//! Serial external effects with an immutable plan and durable step outcomes.
use crate::journal::Device;
use crate::*;
#[async_trait::async_trait(?Send)]
pub trait Local: Send + Sync {
    async fn authorize(&self) -> Result<()> {
        Ok(())
    }
    async fn validate(&self, plan: &Plan, resuming: bool) -> Result<()>;
    async fn push(&self, segment: &Segment) -> Result<()>;
    async fn remote_head(&self, branch: &str) -> Result<Option<String>>;
}
fn initial(plan: &Plan) -> Receipt {
    let mut steps = vec![];
    for kind in [
        StepKind::Unstack,
        StepKind::Push,
        StepKind::Create,
        StepKind::Retarget,
        StepKind::Description,
        StepKind::NativeStack,
    ] {
        for item in &plan.items {
            if item.steps.contains(&kind) {
                steps.push(Step {
                    branch: item.segment.branch.clone(),
                    kind: kind.clone(),
                    outcome: Outcome::Pending,
                    fingerprint: None,
                });
            }
        }
    }
    Receipt {
        plan: plan.clone(),
        steps,
        outcome: Outcome::Pending,
        pull_requests: vec![],
        message: None,
    }
}
async fn group(client: &Client, plan: &Plan, device: &Device) -> Result<Vec<PullRequest>> {
    let prs = client.list().await?;
    let recorded = device
        .receipt(&plan.request_id)
        .map(|r| r.pull_requests)
        .unwrap_or_default();
    plan.items
        .iter()
        .map(|i| {
            prs.iter()
                .find(|p| {
                    p.head == i.segment.branch
                        && p.state == PrState::Open
                        && recorded
                            .iter()
                            .find(|known| known.head == i.segment.branch)
                            .or(i.pr.as_ref())
                            .is_none_or(|known| known.number == p.number)
                })
                .cloned()
                .ok_or(Error::Stale)
        })
        .collect()
}
pub async fn submit(
    local: &dyn Local,
    client: &Client,
    device: &mut Device,
    plan: Plan,
    resume: bool,
) -> Result<Receipt> {
    if plan.items.iter().any(|i| {
        i.segment.title.trim().is_empty()
            || i.segment.title.contains('\n')
            || i.segment.body.len() > 128 * 1024
    }) {
        return Err(Error::Invalid);
    }
    if client.connection != plan.connection
        || device.connection().as_ref() != Some(&plan.connection)
    {
        return Err(Error::Stale);
    }
    let expected = planner::plan(
        plan.request_id.clone(),
        plan.operation.clone(),
        plan.connection.clone(),
        plan.items.iter().map(|i| i.segment.clone()).collect(),
        &plan
            .items
            .iter()
            .filter_map(|i| i.pr.clone())
            .collect::<Vec<_>>(),
        plan.draft,
    )?;
    if expected
        .items
        .iter()
        .zip(&plan.items)
        .any(|(a, b)| a.steps != b.steps)
    {
        return Err(Error::Stale);
    }
    let previous = device.receipt(&plan.request_id);
    if let Some(old) = &previous {
        if serde_json::to_vec(&old.plan).ok() != serde_json::to_vec(&plan).ok() {
            return Err(Error::Stale);
        }
        if !resume || old.outcome == Outcome::Complete {
            return Ok(old.clone());
        }
    }
    local.validate(&plan, previous.is_some()).await?;
    let mut r = previous.unwrap_or_else(|| initial(&plan));
    device.put(r.clone())?;
    for index in 0..r.steps.len() {
        if r.steps[index].outcome == Outcome::Complete {
            continue;
        }
        let step = r.steps[index].clone();
        let item = plan
            .items
            .iter()
            .find(|p| p.segment.branch == step.branch)
            .ok_or(Error::Invalid)?;
        let prs = client.list().await?;
        let observed = prs
            .iter()
            .find(|p| p.head == step.branch && p.state == PrState::Open)
            .cloned();
        if let Some(bound) = r
            .pull_requests
            .iter()
            .find(|p| p.head == step.branch)
            .or(item.pr.as_ref())
            && observed.as_ref().is_none_or(|p| p.number != bound.number)
        {
            return Err(Error::Stale);
        }
        let unknown = matches!(step.outcome, Outcome::Running | Outcome::Unknown);
        let satisfied = match step.kind {
            StepKind::Unstack => observed.as_ref().is_some_and(|p| p.native_stack.is_none()),
            StepKind::Push => {
                local.remote_head(&step.branch).await?.as_deref() == Some(&item.segment.head)
            }
            StepKind::Create => observed.as_ref().is_some_and(|p| {
                p.sha == item.segment.head
                    && p.base == item.segment.base
                    && p.body.contains(&marker(&plan, &step.branch))
            }),
            StepKind::Retarget => observed
                .as_ref()
                .is_some_and(|p| p.base == item.segment.base),
            StepKind::Description => {
                if let Some(p) = &observed {
                    if unknown {
                        step.fingerprint
                            .as_ref()
                            .is_some_and(|hash| *hash == fingerprint(&p.body))
                    } else {
                        let all = navigation(client, device, &plan).await?;
                        planner::body(&p.body, &all, p.number, plan.cumulative)? == p.body
                    }
                } else {
                    false
                }
            }
            StepKind::NativeStack => {
                let all = group(client, &plan, device).await?;
                client.native_stack_matches(&all).await?
            }
        };
        if satisfied {
            r.steps[index].outcome = Outcome::Complete;
            if let Some(pr) = observed {
                upsert(&mut r.pull_requests, pr);
            }
            device.put(r.clone())?;
            continue;
        }
        if unknown {
            r.outcome = Outcome::Unknown;
            r.message = Some(
                "Original effect remains unresolved. No automatic replay was attempted.".into(),
            );
            device.put(r.clone())?;
            return Ok(r);
        }
        let description = if step.kind == StepKind::Description {
            let all = navigation(client, device, &plan).await?;
            let pr = observed.as_ref().ok_or(Error::Stale)?;
            let latest = client.pull_request(pr.number).await?;
            Some(planner::body(
                &latest.body,
                &all,
                latest.number,
                plan.cumulative,
            )?)
        } else {
            None
        };
        if let Some(body) = &description {
            r.steps[index].fingerprint = Some(fingerprint(body));
        }
        local.authorize().await?;
        r.steps[index].outcome = Outcome::Running;
        r.outcome = Outcome::Running;
        device.put(r.clone())?;
        let mut changed = None;
        let result = match step.kind {
            StepKind::Unstack => {
                if let Some(p) = &observed {
                    client.unstack(p).await
                } else {
                    Err(Error::Stale)
                }
            }
            StepKind::Push => local.push(&item.segment).await,
            StepKind::Create => {
                if observed.is_some() {
                    Err(Error::Stale)
                } else {
                    let mut segment = item.segment.clone();
                    segment
                        .body
                        .push_str(&format!("\n\n{}", marker(&plan, &step.branch)));
                    client.create(&segment, plan.draft).await.map(|pr| {
                        changed = Some(pr);
                    })
                }
            }
            StepKind::Retarget => {
                if let Some(p) = &observed {
                    if p.native_stack.is_some() {
                        Err(Error::Stale)
                    } else {
                        client
                            .update(p.number, None, None, Some(&item.segment.base))
                            .await
                            .map(|pr| {
                                changed = Some(pr);
                            })
                    }
                } else {
                    Err(Error::Stale)
                }
            }
            StepKind::Description => {
                let p = observed.as_ref().ok_or(Error::Stale)?;
                client
                    .update(p.number, None, description.as_deref(), None)
                    .await
                    .map(|pr| {
                        changed = Some(pr);
                    })
            }
            StepKind::NativeStack => {
                client
                    .native_stack(&group(client, &plan, device).await?)
                    .await
            }
        };
        if let Err(error) = result {
            r.steps[index].outcome = if matches!(error, Error::Unknown) {
                Outcome::Unknown
            } else {
                Outcome::Rejected
            };
            r.outcome = r.steps[index].outcome.clone();
            r.message = Some(error.to_string());
            device.put(r.clone())?;
            return Ok(r);
        }
        r.steps[index].outcome = Outcome::Complete;
        if let Some(pr) = changed {
            upsert(&mut r.pull_requests, pr);
        }
        device.put(r.clone())?;
    }
    r.pull_requests = navigation(client, device, &plan).await?;
    r.outcome = Outcome::Complete;
    r.message = None;
    device.put(r.clone())?;
    Ok(r)
}
/// Only owned description navigation is maintained; local state is never rewritten.
pub async fn maintain(
    local: &dyn Local,
    client: &Client,
    device: &mut Device,
    prs: &[PullRequest],
) -> Result<()> {
    let mut maintained = std::collections::BTreeSet::new();
    for receipt in device
        .receipts()
        .into_iter()
        .rev()
        .filter(|r| r.plan.connection == client.connection && r.outcome == Outcome::Complete)
    {
        let group: Vec<_> = receipt
            .pull_requests
            .iter()
            .filter_map(|known| prs.iter().find(|p| p.number == known.number).cloned())
            .collect();
        if group.len() != receipt.pull_requests.len() {
            continue;
        }
        for p in &group {
            if !maintained.insert(p.number)
                || p.state != PrState::Open
                || !p.body.contains(planner::START)
            {
                continue;
            }
            let fresh = client.pull_request(p.number).await?;
            let body = planner::body(&fresh.body, &group, p.number, receipt.plan.cumulative)?;
            if body != fresh.body {
                local.authorize().await?;
                maintain_body(client, device, &fresh, body).await?;
            }
        }
    }
    Ok(())
}

/// Open discovery plus known PR identities; closed history is never scanned wholesale.
pub async fn observe(client: &Client, device: &Device) -> Result<Vec<PullRequest>> {
    let mut prs = client.list().await?;
    let mut ids: std::collections::BTreeSet<_> = prs.iter().map(|p| p.number).collect();
    let known: std::collections::BTreeSet<_> = device
        .receipts()
        .iter()
        .filter(|r| r.plan.connection == client.connection)
        .flat_map(|r| r.pull_requests.iter().map(|p| p.number))
        .collect();
    if known.len() > 256 {
        return Err(Error::Limit);
    }
    for number in known {
        if ids.insert(number) {
            prs.push(client.pull_request(number).await?);
        }
    }
    Ok(prs)
}

fn marker(plan: &Plan, branch: &str) -> String {
    format!(
        "<!-- jj-store:submission:{}:{} -->",
        plan.request_id, branch
    )
}
fn fingerprint(body: &str) -> String {
    use sha2::{Digest, Sha256};
    hex::encode(Sha256::digest(body.as_bytes()))
}
fn upsert(prs: &mut Vec<PullRequest>, pr: PullRequest) {
    if let Some(old) = prs.iter_mut().find(|p| p.number == pr.number) {
        *old = pr;
    } else {
        prs.push(pr);
    }
}
async fn navigation(client: &Client, device: &Device, plan: &Plan) -> Result<Vec<PullRequest>> {
    let live = observe(client, device).await?;
    let active = group(client, plan, device).await?;
    let mut branches: std::collections::BTreeSet<_> = plan
        .items
        .iter()
        .map(|i| i.segment.branch.clone())
        .collect();
    let receipts = device.receipts();
    for _ in 0..receipts.len() {
        let before = branches.len();
        for r in &receipts {
            if r.plan.connection == plan.connection
                && r.plan
                    .items
                    .iter()
                    .any(|i| branches.contains(&i.segment.branch))
            {
                branches.extend(r.plan.items.iter().map(|i| i.segment.branch.clone()));
            }
        }
        if branches.len() == before {
            break;
        }
    }
    let mut context = vec![];
    for r in &receipts {
        if r.plan.connection != plan.connection {
            continue;
        }
        for old in &r.pull_requests {
            if branches.contains(&old.head)
                && !active.iter().any(|p| p.number == old.number)
                && let Some(pr) = live.iter().find(|p| p.number == old.number)
            {
                upsert(&mut context, pr.clone());
            }
        }
    }
    context.extend(active);
    if context.len() > 64 {
        return Err(Error::Limit);
    }
    Ok(context)
}

/// Portable merges land only at trunk after every recorded predecessor merged.
pub fn merge_dependencies_met(
    pr: &PullRequest,
    connection: &Connection,
    receipts: &[Receipt],
    observed: &[PullRequest],
) -> bool {
    if pr.native_stack.is_some() {
        return connection.provider == Provider::Github;
    }
    if pr.base != connection.target {
        return false;
    }
    for receipt in receipts.iter().filter(|r| r.plan.connection == *connection) {
        let bound = receipt
            .pull_requests
            .iter()
            .find(|p| p.head == pr.head)
            .or_else(|| {
                receipt
                    .plan
                    .items
                    .iter()
                    .filter_map(|i| i.pr.as_ref())
                    .find(|p| p.head == pr.head)
            });
        if bound.is_none_or(|p| p.number != pr.number) {
            continue;
        }
        for item in receipt
            .plan
            .items
            .iter()
            .take_while(|i| i.segment.branch != pr.head)
        {
            let parent = receipt
                .pull_requests
                .iter()
                .find(|p| p.head == item.segment.branch)
                .or(item.pr.as_ref());
            if parent.is_none_or(|known| {
                !observed
                    .iter()
                    .any(|actual| actual.number == known.number && actual.state == PrState::Merged)
            }) {
                return false;
            }
        }
    }
    true
}

async fn maintain_body(
    client: &Client,
    device: &mut Device,
    pr: &PullRequest,
    body: String,
) -> Result<()> {
    if let Some(old) = device.unresolved_operation(Some(pr.number)) {
        if let PrAction::Edit {
            title,
            body: expected,
            ..
        } = &old.action
        {
            if pr.title == *title && pr.body == *expected {
                device.finish_operation(&old.id, Ok(None))?;
            } else {
                return Err(Error::Unknown);
            }
        } else {
            return Err(Error::Unknown);
        }
    }
    let id = format!(
        "navigation-{}-{}",
        pr.number,
        fingerprint(&format!("{}\0{}\0{}", body, pr.body, pr.updated_at))
    );
    let action = PrAction::Edit {
        title: pr.title.clone(),
        body: body.clone(),
        expected_body: pr.body.clone(),
    };
    if let Some(previous) = device.begin_operation(&id, pr.number, &action)? {
        match previous.outcome {
            Outcome::Complete => return Ok(()),
            Outcome::Rejected => device.restart_rejected(&id)?,
            _ => return Err(Error::Unknown),
        }
    }
    let result = client
        .update(pr.number, None, Some(&body), None)
        .await
        .map(|_| None);
    device.finish_operation(&id, result)?;
    Ok(())
}
