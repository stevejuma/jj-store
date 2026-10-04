//! Explicit bounded ref exchange over the owned transport.
use super::*;
pub async fn fetch(
    cache: &Path,
    endpoint: &str,
    patterns: &[String],
) -> Result<BTreeMap<String, String>> {
    qualify(cache).await?;
    let mut args = vec![
        "fetch".into(),
        "--no-write-fetch-head".into(),
        "--no-tags".into(),
        "--prune".into(),
        "--".into(),
        endpoint.into(),
    ];
    // Inspect one completed fetch, including literal names that may be absent.
    // The bounded cache can contain other heads; only the requested scope is
    // converted or recorded in the native repository.
    args.push("+refs/heads/*:refs/bridge/heads/*".into());
    args.push("+refs/tags/*:refs/bridge/tags/*".into());
    if !run(cache, &args).await?.success {
        return Err("Git fetch failed; no import accepted".into());
    }
    let output = run(
        cache,
        &[
            "for-each-ref".into(),
            "--format=%(refname) %(objectname)".into(),
            "refs/bridge/".into(),
        ],
    )
    .await?;
    if !output.success {
        return Err("Git fetched ref inspection failed".into());
    }
    let mut refs = BTreeMap::new();
    for line in std::str::from_utf8(&output.stdout)?.lines() {
        let (name, oid) = line.split_once(' ').ok_or("Git ref output shape")?;
        let name = if let Some(name) = name.strip_prefix("refs/bridge/heads/") {
            format!("refs/heads/{name}")
        } else if let Some(name) = name.strip_prefix("refs/bridge/tags/") {
            format!("refs/tags/{name}")
        } else {
            return Err("Git fetched namespace mismatch".into());
        };
        if !valid_git_ref(&name)
            || !valid_git_oid(oid)
            || refs.insert(name, oid.into()).is_some()
            || refs.len() > 4096
        {
            return Err("Git fetched ref validation/limit".into());
        }
    }
    refs.retain(|name, _| {
        name.strip_prefix("refs/heads/").is_none_or(|name| {
            patterns
                .iter()
                .any(|pattern| match pattern.split_once('*') {
                    Some((prefix, suffix)) => {
                        name.starts_with(prefix)
                            && name.ends_with(suffix)
                            && name.len() >= prefix.len() + suffix.len()
                    }
                    None => pattern == name,
                })
        })
    });
    Ok(refs)
}
pub async fn push(
    cache: &Path,
    endpoint: &str,
    refs: &BTreeMap<String, GitPushRefV1>,
) -> Result<BTreeMap<String, GitPushRefOutcomeV1>> {
    qualify(cache).await?;
    let mut args = vec!["push".into(), "--porcelain".into(), "--no-verify".into()];
    if refs.len() > 1 {
        args.push("--atomic".into());
    }
    for (name, value) in refs {
        args.push(format!(
            "--force-with-lease={name}:{}",
            value.expected.as_deref().unwrap_or("")
        ));
    }
    args.push("--".into());
    args.push(endpoint.into());
    for (name, value) in refs {
        args.push(format!(
            "{}:{name}",
            value.desired.oid.as_deref().unwrap_or("")
        ));
    }
    let output = run(cache, &args).await?;
    let mut outcomes = BTreeMap::new();
    for line in std::str::from_utf8(&output.stdout)?.lines() {
        let fields: Vec<_> = line.split('\t').collect();
        if fields.len() != 3 {
            continue;
        }
        let (_, reference) = fields[1]
            .split_once(':')
            .ok_or("Git push status ref shape")?;
        if !refs.contains_key(reference) || outcomes.contains_key(reference) {
            return Err("Git push status scope".into());
        }
        let status = match fields[0] {
            "=" => GitPushRefOutcomeV1::ObservedSatisfied,
            " " | "+" | "-" | "*" => GitPushRefOutcomeV1::Acknowledged,
            "!" => GitPushRefOutcomeV1::Rejected,
            _ => return Err("Git push status shape".into()),
        };
        outcomes.insert(reference.into(), status);
    }
    if outcomes.len() != refs.len()
        || (!output.success
            && !outcomes
                .values()
                .all(|v| *v == GitPushRefOutcomeV1::Rejected))
    {
        return Ok(refs
            .keys()
            .map(|name| (name.clone(), GitPushRefOutcomeV1::Unknown))
            .collect());
    }
    Ok(outcomes)
}
pub async fn inspect<'a>(
    cache: &Path,
    endpoint: &str,
    refs: impl Iterator<Item = &'a String>,
) -> Result<BTreeMap<String, String>> {
    qualify(cache).await?;
    let selected: BTreeSet<_> = refs.cloned().collect();
    let mut args = vec![
        "ls-remote".into(),
        "--refs".into(),
        "--".into(),
        endpoint.into(),
    ];
    args.extend(selected.iter().cloned());
    let output = run(cache, &args).await?;
    if !output.success {
        return Err("Git recovery ref inspection incomplete; outcome remains unknown".into());
    }
    let mut observed = BTreeMap::new();
    for line in std::str::from_utf8(&output.stdout)?.lines() {
        let (oid, name) = line.split_once('\t').ok_or("Git inspection shape")?;
        if !valid_git_oid(oid)
            || !valid_git_ref(name)
            || !selected.contains(name)
            || observed.insert(name.into(), oid.into()).is_some()
        {
            return Err("Git inspection scope/OID".into());
        }
    }
    Ok(observed)
}
