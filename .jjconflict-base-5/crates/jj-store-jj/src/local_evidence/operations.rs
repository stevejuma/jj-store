use super::*;
use crate::native_dto::Metadata;
use jj_lib::op_store as o;
impl TryFrom<&o::Operation> for GitOperationV1 {
    type Error = Error;
    fn try_from(op: &o::Operation) -> Result<Self> {
        let m = &op.metadata;
        Ok(Self {
            view_id: view_id(&op.view_id)?,
            parents: op.parents.iter().map(operation_id).collect::<Result<_>>()?,
            metadata: Metadata {
                start: m.time.start.into(),
                end: m.time.end.into(),
                description: m.description.clone(),
                hostname: m.hostname.clone(),
                username: m.username.clone(),
                is_snapshot: m.is_snapshot,
                workspace_name: m.workspace_name.as_ref().map(|n| n.as_str().to_owned()),
                attributes: m.attributes.clone(),
            },
            commit_predecessors: op
                .commit_predecessors
                .as_ref()
                .map(|map| {
                    map.iter()
                        .map(|(k, v)| {
                            Ok((
                                git_commit_id(k)?,
                                v.iter().map(git_commit_id).collect::<Result<_>>()?,
                            ))
                        })
                        .collect::<Result<_>>()
                })
                .transpose()?,
        })
    }
}
impl TryFrom<GitOperationV1> for o::Operation {
    type Error = Error;
    fn try_from(op: GitOperationV1) -> Result<Self> {
        let m = op.metadata;
        Ok(Self {
            view_id: native_view_id(op.view_id),
            parents: op.parents.into_iter().map(native_operation_id).collect(),
            metadata: o::OperationMetadata {
                time: o::TimestampRange {
                    start: m.start.into(),
                    end: m.end.into(),
                },
                description: m.description,
                hostname: m.hostname,
                username: m.username,
                is_snapshot: m.is_snapshot,
                workspace_name: m.workspace_name.map(Into::into),
                attributes: m.attributes,
            },
            commit_predecessors: op.commit_predecessors.map(|map| {
                map.into_iter()
                    .map(|(k, v)| {
                        (
                            git_native_commit_id(k),
                            v.into_iter().map(git_native_commit_id).collect(),
                        )
                    })
                    .collect()
            }),
        })
    }
}
