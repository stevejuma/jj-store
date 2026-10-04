//! Bounded metadata candidates; native admission owns directory and Write policy.
use super::*;
pub struct OpenCodeProjectInventory {
    pub projects: Vec<String>,
    pub unsupported: usize,
}
pub struct OpenCodeDirectoryInventory {
    pub directories: Vec<String>,
    pub unsupported: usize,
}
fn safe_directory(value: &str) -> bool {
    let path = std::path::Path::new(value);
    value.len() <= 4096
        && !value.chars().any(char::is_control)
        && path.is_absolute()
        && path.parent().is_some()
        && !value.split('/').any(|part| part == "." || part == "..")
        && !path.components().any(|c| {
            matches!(
                c,
                std::path::Component::CurDir | std::path::Component::ParentDir
            )
        })
}
fn project_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || c == b'_' || c == b'-')
}
impl OpenCodeReadApi {
    pub async fn discover_active(&self) -> Result<OpenCodeApiDiscovery> {
        self.discover_status(&[]).await
    }
    pub async fn discover_active_in(&self, directory: &str) -> Result<OpenCodeApiDiscovery> {
        if !safe_directory(directory) {
            return Err(Error::Service("OpenCode discovery directory unavailable"));
        }
        self.discover_status(&[("directory", directory)]).await
    }
    async fn discover_status(&self, query: &[(&str, &str)]) -> Result<OpenCodeApiDiscovery> {
        self.health().await?;
        let (bytes, _) = self.get(&["session", "status"], query).await?;
        let value: Value = serde_json::from_slice(&bytes)
            .map_err(|_| Error::Service("OpenCode status map unavailable"))?;
        let map = value
            .as_object()
            .filter(|m| m.len() <= 4096)
            .ok_or(Error::Service("OpenCode status map unavailable"))?;
        let mut result = OpenCodeApiDiscovery {
            candidates: Vec::new(),
            observed: map.len(),
            unsupported: 0,
        };
        for (id, status) in map {
            if !native_id_valid(id, "ses") {
                result.unsupported += 1;
                continue;
            }
            match status["type"].as_str() {
                Some("busy" | "retry") => result.candidates.push(OpenCodeSessionCandidate {
                    native_session: id.clone(),
                }),
                Some("idle") => {}
                _ => result.unsupported += 1,
            }
        }
        Ok(result)
    }
    pub async fn project_inventory(&self) -> Result<OpenCodeProjectInventory> {
        self.health().await?;
        let (bytes, _) = self.get(&["project"], &[]).await?;
        let value: Value = serde_json::from_slice(&bytes)
            .map_err(|_| Error::Service("OpenCode project inventory unavailable"))?;
        let rows = value
            .as_array()
            .filter(|r| r.len() <= 4096)
            .ok_or(Error::Service("OpenCode project inventory unavailable"))?;
        let mut result = OpenCodeProjectInventory {
            projects: Vec::new(),
            unsupported: 0,
        };
        let mut seen = BTreeSet::new();
        for row in rows {
            if let Some(id) = row["id"].as_str().filter(|s| project_id(s)) {
                if seen.insert(id.to_owned()) {
                    result.projects.push(id.to_owned());
                }
            } else {
                result.unsupported += 1;
            }
        }
        Ok(result)
    }
    pub async fn project_directories(&self, project: &str) -> Result<OpenCodeDirectoryInventory> {
        if !project_id(project) {
            return Err(Error::Service("OpenCode project identity unavailable"));
        }
        self.health().await?;
        let (bytes, _) = self.get(&["project", project, "directories"], &[]).await?;
        directory_inventory(&bytes, false)
    }
    /// Global/non-Git projects deliberately do not persist project-directory rows.
    /// Bounded historical metadata offers directories only; status still proves activity.
    pub async fn global_session_directories(&self) -> Result<OpenCodeDirectoryInventory> {
        self.health().await?;
        let (bytes, _) = self
            .get(
                &["session"],
                &[("directory", "/"), ("scope", "project"), ("limit", "4096")],
            )
            .await?;
        directory_inventory(&bytes, true)
    }
}

fn directory_inventory(
    bytes: &[u8],
    truncated_at_limit: bool,
) -> Result<OpenCodeDirectoryInventory> {
    let value: Value = serde_json::from_slice(bytes)
        .map_err(|_| Error::Service("OpenCode directory inventory unavailable"))?;
    let rows = value
        .as_array()
        .filter(|r| r.len() <= 4096)
        .ok_or(Error::Service("OpenCode directory inventory unavailable"))?;
    let mut result = OpenCodeDirectoryInventory {
        directories: Vec::new(),
        unsupported: usize::from(truncated_at_limit && rows.len() == 4096),
    };
    let mut seen = BTreeSet::new();
    for row in rows {
        if let Some(directory) = row["directory"].as_str().filter(|s| safe_directory(s)) {
            if seen.insert(directory.to_owned()) {
                result.directories.push(directory.to_owned());
            }
        } else {
            result.unsupported += 1;
        }
    }
    Ok(result)
}
