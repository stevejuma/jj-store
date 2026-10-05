//! Installation-owned automatic capture actor; directory admission is always current.
use crate::{
    Error, Result,
    capture_admission::CaptureAdmission,
    config::{Access, Config},
    installation::Identity,
};
use jj_store_model::PrincipalId;
use jj_store_service::{
    administration::RepositoryHost, auth::Permission, local_management::VerifiedCaptureWorkspace,
    transport::NativeRequest,
};

mod publisher;
pub use publisher::GuardedCapturePublisher;

pub struct CaptureAuthority {
    principal: PrincipalId,
}
impl CaptureAuthority {
    /// Read-only preparation; disabled configuration does not read identity state.
    pub fn prepare(config: &Config) -> Result<Option<Self>> {
        config.validate()?;
        if !config.agent_capture.enabled {
            return Ok(None);
        }
        let Access::LocalTrusted {
            installation,
            permission,
            ..
        } = &config.access
        else {
            return Err(Error::Config(
                "automatic capture requires LocalTrusted authority",
            ));
        };
        if !matches!(permission, Permission::Write | Permission::Admin) {
            return Err(Error::Config("automatic capture requires write authority"));
        }
        let identity = Identity::load(installation)?;
        Ok(Some(Self::from_identity(&identity)))
    }
    pub(crate) fn from_identity(identity: &Identity) -> Self {
        Self {
            principal: identity.principal(b"agent-capture-worker"),
        }
    }
    /// Every action obtains fresh current directory admission and native Write authority.
    pub fn publisher<'a>(
        &'a self,
        host: &'a RepositoryHost,
        admission: &'a CaptureAdmission,
        proof: &'a VerifiedCaptureWorkspace,
    ) -> GuardedCapturePublisher<'a> {
        GuardedCapturePublisher::new(self, host, admission, proof)
    }
    pub fn principal(&self) -> PrincipalId {
        self.principal
    }
    /// This is native write authorization, not lasting directory permission.
    /// The worker must revalidate admission before each publication.
    pub async fn bind(
        &self,
        host: &RepositoryHost,
        admission: &CaptureAdmission,
        proof: &VerifiedCaptureWorkspace,
    ) -> Result<NativeRequest> {
        admission
            .revalidate(proof)
            .await
            .map_err(|_| Error::Service("capture admission unavailable"))?;
        let request = host
            .endpoint(proof.workspace.repository)
            .and_then(|endpoint| endpoint.bind_local_trusted(self.principal, Permission::Write))
            .map_err(|_| Error::Service("capture native binding unavailable"))?;
        request
            .authorize(true)
            .await
            .map_err(|_| Error::Service("capture write authorization unavailable"))?;
        admission
            .revalidate(proof)
            .await
            .map_err(|_| Error::Service("capture admission changed"))?;
        Ok(request)
    }
}
