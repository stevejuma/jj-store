//! Feasibility only: no production route or unbounded reader is exposed.
use futures::{StreamExt as _, TryStreamExt as _};
use jj_lib::{
    annotate::FileAnnotator,
    backend as b,
    commit::Commit,
    config::StackedConfig,
    merged_tree::MergedTree,
    repo::{ReadonlyRepo, Repo},
    repo_path::{RepoPath, RepoPathComponentBuf},
    revset::{RevsetEvaluationError, RevsetExpression},
    settings::UserSettings,
    simple_backend::SimpleBackend,
    workspace::Workspace,
};
use std::sync::Arc;

/// The proposed bounded domain: consecutive topological ancestors of all pending
/// origins. Keep the domain construction separate from compute for measurement.
pub(crate) async fn domain(
    repo: &dyn Repo,
    annotator: &FileAnnotator,
    limit: usize,
) -> Result<Vec<b::CommitId>, RevsetEvaluationError> {
    RevsetExpression::commits(annotator.pending_commits().cloned().collect())
        .ancestors()
        .evaluate(repo)?
        .stream()
        .take(limit)
        .try_collect()
        .await
}

fn origins(annotator: &FileAnnotator) -> Vec<Option<(b::CommitId, usize)>> {
    annotator
        .to_annotation()
        .line_origins()
        .map(|(origin, _)| origin.ok().map(|o| (o.commit_id.clone(), o.line_number)))
        .collect()
}

pub(crate) async fn commit(
    repo: &Arc<ReadonlyRepo>,
    parents: Vec<b::CommitId>,
    text: &[u8],
) -> (Arc<ReadonlyRepo>, Commit) {
    let file = repo
        .store()
        .write_file(
            RepoPath::from_internal_string("file").unwrap(),
            &mut futures::io::Cursor::new(text),
        )
        .await
        .unwrap();
    let tree = b::Tree::from_sorted_entries(vec![(
        RepoPathComponentBuf::new("file").unwrap(),
        b::TreeValue::File {
            id: file,
            executable: false,
            copy_id: b::CopyId::placeholder(),
        },
    )]);
    let tree = repo
        .store()
        .write_tree(RepoPath::root(), tree)
        .await
        .unwrap();
    let mut tx = repo.start_transaction();
    let commit = tx
        .repo_mut()
        .new_commit(
            parents,
            MergedTree::resolved(repo.store().clone(), tree.id().clone()),
        )
        .write()
        .await
        .unwrap();
    let repo = tx.commit("blame feasibility fixture").await.unwrap();
    (repo, commit)
}

#[tokio::test]
async fn incremental_domains_match_complete_annotation_for_edits_and_merges() {
    // Exercise both native-width and Git-width IDs without adding service APIs.
    for git in [false, true] {
        let directory = tempfile::tempdir().unwrap();
        let settings =
            UserSettings::from_config_and_home_dir(StackedConfig::with_defaults(), None).unwrap();
        let (_workspace, mut repo) = if git {
            Workspace::init_internal_git(&settings, directory.path(), gix::hash::Kind::Sha1)
                .await
                .unwrap()
        } else {
            Workspace::init_with_backend(
                &settings,
                directory.path(),
                &|_, path| Ok(Box::new(SimpleBackend::init(path))),
                jj_lib::signing_factory::signer_from_settings(&settings).unwrap(),
            )
            .await
            .unwrap()
        };
        let root = repo.store().root_commit_id().clone();
        let (next, base) = commit(&repo, vec![root], b"base\nrepeat\nrepeat\nlast").await;
        repo = next;
        let (next, left) = commit(
            &repo,
            vec![base.id().clone()],
            b"left\nrepeat\nrepeat\nlast",
        )
        .await;
        repo = next;
        let (next, right) = commit(
            &repo,
            vec![base.id().clone()],
            b"base\nrepeat\nrepeat\nright",
        )
        .await;
        repo = next;
        let (next, merge) = commit(
            &repo,
            vec![left.id().clone(), right.id().clone()],
            b"left\nrepeat\nrepeat\nright",
        )
        .await;
        repo = next;
        let (next, empty) = commit(&repo, vec![merge.id().clone()], b"").await;
        repo = next;
        for selected in [base, left, right, merge, empty] {
            let path = RepoPath::from_internal_string("file").unwrap();
            let mut full = FileAnnotator::from_commit(&selected, path).await.unwrap();
            full.compute(repo.as_ref(), &RevsetExpression::all())
                .await
                .unwrap();
            for size in [1, 2, 3, 64] {
                let mut partial = FileAnnotator::from_commit(&selected, path).await.unwrap();
                for _ in 0..20 {
                    if partial.pending_commits().next().is_none() {
                        break;
                    }
                    let ids = domain(repo.as_ref(), &partial, size).await.unwrap();
                    partial
                        .compute(repo.as_ref(), &RevsetExpression::commits(ids))
                        .await
                        .unwrap();
                    for (actual, expected) in origins(&partial).iter().zip(origins(&full)) {
                        if let Some(actual) = actual {
                            assert_eq!(
                                Some(actual.clone()),
                                expected,
                                "premature origin at batch size {size}, git={git}"
                            );
                        }
                    }
                }
                assert_eq!(
                    origins(&partial),
                    origins(&full),
                    "incomplete or incorrect batched annotation, size={size}, git={git}"
                );
                assert_eq!(partial.pending_commits().count(), 0);
            }
        }
    }
}

/// Admission/cancellation probe for a legal text preview, not a throughput test.
/// Reverse-ordered unique lines exercise JJ's synchronous LCS comparison.
#[tokio::test]
#[ignore = "explicit synchronous comparison feasibility measurement"]
async fn synchronous_comparison_honors_progress_deadline() {
    let directory = tempfile::tempdir().unwrap();
    let settings =
        UserSettings::from_config_and_home_dir(StackedConfig::with_defaults(), None).unwrap();
    let (_workspace, repo) = Workspace::init_with_backend(
        &settings,
        directory.path(),
        &|_, path| Ok(Box::new(SimpleBackend::init(path))),
        jj_lib::signing_factory::signer_from_settings(&settings).unwrap(),
    )
    .await
    .unwrap();
    let old = (0..40_000).map(|n| format!("{n:05}\n")).collect::<String>();
    let new = (0..40_000)
        .rev()
        .map(|n| format!("{n:05}\n"))
        .collect::<String>();
    assert!(old.len() < super::content::Limits::default().max_text_bytes);
    let root = repo.store().root_commit_id().clone();
    let (repo, before) = commit(&repo, vec![root], old.as_bytes()).await;
    let (repo, after) = commit(&repo, vec![before.id().clone()], new.as_bytes()).await;
    let mut annotator =
        FileAnnotator::from_commit(&after, RepoPath::from_internal_string("file").unwrap())
            .await
            .unwrap();
    let start = std::time::Instant::now();
    let result = tokio::time::timeout(
        std::time::Duration::from_secs(1),
        annotator.compute(
            repo.as_ref(),
            &RevsetExpression::commits(vec![after.id().clone()]),
        ),
    )
    .await;
    let elapsed = start.elapsed();
    eprintln!(
        "blame synchronous comparison: bytes={}, lines=40000, domain=1, timeout=1s, elapsed={elapsed:?}, result={result:?}",
        old.len()
    );
    assert!(
        elapsed < std::time::Duration::from_secs(5),
        "one legal file comparison escaped the cooperative deadline and warm progress target"
    );
}
