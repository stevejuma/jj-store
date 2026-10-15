use crate::*;
pub const START: &str = "<!-- jj-store:stack:v1 -->";
pub const END: &str = "<!-- /jj-store:stack -->";
/// Push and retarget are independent effects.
pub fn effects(segment: &Segment, current_base: Option<&str>) -> Vec<StepKind> {
    let mut steps = vec![];
    if segment.remote_head.as_deref() != Some(&segment.head) {
        steps.push(StepKind::Push);
    }
    match current_base {
        None => steps.push(StepKind::Create),
        Some(base) if base != segment.base => steps.push(StepKind::Retarget),
        _ => {}
    }
    steps.push(StepKind::Description);
    steps
}
/// Preserve every authored byte outside the single owned navigation block.
pub fn body(authored: &str, prs: &[PullRequest], number: u64, cumulative: bool) -> Result<String> {
    if authored.len() > 128 * 1024 || prs.len() > 64 {
        return Err(Error::Limit);
    }
    let mut block = format!("{START}\n\n### Stack\n\n");
    if cumulative {
        block.push_str("Fork stack: these upstream PRs have cumulative diffs. Merge bottom first and restack before merging the next layer.\n\n");
    }
    for (i, p) in prs.iter().enumerate() {
        let u = url::Url::parse(&p.url).map_err(|_| Error::Invalid)?;
        if u.scheme() != "https" || !u.username().is_empty() || u.password().is_some() {
            return Err(Error::Invalid);
        }
        block.push_str(&format!(
            "{}. [#{}]({}) · {:?}{}\n",
            i + 1,
            p.number,
            p.url,
            p.state,
            if p.number == number {
                " ← this PR"
            } else {
                ""
            }
        ));
    }
    block.push_str(END);
    let starts: Vec<_> = authored.match_indices(START).collect();
    let ends: Vec<_> = authored.match_indices(END).collect();
    match (starts.as_slice(), ends.as_slice()) {
        ([], []) => Ok(format!("{authored}\n\n{block}")),
        ([(a, _)], [(b, _)]) if a < b => Ok(format!(
            "{}{}{}",
            &authored[..*a],
            block,
            &authored[b + END.len()..]
        )),
        _ => Err(Error::Stale),
    }
}
pub fn plan(
    request_id: String,
    operation: String,
    connection: Connection,
    segments: Vec<Segment>,
    prs: &[PullRequest],
    draft: bool,
) -> Result<Plan> {
    connection.validate()?;
    if segments.is_empty()
        || segments.len() > 64
        || !safe_name(&request_id)
        || (operation.len() != 128 || !operation.bytes().all(|b| b.is_ascii_hexdigit()))
    {
        return Err(Error::Invalid);
    }
    let cumulative = connection.fork();
    let mut names = std::collections::BTreeSet::new();
    let mut items = vec![];
    for mut s in segments {
        if !safe_name(&s.branch)
            || !names.insert(s.branch.clone())
            || (s.head.len() != 40 || !s.head.bytes().all(|b| b.is_ascii_hexdigit()))
        {
            return Err(Error::Invalid);
        }
        if cumulative {
            s.base = connection.target.clone();
        }
        let matches: Vec<_> = prs
            .iter()
            .filter(|p| p.head == s.branch && p.state == PrState::Open)
            .collect();
        if matches.len() > 1 {
            return Err(Error::Stale);
        }
        if matches.is_empty()
            && prs
                .iter()
                .any(|p| p.head == s.branch && p.state != PrState::Open)
        {
            return Err(Error::Stale);
        }
        let pr = matches.first().cloned().cloned();
        let steps = effects(&s, pr.as_ref().map(|p| p.base.as_str()));
        items.push(PlanItem {
            segment: s,
            pr,
            steps,
        });
    }
    let native = connection.provider == Provider::Github && connection.native_stacks && !cumulative;
    let existing = items
        .iter()
        .any(|i| i.pr.as_ref().is_some_and(|p| p.native_stack.is_some()));
    if native && existing && items.iter().any(|i| i.segment.shared) {
        return Err(Error::Unsupported);
    }
    let reshape = existing
        && items.iter().any(|i| {
            i.pr.as_ref().is_none_or(|p| p.native_stack.is_none())
                || i.steps.contains(&StepKind::Retarget)
        });
    if reshape {
        for item in &mut items {
            if item.pr.as_ref().is_some_and(|p| p.native_stack.is_some()) {
                item.steps.insert(0, StepKind::Unstack);
            }
        }
    }
    if native && items.len() > 1 && !items.iter().any(|i| i.segment.shared) {
        items
            .last_mut()
            .ok_or(Error::Invalid)?
            .steps
            .push(StepKind::NativeStack);
    }
    Ok(Plan {
        request_id,
        operation,
        connection,
        draft,
        cumulative,
        items,
    })
}

/// Replace authored text while retaining the current owned navigation verbatim.
pub fn edit_body(current: &str, edited: &str) -> Result<String> {
    let Some(start) = current.find(START) else {
        return Ok(edited.to_string());
    };
    let end = current
        .find(END)
        .filter(|end| *end > start)
        .ok_or(Error::Stale)?
        + END.len();
    if edited.contains(START) {
        let a = edited.find(START).ok_or(Error::Stale)?;
        let b = edited
            .find(END)
            .filter(|end| *end > a)
            .ok_or(Error::Stale)?
            + END.len();
        return Ok(format!(
            "{}{}{}",
            &edited[..a],
            &current[start..end],
            &edited[b..]
        ));
    }
    Ok(format!("{edited}\n\n{}", &current[start..end]))
}
