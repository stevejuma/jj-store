//! Resolve only existing, revalidated folder bindings from the owned service.
use super::Result;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::{io::Read, time::Duration};
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Target {
    pub id: String,
    pub repository: String,
    pub path: String,
    pub label: String,
}
#[derive(Clone)]
pub struct TargetResolver {
    origin: Url,
    client: Client,
}
impl TargetResolver {
    pub fn new(origin: Url) -> Result<Self> {
        if origin.scheme() != "http"
            || origin.host_str() != Some("127.0.0.1")
            || origin.port().is_none()
            || !origin.username().is_empty()
            || origin.password().is_some()
            || origin.path() != "/"
            || origin.query().is_some()
            || origin.fragment().is_some()
        {
            return Err("Terminal requires the managed loopback service".into());
        }
        let client = Client::builder()
            .no_proxy()
            .redirect(reqwest::redirect::Policy::none())
            .timeout(Duration::from_secs(5))
            .build()
            .map_err(|e| e.to_string())?;
        Ok(Self { origin, client })
    }
    pub fn origin(&self) -> &Url {
        &self.origin
    }
    fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = self.origin.join(path).map_err(|e| e.to_string())?;
        let response = self
            .client
            .get(url)
            .header("Origin", self.origin.origin().ascii_serialization())
            .header("x-jj-store-protocol", "1")
            .header("Sec-Fetch-Site", "same-origin")
            .header("Sec-Fetch-Mode", "cors")
            .header("Sec-Fetch-Dest", "empty")
            .send()
            .map_err(|_| "Local folder service is unavailable")?;
        if response.status() != reqwest::StatusCode::OK {
            return Err("Local folder read was denied".into());
        }
        if !response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .is_some_and(|v| v.starts_with("application/json"))
        {
            return Err("Invalid local folder response".into());
        }
        let mut bytes = Vec::new();
        response
            .take(1024 * 1024 + 1)
            .read_to_end(&mut bytes)
            .map_err(|e| e.to_string())?;
        if bytes.len() > 1024 * 1024 {
            return Err("Local folder response exceeds limit".into());
        }
        serde_json::from_slice(&bytes).map_err(|_| "Invalid local folder response".into())
    }
    pub fn list(&self, repository: &str, profile: &str) -> Result<Vec<Target>> {
        if repository.len() != 32
            || !repository
                .bytes()
                .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
        {
            return Err("Invalid repository identity".into());
        }
        let targets = match profile {
            "git_sha1_jj045" => {
                let value: Bindings = self.get("v1/local/repositories")?;
                if value.version != 1 || value.bindings.len() > 256 {
                    return Err("Invalid folder bindings".into());
                }
                value
                    .bindings
                    .into_iter()
                    .filter(|v| v.repository == repository && v.availability == "ready")
                    .map(|v| Target {
                        id: format!("local:{}:{}", repository, v.generation),
                        repository: repository.into(),
                        path: v.path,
                        label: v.name,
                    })
                    .collect()
            }
            "native" => {
                let value: Links = self.get(&format!("v1/repos/{repository}/local/workspaces"))?;
                if value.version != 1 || value.links.len() > 64 {
                    return Err("Invalid workspace links".into());
                }
                value
                    .links
                    .into_iter()
                    .filter(|v| v.availability == "ready")
                    .map(|v| Target {
                        id: format!("workspace:{}:{}", value.revision, v.id),
                        repository: repository.into(),
                        label: v.path.clone(),
                        path: v.path,
                    })
                    .collect()
            }
            _ => return Err("Unsupported repository profile".into()),
        };
        Ok(targets)
    }
    pub fn resolve(&self, repository: &str, profile: &str, id: &str) -> Result<Target> {
        let target = self
            .list(repository, profile)?
            .into_iter()
            .find(|v| v.id == id)
            .ok_or("The local folder link changed. Refresh and select it again.")?;
        let path = std::path::Path::new(&target.path);
        if target.path.len() > 4096 || !path.is_absolute() || !path.is_dir() {
            return Err("The linked local folder is unavailable".into());
        }
        Ok(target)
    }
}
#[derive(Deserialize)]
struct Bindings {
    version: u32,
    bindings: Vec<Binding>,
}
#[derive(Deserialize)]
struct Binding {
    repository: String,
    name: String,
    path: String,
    generation: u64,
    availability: String,
}
#[derive(Deserialize)]
struct Links {
    version: u32,
    revision: u64,
    links: Vec<Link>,
}
#[derive(Deserialize)]
struct Link {
    id: String,
    path: String,
    availability: String,
}
