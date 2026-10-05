use super::*;
use jj_store_jj::local_browser::{self, Access, Error};
use std::{
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

struct AfterWrite {
    base: Arc<dyn Access>,
    root: PathBuf,
    replace: bool,
    fired: AtomicBool,
}
#[async_trait::async_trait]
impl Access for AfterWrite {
    fn repository(&self) -> m::RepositoryId {
        self.base.repository()
    }
    fn generation(&self) -> u64 {
        self.base.generation()
    }
    fn metadata_path(&self) -> &Path {
        self.base.metadata_path()
    }
    async fn authorize(&self) -> local_browser::Result<()> {
        self.base.authorize().await?;
        let index = self.root.join(".jj/repo/index");
        let selected = fs::read_dir(index.join("segments"))
            .ok()
            .is_some_and(|mut entries| {
                entries.any(|e| e.unwrap().file_name().as_encoded_bytes().len() == 128)
            });
        if selected && !self.fired.swap(true, Ordering::SeqCst) {
            if !self.replace {
                return Err(Error::Unavailable);
            }
            fs::rename(&index, self.root.join(".jj/repo/saved-index")).unwrap();
            fs::create_dir(index).unwrap();
        }
        Ok(())
    }
}
#[tokio::test]
async fn index_directory_replacement_and_authorization_loss_stop_before_association_selection() {
    for replace in [false, true] {
        let f = Fixture::new().await;
        let (witness, key) = prepare(&f).await;
        let before = f.state();
        fs::remove_dir_all(f.root().join(".jj/repo/index")).unwrap();
        let access = Arc::new(AfterWrite {
            base: f.access(),
            root: f.root(),
            replace,
            fired: AtomicBool::new(false),
        });
        let recovery = Coordinator::new(1)
            .unwrap()
            .restore_history(access, f.root(), witness.clone(), key.clone())
            .await
            .unwrap();
        assert!(recovery.restore_indexes().await.is_err());
        assert_eq!(f.state(), before);
        let partial = if replace {
            assert!(
                fs::read_dir(f.root().join(".jj/repo/index"))
                    .unwrap()
                    .next()
                    .is_none()
            );
            let path = f.root().join(".jj/repo/saved-index");
            assert!(
                fs::read_dir(path.join("op_links"))
                    .unwrap()
                    .next()
                    .is_none()
            );
            fs::read_dir(path.join("segments"))
                .unwrap()
                .map(|e| {
                    let e = e.unwrap();
                    (
                        format!("segments/{}", e.file_name().to_str().unwrap()),
                        fs::read(e.path()).unwrap(),
                    )
                })
                .collect::<Vec<_>>()
        } else {
            indexes::index_state(&f.root())
        };
        assert!(!partial.is_empty());
        assert!(partial.iter().all(|(n, _)| !n.starts_with("op_links/")));
        drop(
            Coordinator::new(1)
                .unwrap()
                .restore_history(f.access(), f.root(), witness, key)
                .await
                .unwrap()
                .restore_indexes()
                .await
                .unwrap(),
        );
        let complete = indexes::index_state(&f.root());
        for entry in partial {
            assert!(complete.contains(&entry));
        }
        assert_eq!(f.state(), before);
    }
}

#[tokio::test]
async fn unsafe_index_directories_files_and_oversized_segments_fail_without_touching_targets() {
    let f = Fixture::new().await;
    let (witness, key) = prepare(&f).await;
    fs::remove_dir_all(f.root().join(".jj/repo/index")).unwrap();
    let before = f.state();
    let index = f.root().join(".jj/repo/index");
    fs::create_dir_all(index.join("segments")).unwrap();
    fs::create_dir(index.join("op_links")).unwrap();
    fs::write(index.join("type"), "default").unwrap();
    let outside = tempfile::tempdir().unwrap();
    let target = outside.path().join("target");
    fs::write(&target, "untouched outside index").unwrap();
    for path in [
        index.join("segments").join("f".repeat(128)),
        index.join("op_links").join("f".repeat(128)),
    ] {
        for hard in [false, true] {
            if hard {
                fs::hard_link(&target, &path).unwrap();
            } else {
                std::os::unix::fs::symlink(&target, &path).unwrap();
            }
            let recovery = Coordinator::new(1)
                .unwrap()
                .restore_history(f.access(), f.root(), witness.clone(), key.clone())
                .await
                .unwrap();
            assert!(recovery.restore_indexes().await.is_err());
            assert_eq!(fs::read(&target).unwrap(), b"untouched outside index");
            fs::remove_file(&path).unwrap();
        }
    }
    let oversized = index.join("segments").join("e".repeat(128));
    let file = fs::File::create(&oversized).unwrap();
    file.set_len(4 * 1024 * 1024 + 1).unwrap();
    let recovery = Coordinator::new(1)
        .unwrap()
        .restore_history(f.access(), f.root(), witness.clone(), key.clone())
        .await
        .unwrap();
    assert!(matches!(
        recovery.restore_indexes().await,
        Err(Error::Limit)
    ));
    assert_eq!(file.metadata().unwrap().len(), 4 * 1024 * 1024 + 1);
    fs::remove_file(oversized).unwrap();
    for path in [index.join("segments"), index.clone()] {
        let saved = outside.path().join("saved");
        fs::rename(&path, &saved).unwrap();
        std::os::unix::fs::symlink(outside.path(), &path).unwrap();
        let recovery = Coordinator::new(1)
            .unwrap()
            .restore_history(f.access(), f.root(), witness.clone(), key.clone())
            .await
            .unwrap();
        assert!(recovery.restore_indexes().await.is_err());
        fs::remove_file(&path).unwrap();
        fs::rename(saved, path).unwrap();
    }
    assert_eq!(fs::read(&target).unwrap(), b"untouched outside index");
    assert_eq!(f.state(), before);
    assert!(
        fs::read_dir(index.join("op_links"))
            .unwrap()
            .next()
            .is_none()
    );
    assert!(
        fs::read_dir(index.join("segments"))
            .unwrap()
            .next()
            .is_none()
    );
}
