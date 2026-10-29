use super::*;
use crate::{
    backend::{BackendLimits, NativeBackend},
    query::access::ReadOnlyAuthority,
};
use jj_lib::{
    config::StackedConfig, repo_path::RepoPathBuf, settings::UserSettings, signing::Signer,
    store::Store, tree_merge::MergeOptions,
};

pub(super) fn path(value: &str) -> Result<RepoPathBuf> {
    if value.len() > 4096 {
        return Err(Error::Limit);
    }
    let path = RepoPathBuf::from_internal_string(value.to_owned()).map_err(|_| Error::Invalid)?;
    if value.contains('\0') || value.split('/').any(|name| matches!(name, "." | "..")) {
        return Err(Error::Invalid);
    }
    if path.components().count() > 64 {
        return Err(Error::Limit);
    }
    Ok(path)
}
pub(super) fn terms(value: &b::MergedTreeValue) -> Result<Vec<Option<dto::TreeValue>>> {
    value
        .iter()
        .map(|value| {
            value
                .as_ref()
                .map(dto::TreeValue::try_from)
                .transpose()
                .map_err(|_| Error::Corrupt)
        })
        .collect()
}
pub(super) fn map_backend(error: b::BackendError) -> Error {
    match error {
        b::BackendError::ObjectNotFound { .. } => Error::Incomplete,
        b::BackendError::ReadObject { source, .. } => source
            .downcast::<objects::Error>()
            .map(|e| map_object(*e))
            .unwrap_or(Error::Unavailable),
        _ => Error::Unavailable,
    }
}
impl Context {
    pub(super) fn native_store(&self) -> Result<Arc<Store>> {
        let authority = Arc::new(ReadOnlyAuthority {
            access: self.access.clone(),
            budget: self.budget.clone(),
        });
        let budget = self.budget.clone();
        let files = crate::query::code_index::merge_files::Files::new(
            crate::query::code_index::merge_files::Hash::Native,
            self.limits
                .max_blob_bytes
                .min(BackendLimits::default().max_bytes as u64) as usize,
            BackendLimits::default().max_bytes,
            move |bytes| {
                budget.work(1)?;
                budget.read_bytes(bytes)
            },
        );
        let backend = NativeBackend::new(self.objects.clone(), authority, BackendLimits::default())
            .map_err(|_| Error::Unavailable)?
            .with_derived_files(Some(files));
        let settings = UserSettings::from_config_and_home_dir(StackedConfig::with_defaults(), None)
            .map_err(|_| Error::Unavailable)?;
        Ok(Store::new(
            Box::new(backend),
            Signer::new(None, vec![]),
            MergeOptions::from_settings(&settings).map_err(|_| Error::Unavailable)?,
        ))
    }
    pub(super) async fn commit(
        &self,
        store: &Arc<Store>,
        id: m::CommitId,
    ) -> Result<jj_lib::commit::Commit> {
        self.member(m::ObjectKey::Commit(id)).await?;
        store
            .get_commit_async(&b::CommitId::from_bytes(id.as_bytes()))
            .await
            .map_err(map_backend)
    }
    pub(super) async fn text(&self, id: m::FileId) -> Result<Text> {
        self.member(m::ObjectKey::File(id)).await?;
        let spool = self.objects.get_file(id).await.map_err(map_object)?;
        if spool.len() > self.limits.max_text_bytes as u64 {
            return Ok(Text::TooLarge);
        }
        let mut bytes = Vec::with_capacity(spool.len() as usize);
        spool
            .reader()
            .map_err(|_| Error::Unavailable)?
            .read_to_end(&mut bytes)
            .await
            .map_err(|_| Error::Unavailable)?;
        self.budget.authorize(self.access.as_ref()).await?;
        if bytes.contains(&0) {
            return Ok(Text::Binary);
        }
        Ok(match String::from_utf8(bytes) {
            Ok(text) => Text::Utf8(text),
            Err(_) => Text::Binary,
        })
    }
}
pub(super) enum Text {
    Utf8(String),
    Binary,
    TooLarge,
}
