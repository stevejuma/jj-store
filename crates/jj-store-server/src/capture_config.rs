//! Strict startup-selected capture settings; no source I/O or enablement side effects.
use crate::{
    Error, Result,
    config::{Access, Storage},
};
use jj_store_capture::profile::CaptureProfile;
use jj_store_service::auth::Permission;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct AgentCaptureConfig {
    pub enabled: bool,
    pub max_readers: usize,
    pub rescan_seconds: u64,
    pub max_sources: usize,
    pub max_pending_bytes: u64,
    pub harnesses: Vec<HarnessConfig>,
}
impl Default for AgentCaptureConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            max_readers: 4,
            rescan_seconds: 30,
            max_sources: 1024,
            max_pending_bytes: 64 * 1024 * 1024,
            harnesses: Vec::new(),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HarnessConfig {
    pub profile: CaptureProfile,
    pub product_version: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roots: Vec<PathBuf>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}
impl HarnessConfig {
    pub fn roots(&self, home: Option<&Path>, codex_home: Option<&Path>) -> Result<Vec<PathBuf>> {
        match self.profile {
            CaptureProfile::CodexCliJsonlV1 | CaptureProfile::CodexDesktopJsonlV1 => {
                self.codex_roots(home, codex_home)
            }
            CaptureProfile::CursorDesktopTranscriptV1 | CaptureProfile::CursorCliTranscriptV1 => {
                if !self.roots.is_empty() {
                    return Ok(self.roots.clone());
                }
                let data = if self.profile == CaptureProfile::CursorCliTranscriptV1 {
                    std::env::var_os("CURSOR_DATA_DIR").map(PathBuf::from)
                } else {
                    None
                }
                .or_else(|| home.map(|p| p.join(".cursor")))
                .ok_or(Error::Config("Cursor capture home unavailable"))?;
                if !data.is_absolute()
                    || data.parent().is_none()
                    || data
                        .to_str()
                        .is_none_or(|s| s.len() > 4096 || s.chars().any(char::is_control))
                    || data.components().any(|p| {
                        matches!(
                            p,
                            std::path::Component::CurDir | std::path::Component::ParentDir
                        )
                    })
                {
                    return Err(Error::Config("Cursor capture root unavailable"));
                }
                Ok(vec![data.join("projects")])
            }
            CaptureProfile::OpencodeReadOnlyApiV1 => Ok(vec![]),
            CaptureProfile::OpencodeExportJsonV1 => {
                Err(Error::Config("capture export unavailable"))
            }
        }
    }
    /// Pure root selection; callers must separately enable and prepare capture.
    pub fn codex_roots(
        &self,
        home: Option<&Path>,
        codex_home: Option<&Path>,
    ) -> Result<Vec<PathBuf>> {
        if self.product_version != self.profile.product_version() || self.endpoint.is_some() {
            return Err(Error::Config("Codex capture location profile"));
        }
        jj_store_capture::codex_discovery::resolve_codex_roots(
            self.profile,
            &self.roots,
            home,
            codex_home,
        )
        .map_err(|_| Error::Config("Codex capture locations unavailable"))
    }
}
impl AgentCaptureConfig {
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }
    pub(crate) fn resolve_paths(&mut self, base: &Path) {
        for harness in &mut self.harnesses {
            for root in &mut harness.roots {
                *root = base.join(&*root);
            }
        }
    }
    pub fn validate(&self, access: &Access, storage: &Storage) -> Result<()> {
        if !(1..=4).contains(&self.max_readers)
            || !(1..=30).contains(&self.rescan_seconds)
            || !(1..=4096).contains(&self.max_sources)
            || !(1..=64 * 1024 * 1024).contains(&self.max_pending_bytes)
            || self.harnesses.len() > 16
            || (self.enabled && self.harnesses.is_empty())
        {
            return Err(Error::Config(
                "agent capture resource budgets or missing profiles",
            ));
        }
        if self.enabled
            && (!matches!(
                access,
                Access::LocalTrusted {
                    permission: Permission::Write | Permission::Admin,
                    ..
                }
            ) || matches!(storage, Storage::Filesystem))
        {
            return Err(Error::Config(
                "agent capture requires writable LocalTrusted native serving",
            ));
        }
        let mut profiles = BTreeSet::new();
        for harness in &self.harnesses {
            if !profiles.insert(harness.profile)
                || harness.product_version != harness.profile.product_version()
                || harness.roots.len() > 16
            {
                return Err(Error::Config(
                    "agent capture profile, product version or root bounds",
                ));
            }
            let mut roots = BTreeSet::new();
            for root in &harness.roots {
                if !root.is_absolute()
                    || root
                        .to_str()
                        .is_none_or(|s| s.len() > 4096 || s.contains('\0'))
                    || !roots.insert(root)
                {
                    return Err(Error::Config("agent capture approved roots"));
                }
            }
            if harness.profile == CaptureProfile::OpencodeReadOnlyApiV1 {
                if !harness.roots.is_empty()
                    || !harness.endpoint.as_deref().is_some_and(valid_endpoint)
                {
                    return Err(Error::Config(
                        "agent capture requires an explicit read-only loopback API",
                    ));
                }
            } else if harness.endpoint.is_some() {
                return Err(Error::Config(
                    "agent capture endpoint requires the read-only API profile",
                ));
            }
        }
        Ok(())
    }
}
fn valid_endpoint(endpoint: &str) -> bool {
    let Ok(url) = url::Url::parse(endpoint) else {
        return false;
    };
    matches!(url.scheme(), "http" | "https")
        && url.username().is_empty()
        && url.password().is_none()
        && url.query().is_none()
        && url.fragment().is_none()
        && url.path() == "/"
        && url.host_str().is_some_and(|host| {
            host.trim_matches(['[', ']'])
                .parse::<std::net::IpAddr>()
                .is_ok_and(|ip| ip.is_loopback())
        })
}
