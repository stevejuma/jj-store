use super::super::records::destination::{create, optional, same};
use super::*;
use crate::local_browser::working_tree::current::physical;
use rustix::fs;
use std::fs::File;
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Snapshot {
    pub segments: std::collections::BTreeMap<String, physical::Bytes>,
    pub heads: std::collections::BTreeMap<String, physical::Bytes>,
}
pub(super) struct Destination {
    path: PathBuf,
    store: File,
    pub extra: Option<File>,
    pub heads: Option<File>,
}
fn names(dir: &File, budget: &Budget) -> Result<Vec<String>> {
    let mut names = vec![];
    for entry in fs::Dir::read_from(dir).map_err(|_| Error::Unavailable)? {
        budget.work(1)?;
        let entry = entry.map_err(|_| Error::Unavailable)?;
        let name = entry.file_name().to_str().map_err(|_| Error::Invalid)?;
        if matches!(name, "." | "..") {
            continue;
        }
        if names.len() >= 256 {
            return Err(Error::Limit);
        }
        names.push(name.to_owned());
    }
    names.sort();
    Ok(names)
}
fn segment(name: &str) -> Result<()> {
    if name.len() != 128
        || !name
            .bytes()
            .all(|v| v.is_ascii_digit() || (b'a'..=b'f').contains(&v))
    {
        return Err(Error::Invalid);
    }
    Ok(())
}
impl Destination {
    pub fn open(metadata: &std::path::Path) -> Result<Self> {
        let path = metadata.join("store");
        let store = physical::directory(&path)?;
        let extra = optional(&store, "extra")?;
        let heads = extra
            .as_ref()
            .map(|dir| optional(dir, "heads"))
            .transpose()?
            .flatten();
        Ok(Self {
            path,
            store,
            extra,
            heads,
        })
    }
    pub fn check(&self) -> Result<()> {
        same(&self.store, &physical::directory(&self.path)?)?;
        match (&self.extra, optional(&self.store, "extra")?) {
            (Some(saved), Some(current)) => {
                same(saved, &current)?;
                match (&self.heads, optional(saved, "heads")?) {
                    (Some(saved), Some(current)) => same(saved, &current)?,
                    (None, None) => (),
                    _ => return Err(Error::EpochChanged),
                }
            }
            (None, None) => (),
            _ => return Err(Error::EpochChanged),
        }
        Ok(())
    }
    pub fn ensure(&mut self) -> Result<()> {
        self.check()?;
        if self.extra.is_none() {
            self.extra = Some(create(&self.store, "extra")?);
        }
        if self.heads.is_none() {
            self.heads = Some(create(self.extra.as_ref().unwrap(), "heads")?);
        }
        self.check()
    }
    pub fn capture(&self, budget: &Budget) -> Result<Snapshot> {
        self.check()?;
        let mut value = Snapshot {
            segments: Default::default(),
            heads: Default::default(),
        };
        let Some(extra) = &self.extra else {
            return Ok(value);
        };
        let mut total = 0;
        for name in names(extra, budget)? {
            if name == "heads" {
                continue;
            }
            if name == "lock" || name.starts_with(".lapis-stack-recovery-") {
                physical::read(extra, &name, MAX_SEGMENT, false, budget)?
                    .ok_or(Error::Unavailable)?;
                continue;
            }
            segment(&name)?;
            if value.segments.len() >= 128 {
                return Err(Error::Limit);
            }
            let bytes = physical::read(extra, &name, MAX_SEGMENT, false, budget)?
                .ok_or(Error::Unavailable)?;
            total += bytes.bytes.len();
            if total > MAX_EXTRA_BYTES {
                return Err(Error::Limit);
            }
            value.segments.insert(name, bytes);
        }
        if let Some(heads) = &self.heads {
            for name in names(heads, budget)? {
                segment(&name)?;
                if value.heads.len() >= 64 {
                    return Err(Error::Limit);
                }
                let bytes =
                    physical::read(heads, &name, 0, false, budget)?.ok_or(Error::Unavailable)?;
                if !value.segments.contains_key(&name) {
                    return Err(Error::Incomplete);
                }
                value.heads.insert(name, bytes);
            }
        }
        self.check()?;
        Ok(value)
    }
}
impl Snapshot {
    pub fn materialize(&self, root: &std::path::Path) -> Result<()> {
        let extra = root.join("store/extra");
        std::fs::create_dir_all(extra.join("heads")).map_err(|_| Error::Unavailable)?;
        for (name, bytes) in &self.segments {
            std::fs::write(extra.join(name), &bytes.bytes).map_err(|_| Error::Unavailable)?;
        }
        for name in self.heads.keys() {
            std::fs::write(extra.join("heads").join(name), []).map_err(|_| Error::Unavailable)?;
        }
        Ok(())
    }
}
