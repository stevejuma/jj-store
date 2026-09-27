//! Lossless native conflict descriptors. Fingerprints never grant authority.
use super::native::{Text, map_backend};
use super::*;
use blake2::{Blake2b512, Digest as _};
use jj_lib::{matchers::PrefixMatcher, object_id::ObjectId as _};
use serde::Deserialize;

#[derive(Clone, Debug, Serialize, Deserialize, schemars::JsonSchema)]
#[schemars(rename = "ConflictRequest")]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub commit: m::CommitId,
    pub path: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "ConflictRole")]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Add,
    Remove,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "ConflictTerm")]
pub struct Term {
    pub id: u32,
    pub role: Role,
    pub label: Option<String>,
    pub value: Option<dto::TreeValue>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "ConflictUnsupported")]
#[serde(rename_all = "snake_case")]
pub enum Unsupported {
    MultipleSides,
    DeleteModify,
    Structural,
    ExecutableMode,
    CopyHistory,
    Binary,
    TooLarge,
    InvalidPath,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "ConflictEditor")]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Editor {
    Text {
        contents: [String; 3],
        executable: bool,
    },
    Unsupported {
        reason: Unsupported,
    },
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "ConflictDescriptor")]
pub struct Descriptor {
    pub path: String,
    pub fingerprint: m::ConflictFingerprint,
    pub terms: Vec<Term>,
    pub editor: Editor,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, schemars::JsonSchema)]
