//! Supported local Actions trigger filtering; execution semantics stay in Forgejo.
use crate::{Error, MAX_TEXT, Result, text};
use globset::GlobBuilder;
use serde_yaml::Value;

pub fn workflow_path(path: &str) -> Result<()> {
    if path.len() > 1024
        || !path.starts_with(".github/workflows/")
        || !(path.ends_with(".yml") || path.ends_with(".yaml"))
        || path
            .split('/')
            .any(|p| p.is_empty() || p == "." || p == "..")
        || path.contains('\\')
        || path.chars().any(char::is_control)
    {
        return Err(Error::Invalid("GitHub workflow path"));
    }
    Ok(())
}

pub struct Context<'a> {
    pub event: &'a str,
    pub branch: Option<&'a str>,
    /// None means the pinned comparison cannot be established, not no changes.
    pub paths: Option<&'a [String]>,
}
pub struct Workflow {
    pub document: Value,
}
impl Workflow {
    pub fn parse(yaml: &str) -> Result<Self> {
        text(yaml, MAX_TEXT)?;
        let document: Value =
            serde_yaml::from_str(yaml).map_err(|_| Error::Invalid("Actions YAML"))?;
        if !document.is_mapping()
            || !document["jobs"].is_mapping()
            || document["jobs"]
                .as_mapping()
                .is_none_or(|j| j.is_empty() || j.len() > 32)
        {
            return Err(Error::Invalid("Actions jobs"));
        }
        if !document["concurrency"].is_null() {
            return Err(Error::Invalid("workflow concurrency unsupported"));
        }
        for job in document["jobs"].as_mapping().expect("jobs").values() {
            if !job["uses"].is_null() || !job["services"].is_null() || !job["container"].is_null() {
                return Err(Error::Invalid(
                    "reusable workflows, services and container overrides unsupported",
                ));
            }
            if !job["steps"].is_sequence()
                || job["steps"].as_sequence().is_none_or(|s| s.len() > 256)
            {
                return Err(Error::Invalid("Actions steps"));
            }
        }
        Ok(Self { document })
    }
    pub fn matches(&self, context: Context<'_>) -> Result<bool> {
        let on = &self.document["on"];
        if let Some(event) = on.as_str() {
            return Ok(event == context.event);
        }
        if let Some(events) = on.as_sequence() {
            if events.iter().any(|v| !v.is_string()) {
                return Err(Error::Invalid("Actions events"));
            }
            return Ok(events.iter().any(|v| v.as_str() == Some(context.event)));
        }
        let events = on.as_mapping().ok_or(Error::Invalid("Actions on"))?;
        let Some(filters) = events.get(Value::String(context.event.into())) else {
            return Ok(false);
        };
        if filters.is_null() {
            return Ok(true);
        }
        let filters = filters
            .as_mapping()
            .ok_or(Error::Invalid("Actions event filters"))?;
        if context.event == "workflow_dispatch" {
            if filters.keys().any(|k| k.as_str() != Some("inputs")) {
                return Err(Error::Invalid("dispatch filters"));
            }
            if filters
                .get(Value::String("inputs".into()))
                .is_some_and(|v| !v.is_null())
            {
                return Err(Error::Invalid("dispatch inputs unsupported"));
            }
            return Ok(true);
        }
        if context.event != "push" {
            return Ok(false);
        }
        for key in filters.keys() {
            if !matches!(
                key.as_str(),
                Some(
                    "branches"
                        | "branches-ignore"
                        | "paths"
                        | "paths-ignore"
                        | "tags"
                        | "tags-ignore"
                )
            ) {
                return Err(Error::Invalid("push filter unsupported"));
            }
        }
        let get = |name: &str| -> Result<Option<Vec<String>>> {
            let Some(value) = filters.get(Value::String(name.into())) else {
                return Ok(None);
            };
            let values = value.as_sequence().ok_or(Error::Invalid("filter list"))?;
            if values.is_empty() || values.len() > 64 {
                return Err(Error::Invalid("filter list bounds"));
            }
            values
                .iter()
                .map(|v| {
                    v.as_str()
                        .map(str::to_owned)
                        .ok_or(Error::Invalid("filter pattern"))
                })
                .collect::<Result<Vec<_>>>()
                .map(Some)
        };
        let branches = get("branches")?;
        let excluded = get("branches-ignore")?;
        let paths = get("paths")?;
        let ignored = get("paths-ignore")?;
        if branches.is_some() && excluded.is_some() || paths.is_some() && ignored.is_some() {
            return Err(Error::Invalid("include and ignore filters conflict"));
        }
        // This profile observes branch changes, not tag publications.
        if get("tags")?.is_some() && branches.is_none() && excluded.is_none() {
            return Ok(false);
        }
        let _ = get("tags-ignore")?;
        if let Some(patterns) = branches {
            let _ = match_patterns(&patterns, "")?;
            if let Some(branch) = context.branch {
                if !match_patterns(&patterns, branch)? {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        }
        if let Some(patterns) = excluded {
            if let Some(branch) = context.branch {
                if match_patterns(&patterns, branch)? {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        }
        if paths.is_some() || ignored.is_some() {
            let changed = context
                .paths
                .ok_or(Error::Invalid("changed paths unavailable"))?;
            if let Some(patterns) = paths {
                let mut found = false;
                for path in changed {
                    found |= match_patterns(&patterns, path)?;
                }
                if !found {
                    return Ok(false);
                }
            }
            if let Some(patterns) = ignored {
                let mut all = true;
                for path in changed {
                    all &= match_patterns(&patterns, path)?;
                }
                if all {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }
    /// Only the private execution copy receives an attempt-specific name.
    pub fn execution_yaml(&self, name: &str) -> Result<String> {
        let mut document = self.document.clone();
        document["name"] = Value::String(name.into());
        for job in document["jobs"]
            .as_mapping_mut()
            .ok_or(Error::Invalid("Actions jobs"))?
            .values_mut()
        {
            let steps = job["steps"]
                .as_sequence_mut()
                .ok_or(Error::Invalid("Actions steps"))?;
            let mut checkout = false;
            for step in steps.iter() {
                if step["uses"].as_str().is_some_and(|value| {
                    value.starts_with("actions/checkout@")
                        || value.starts_with("https://github.com/actions/checkout@")
                }) {
                    checkout = true;
                    for key in ["repository", "ref", "path"] {
                        if !step["with"][key].is_null() {
                            return Err(Error::Invalid("checkout source overrides unsupported"));
                        }
                    }
                }
            }
            if !checkout {
                let setup: Value = serde_yaml::from_str(
                    "name: Prepare pinned source\nid: jj-store-source\nuses: actions/checkout@v4\n",
                )
                .map_err(|_| Error::Invalid("source setup"))?;
                steps.insert(0, setup);
            }
        }
        let yaml = serde_yaml::to_string(&document)
            .map_err(|_| Error::Invalid("Actions YAML encoding"))?;
        text(&yaml, MAX_TEXT)?;
        Ok(yaml)
    }
}
fn match_patterns(patterns: &[String], value: &str) -> Result<bool> {
    let mut matched = false;
    let mut positive = false;
    for pattern in patterns {
        let (negated, pattern) = pattern
            .strip_prefix('!')
            .map_or((false, pattern.as_str()), |p| (true, p));
        if pattern.is_empty() || pattern.len() > 1024 || pattern.contains('+') {
            return Err(Error::Invalid("unsupported glob pattern"));
        }
        positive |= !negated;
        let glob = GlobBuilder::new(pattern)
            .literal_separator(true)
            .backslash_escape(true)
            .build()
            .map_err(|_| Error::Invalid("glob pattern"))?;
        if glob.compile_matcher().is_match(value) {
            matched = !negated;
        }
    }
    if !positive {
        return Err(Error::Invalid("filter requires a positive pattern"));
    }
    Ok(matched)
}
