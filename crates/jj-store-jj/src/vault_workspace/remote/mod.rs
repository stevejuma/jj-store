//! Native-approved Git transfer. Network calls never hold the capture lease.
mod import;
mod process;
pub(super) mod publication;
use super::*;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use jj_lib::git;
pub use publication::{PendingPublication, PushIntent, PushOutcome};
use url::Url;
use zeroize::Zeroizing;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RemoteTarget {
    pub url: String,
    pub branch: String,
}
impl RemoteTarget {
    pub fn validate(&self) -> Result<()> {
        checked_branch(&self.branch)?;
        let url = Url::parse(&self.url).map_err(|_| Error::Binding)?;
        if url.scheme() != "https"
            || url.host_str().is_none()
            || !url.username().is_empty()
            || url.password().is_some()
            || url.query().is_some()
            || url.fragment().is_some()
            || url.path().trim_matches('/').is_empty()
            || self.url.len() > 4096
            || self.url.chars().any(char::is_control)
        {
            return Err(Error::Binding);
        }
        Ok(())
    }
}
fn checked_branch(branch: &str) -> Result<()> {
    if branch.is_empty()
        || branch.len() > 256
        || !branch
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._-/".contains(&b))
        || branch.starts_with('-')
        || branch.contains("..")
        || branch
            .split('/')
            .any(|p| p.is_empty() || p.starts_with('.') || p.ends_with('.') || p.ends_with(".lock"))
    {
        return Err(Error::Binding);
    }
    Ok(())
}
/// Intentionally neither serializable nor printable.
pub struct GitCredentials(Zeroizing<String>);
impl GitCredentials {
    pub fn token(token: &str) -> Result<Self> {
        if token.is_empty() || token.len() > 4096 || token.chars().any(char::is_control) {
            return Err(Error::Binding);
        }
        let plain = Zeroizing::new(format!("x-access-token:{token}"));
        Ok(Self(Zeroizing::new(format!(
            "Authorization: Basic {}",
            STANDARD.encode(plain.as_bytes())
        ))))
    }
}
/// Opaque proof of a transfer into this admitted native binding.
pub struct FetchedHead {
    binding: Binding,
    endpoint: String,
    branch: String,
    commit: String,
}
impl FetchedHead {
    pub fn commit(&self) -> &str {
        &self.commit
    }
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }
}
pub struct RemoteTransfer {
    binding: Binding,
    endpoint: String,
    branch: String,
    git: PathBuf,
    credential: Option<GitCredentials>,
    local: bool,
}
impl RemoteTransfer {
    pub async fn connect(
        binding: Binding,
        target: RemoteTarget,
        credential: GitCredentials,
    ) -> Result<Self> {
        target.validate()?;
        Self::initialize(binding, target.url, target.branch, Some(credential), false).await
    }
    /// Explicit native-only local transport for admitted local backups and tests.
    /// Never deserialize a renderer path into this capability.
    pub async fn connect_local(binding: Binding, path: &Path, branch: &str) -> Result<Self> {
        checked_branch(branch)?;
        let path = path.canonicalize()?;
        let text = path.to_str().ok_or(Error::Binding)?.to_owned();
        Self::initialize(binding, text, branch.into(), None, true).await
    }
    async fn initialize(
        binding: Binding,
        endpoint: String,
        branch: String,
        credential: Option<GitCredentials>,
        local: bool,
    ) -> Result<Self> {
        let session = Session::open(binding.clone()).await?;
        let git = git::get_git_backend(session.repo.store())
            .map_err(jj)?
            .git_repo_path()
            .to_owned();
        drop(session);
        Ok(Self {
            binding,
            endpoint,
            branch,
            git,
            credential,
            local,
        })
    }
    fn reference(&self) -> String {
        format!("refs/heads/{}", self.branch)
    }
    pub async fn observe(&self) -> Result<Option<String>> {
        let reference = self.reference();
        let (status, text) = self
            .run(&[
                "ls-remote",
                "--exit-code",
                "--heads",
                "--",
                &self.endpoint,
                &reference,
            ])
            .await?;
        if status == Some(2) && text.is_empty() {
            return Ok(None);
        }
        if status != Some(0) {
            return Err(Error::Jj("Remote branch observation failed".into()));
        }
        let (commit, observed) = text.trim_end().split_once('\t').ok_or(Error::Binding)?;
        if observed != reference || !oid(commit) {
            return Err(Error::Binding);
        }
        Ok(Some(commit.into()))
    }
    pub async fn fetch(&self) -> Result<Option<FetchedHead>> {
        let Some(commit) = self.observe().await? else {
            return Ok(None);
        };
        let destination = format!("+{commit}:refs/remotes/lapis-backup/{}", self.branch);
        let (status, _) = self
            .run(&[
                "fetch",
                "--no-tags",
                "--no-recurse-submodules",
                "--no-write-fetch-head",
                "--",
                &self.endpoint,
                &destination,
            ])
            .await?;
        if status != Some(0) {
            return Err(Error::Jj("Remote backup fetch failed".into()));
        }
        Ok(Some(FetchedHead {
            binding: self.binding.clone(),
            endpoint: self.endpoint.clone(),
            branch: self.branch.clone(),
            commit,
        }))
    }
}
fn oid(value: &str) -> bool {
    value.len() == 40
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
