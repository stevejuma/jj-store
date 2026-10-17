//! Complete bounded scan of selected aliases; incomplete scans grant no identity proof.
use super::*;
use jj_agent::AgentIndexPageV1;

const MAX_ALIASES: usize = 4096;
const MAX_PAGES: usize = 8192;

pub(super) async fn selected_aliases(
    base: &dyn AgentSnapshotRead,
) -> Result<Option<Vec<AgentAliasV1>>> {
    scan(base, MAX_ALIASES, MAX_PAGES).await
}

async fn scan(
    base: &dyn AgentSnapshotRead,
    max_aliases: usize,
    max_pages: usize,
) -> Result<Option<Vec<AgentAliasV1>>> {
    let mut stack: Vec<_> = base
        .root()
        .aliases
        .into_iter()
        .map(|r| (r, String::new()))
        .collect();
    let mut seen = BTreeSet::new();
    let mut aliases = Vec::new();
    while let Some((reference, expected_prefix)) = stack.pop() {
        if seen.len() == max_pages {
            return Ok(None);
        }
        if !seen.insert(reference) {
            return Err(Error::Corrupt("capture alias catalog cycle"));
        }
        let AgentObjectV1::IndexPage(page) = base.read_object(reference).await? else {
            return Err(Error::Corrupt("capture alias catalog page"));
        };
        match page {
            AgentIndexPageV1::Leaf { prefix, entries } => {
                if prefix != expected_prefix {
                    return Err(Error::Corrupt("capture alias catalog prefix"));
                }
                for (key, reference) in entries {
                    let values = read_aliases(base, key, reference).await?;
                    if aliases.len() + values.len() > max_aliases {
                        return Ok(None);
                    }
                    aliases.extend(values);
                }
            }
            AgentIndexPageV1::Branch { prefix, children } => {
                if prefix != expected_prefix {
                    return Err(Error::Corrupt("capture alias catalog prefix"));
                }
                stack.extend(
                    children
                        .into_iter()
                        .map(|(nibble, child)| (child, format!("{prefix}{nibble}"))),
                );
            }
        }
    }
    Ok(Some(aliases))
}

#[cfg(test)]
#[path = "catalog_tests.rs"]
mod tests;

pub(super) async fn exact_aliases(
    base: &dyn AgentSnapshotRead,
    alias: &AgentAliasV1,
) -> Result<Vec<AgentAliasV1>> {
    let key = alias.catalog_key().map_err(agent_error)?;
    match base.lookup(AgentCatalogKind::Aliases, key).await? {
        Some(reference) => read_aliases(base, key, reference).await,
        None => Ok(Vec::new()),
    }
}

async fn read_aliases(
    base: &dyn AgentSnapshotRead,
    key: AgentCatalogKey,
    reference: AgentObjectRef,
) -> Result<Vec<AgentAliasV1>> {
    let objects = match base.read_object(reference).await? {
        AgentObjectV1::Alias(alias) => vec![alias],
        AgentObjectV1::CatalogConflict(conflict)
            if conflict.catalog == AgentCatalogKind::Aliases && conflict.key == key =>
        {
            let mut aliases = Vec::new();
            for alternative in conflict.alternatives {
                let AgentObjectV1::Alias(alias) = base.read_object(alternative).await? else {
                    return Err(Error::Corrupt("capture alias alternative"));
                };
                aliases.push(alias);
            }
            aliases
        }
        _ => return Err(Error::Corrupt("capture alias selection")),
    };
    for alias in &objects {
        if alias.catalog_key().map_err(agent_error)? != key {
            return Err(Error::Corrupt("capture alias key"));
        }
    }
    Ok(objects)
}