#[schemars(rename = "ConflictResponse")]
pub struct Response {
    pub version: u32,
    pub commit: m::CommitId,
    pub change: m::ChangeId,
    pub conflicts: Vec<Descriptor>,
    pub stats: Stats,
}
/// Recomputed from checked native values during mutation validation. This is an
/// integrity binding, not a signed token or a substitute for authorization.
pub fn fingerprint(
    repository: m::RepositoryId,
    epoch: m::EpochId,
    commit: m::CommitId,
    change: m::ChangeId,
    path: &str,
    terms: &[Term],
) -> Result<m::ConflictFingerprint> {
    #[derive(Serialize)]
    struct Input<'a> {
        version: u32,
        profile: &'a str,
        repository: m::RepositoryId,
        epoch: m::EpochId,
        commit: m::CommitId,
        change: m::ChangeId,
        path: &'a str,
        terms: &'a [Term],
    }
    let input = Input {
        version: 1,
        profile: crate::factories::PROFILE,
        repository,
        epoch,
        commit,
        change,
        path,
        terms,
    };
    let mut hash = Blake2b512::new();
    hash.update(b"jj-store/conflict-fingerprint/v1\0");
    hash.update(serde_json::to_vec(&input).map_err(|_| Error::Unavailable)?);
    Ok(m::ConflictFingerprint::new(hash.finalize().into()))
}
/// Shared by the read descriptor and atomic mutation validation. Never simplify
/// or renumber terms before fingerprinting and selecting a native term.
pub(crate) fn native_terms(
    tree: &jj_lib::merged_tree::MergedTree,
    value: &b::MergedTreeValue,
) -> Result<Vec<Term>> {
    let values = native::terms(value)?;
    let labels = tree.labels_by_term("");
    if values.len() != labels.iter().len() {
        return Err(Error::Corrupt);
    }
    values
        .into_iter()
        .zip(labels.iter())
        .enumerate()
        .map(|(i, (value, label))| {
            Ok(Term {
                id: i.try_into().map_err(|_| Error::Limit)?,
                role: if i % 2 == 0 { Role::Add } else { Role::Remove },
                value,
                label: tree.labels().has_labels().then(|| (*label).to_owned()),
            })
        })
        .collect()
}
pub(super) async fn read(context: Context, request: Request) -> Result<Response> {
    let path = native::path(&request.path)?;
    let store = context.native_store()?;
    let commit = context.commit(&store, request.commit).await?;
    let change =
        m::ChangeId::from_slice(commit.change_id().as_bytes()).map_err(|_| Error::Corrupt)?;
    let tree = commit.tree();
    let matcher = PrefixMatcher::new([path.as_ref()]);
    let mut conflicts = vec![];
    for (path, value) in tree.conflicts_matching(&matcher) {
        context.budget.authorize(context.access.as_ref()).await?;
        if conflicts.len() >= context.limits.max_changes {
            return Err(Error::Limit);
        }
        let value = value.map_err(map_backend)?;
        let terms = native_terms(&tree, &value)?;
        for term in &terms {
            context.budget.work(1)?;
            context.budget.output(term)?;
        }
        let path = path.as_internal_file_string().to_owned();
        let fingerprint = fingerprint(
            context.access.repository(),
            context.access.epoch(),
            request.commit,
            change,
            &path,
            &terms,
        )?;
        let editor = context.conflict_editor(&path, &terms).await?;
        conflicts.push(Descriptor {
            path,
            fingerprint,
            terms,
            editor,
        });
    }
    let result = Response {
        version: 1,
        commit: request.commit,
        change,
        conflicts,
        stats: context.budget.stats(),
    };
    context.budget.complete_output(&result)?;
    Ok(result)
}
fn unsupported(reason: Unsupported) -> Editor {
    Editor::Unsupported { reason }
}
impl Context {
    async fn conflict_editor(&self, path: &str, terms: &[Term]) -> Result<Editor> {
        if native::path(path).is_err() {
            return Ok(unsupported(Unsupported::InvalidPath));
        }
        if terms.len() != 3 {
            return Ok(unsupported(Unsupported::MultipleSides));
        }
        let mut files = vec![];
        for term in terms {
            match &term.value {
                Some(dto::TreeValue::File {
                    id,
                    executable,
                    copy,
                }) => files.push((*id, *executable, *copy)),
                None => return Ok(unsupported(Unsupported::DeleteModify)),
                _ => return Ok(unsupported(Unsupported::Structural)),
            }
        }
        if files.iter().any(|file| file.1 != files[0].1) {
            return Ok(unsupported(Unsupported::ExecutableMode));
        }
        if files.iter().any(|file| file.2 != files[0].2) {
            return Ok(unsupported(Unsupported::CopyHistory));
        }
        let mut contents = vec![];
        let mut bytes = 0usize;
        for (id, _, _) in &files {
            let text = match self.text(*id).await? {
                Text::Utf8(text) => text,
                Text::Binary => return Ok(unsupported(Unsupported::Binary)),
                Text::TooLarge => return Ok(unsupported(Unsupported::TooLarge)),
            };
            bytes = bytes.checked_add(text.len()).ok_or(Error::Limit)?;
            if bytes > self.limits.max_text_bytes {
                return Ok(unsupported(Unsupported::TooLarge));
            }
            self.budget.output(&text)?;
            contents.push(text);
        }
        Ok(Editor::Text {
            contents: contents.try_into().map_err(|_| Error::Unavailable)?,
            executable: files[0].1,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn conflict_fingerprint_v1_matches_independent_canonical_vector() {
        let terms = vec![
            Term {
                id: 0,
                role: Role::Add,
                label: Some("left".into()),
                value: Some(dto::TreeValue::File {
                    id: m::FileId::new([7; 64]),
                    executable: false,
                    copy: m::CopyReference::Placeholder,
                }),
            },
            Term {
                id: 1,
                role: Role::Remove,
                label: None,
                value: None,
            },
            Term {
                id: 2,
                role: Role::Add,
                label: Some("右".into()),
                value: Some(dto::TreeValue::File {
                    id: m::FileId::new([8; 64]),
                    executable: true,
                    copy: m::CopyReference::Object(m::CopyId::new([9; 64])),
                }),
            },
        ];
        let value = fingerprint(
            m::RepositoryId::new([1; 16]),
            m::EpochId::new([2; 16]),
            m::CommitId::new([3; 64]),
            m::ChangeId::new([4; 16]),
            "src/file",
            &terms,
        )
        .unwrap();
        // Python hashlib.blake2b over the independently specified canonical V1 JSON.
        assert_eq!(
            value.to_string(),
            "8ccc9eb4ea78eb3f821ae157c9a72df6aacda401322aa66be5220ee4e0f455b13dcaf9c247658ec326b74f2a7b4d3937f50bb5471ae7538e27729ecb481f53ec"
        );
        assert!(serde_json::from_str::<m::ConflictFingerprint>("\"ABC\"").is_err());
    }
}
