use super::*;
use crate::native_dto::RemoteState;
use jj_lib::op_store as o;
use std::collections::{BTreeMap, HashSet};
impl TryFrom<&o::RefTarget> for GitRefTargetV1 {
    type Error = Error;
    fn try_from(r: &o::RefTarget) -> Result<Self> {
        Ok(Self(
            r.as_merge()
                .iter()
                .map(|id| id.as_ref().map(git_commit_id).transpose())
                .collect::<Result<_>>()?,
        ))
    }
}
impl TryFrom<GitRefTargetV1> for o::RefTarget {
    type Error = Error;
    fn try_from(r: GitRefTargetV1) -> Result<Self> {
        Ok(Self::from_merge(merge(
            r.0.into_iter()
                .map(|id| id.map(git_native_commit_id))
                .collect(),
        )?))
    }
}

impl TryFrom<&o::RemoteRef> for GitRemoteRefV1 {
    type Error = Error;
    fn try_from(r: &o::RemoteRef) -> Result<Self> {
        Ok(Self {
            target: (&r.target).try_into()?,
            state: match r.state {
                o::RemoteRefState::New => RemoteState::New,
                o::RemoteRefState::Tracked => RemoteState::Tracked,
            },
        })
    }
}
impl TryFrom<GitRemoteRefV1> for o::RemoteRef {
    type Error = Error;
    fn try_from(r: GitRemoteRefV1) -> Result<Self> {
        Ok(Self {
            target: r.target.try_into()?,
            state: match r.state {
                RemoteState::New => o::RemoteRefState::New,
                RemoteState::Tracked => o::RemoteRefState::Tracked,
            },
        })
    }
}

// Map keys are native string wrappers. Convert explicitly; ordering is preserved.
macro_rules! wire_map {
    ($map:expr, $convert:expr) => {
        $map.iter()
            .map(|(k, v)| Ok((k.as_str().to_owned(), ($convert)(v)?)))
            .collect::<Result<BTreeMap<_, _>>>()?
    };
}
macro_rules! native_map {
    ($map:expr, $convert:expr) => {
        $map.into_iter()
            .map(|(k, v)| Ok((k.into(), ($convert)(v)?)))
            .collect::<Result<BTreeMap<_, _>>>()?
    };
}
impl TryFrom<&o::View> for GitViewV1 {
    type Error = Error;
    fn try_from(v: &o::View) -> Result<Self> {
        let mut head_ids = v
            .head_ids
            .iter()
            .map(git_commit_id)
            .collect::<Result<Vec<_>>>()?;
        head_ids.sort();
        Ok(Self {
            head_ids,
            local_bookmarks: wire_map!(v.local_bookmarks, GitRefTargetV1::try_from),
            local_tags: wire_map!(v.local_tags, GitRefTargetV1::try_from),
            remote_views: v
                .remote_views
                .iter()
                .map(|(k, v)| {
                    Ok((
                        k.as_str().to_owned(),
                        GitRemoteViewV1 {
                            bookmarks: wire_map!(v.bookmarks, GitRemoteRefV1::try_from),
                            tags: wire_map!(v.tags, GitRemoteRefV1::try_from),
                        },
                    ))
                })
                .collect::<Result<_>>()?,
            git_refs: wire_map!(v.git_refs, GitRefTargetV1::try_from),
            git_heads: wire_map!(v.git_heads, GitRefTargetV1::try_from),
            wc_commit_ids: wire_map!(v.wc_commit_ids, git_commit_id),
        })
    }
}
impl TryFrom<GitViewV1> for o::View {
    type Error = Error;
    fn try_from(v: GitViewV1) -> Result<Self> {
        sorted_unique(&v.head_ids)?;
        if v.head_ids.is_empty() {
            return Err(Error::Invalid("view must have heads"));
        }
        Ok(Self {
            head_ids: v
                .head_ids
                .into_iter()
                .map(git_native_commit_id)
                .collect::<HashSet<_>>(),
            local_bookmarks: native_map!(v.local_bookmarks, o::RefTarget::try_from),
            local_tags: native_map!(v.local_tags, o::RefTarget::try_from),
            remote_views: v
                .remote_views
                .into_iter()
                .map(|(k, v)| {
                    Ok((
                        k.into(),
                        o::RemoteView {
                            bookmarks: native_map!(v.bookmarks, o::RemoteRef::try_from),
                            tags: native_map!(v.tags, o::RemoteRef::try_from),
                        },
                    ))
                })
                .collect::<Result<_>>()?,
            git_refs: native_map!(v.git_refs, o::RefTarget::try_from),
            git_heads: native_map!(v.git_heads, o::RefTarget::try_from),
            wc_commit_ids: v
                .wc_commit_ids
                .into_iter()
                .map(|(k, v)| (k.into(), git_native_commit_id(v)))
                .collect(),
        })
    }
}
