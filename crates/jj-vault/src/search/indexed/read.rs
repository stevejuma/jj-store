//! Admit a committed projection snapshot without initializing or repairing it.
use super::{Engine, State, disk::schema, error};
use std::{
    fs::{File, OpenOptions, TryLockError},
    path::Path,
    time::{Duration, Instant},
};
use tantivy::{Index, ReloadPolicy};

fn admit(root: &Path) -> Result<File, String> {
    let owner = OpenOptions::new()
        .read(true)
        .write(true)
        .open(root.join("owner.lock"))
        .map_err(error)?;
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        match owner.try_lock() {
            Ok(()) => return Ok(owner),
            Err(TryLockError::WouldBlock) if Instant::now() < deadline => {
                std::thread::sleep(Duration::from_millis(10));
            }
            Err(TryLockError::WouldBlock) => return Err("History index is busy".into()),
            Err(TryLockError::Error(failure)) => return Err(error(failure)),
        }
    }
}

impl Engine {
    pub(super) fn read(root: &Path, namespace: &str) -> Result<Self, String> {
        let owner = admit(root)?;
        let (definition, fields) = schema();
        let index = Index::open_in_dir(root.join("index")).map_err(error)?;
        if index.schema() != definition {
            return Err("History index schema is unavailable".into());
        }
        let state = match index.load_metas().map_err(error)?.payload {
            Some(payload) => serde_json::from_str::<State>(&payload)
                .map_err(error)
                .and_then(|state| {
                    if state.namespace != namespace {
                        return Err("History index namespace changed".into());
                    }
                    Ok(state)
                })?,
            None => State {
                namespace: namespace.into(),
                ..State::default()
            },
        };
        let reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::Manual)
            .try_into()
            .map_err(error)?;
        // The manual reader pins this committed generation. Slow source membership
        // checks cannot block the publisher or change pagination under the query.
        owner.unlock().map_err(error)?;
        Ok(Self {
            index,
            reader,
            fields,
            state,
            _owner: owner,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn admitted_reader_releases_the_publisher_lock() {
        let root = tempfile::tempdir().unwrap();
        drop(Engine::open(root.path(), "namespace").unwrap());
        let reader = Engine::read(root.path(), "namespace").unwrap();
        let publisher = Engine::open(root.path(), "namespace").unwrap();
        assert_eq!(reader.status().generation, publisher.status().generation);
        let mut writer = publisher
            .index
            .writer_with_num_threads::<tantivy::schema::TantivyDocument>(1, 16 * 1024 * 1024)
            .unwrap();
        writer.add_document(tantivy::doc!()).unwrap();
        writer.commit().unwrap();
        publisher.reader.reload().unwrap();
        assert_eq!(publisher.reader.searcher().num_docs(), 1);
        assert_eq!(reader.reader.searcher().num_docs(), 0);
    }
}
