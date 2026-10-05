use super::super::records::destination::{create, optional, same};
use super::*;
use crate::local_browser::working_tree::current::physical;
use rustix::fs;
use std::{collections::BTreeMap, fs::File};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) struct Snapshot {
    pub kind: Option<physical::Bytes>,
    pub files: BTreeMap<(String, String), physical::Bytes>,
}
pub(super) struct Destination {
    path: PathBuf,
    root: File,
    pub index: Option<File>,
    pub directories: BTreeMap<String, Option<File>>,
}
pub(super) fn names(dir: &File, budget: &Budget) -> Result<Vec<String>> {
    let mut names = vec![];
    for e in fs::Dir::read_from(dir).map_err(|_| Error::Unavailable)? {
        budget.work(1)?;
        let e = e.map_err(|_| Error::Unavailable)?;
        let name = e.file_name().to_str().map_err(|_| Error::Invalid)?;
        if matches!(name, "." | "..") {
            continue;
        }
        if names.len() >= 4352 {
            return Err(Error::Limit);
        }
        names.push(name.to_owned());
    }
    names.sort();
    Ok(names)
}
impl Destination {
    pub fn open(path: &std::path::Path) -> Result<Self> {
        let root = physical::directory(path)?;
        let index = optional(&root, "index")?;
        let mut directories = BTreeMap::new();
        for name in ["segments", "op_links", "changed_paths"] {
            directories.insert(
                name.into(),
                index
                    .as_ref()
                    .map(|dir| optional(dir, name))
                    .transpose()?
                    .flatten(),
            );
        }
        Ok(Self {
            path: path.to_owned(),
            root,
            index,
            directories,
        })
    }
    pub fn check(&self) -> Result<()> {
        same(&self.root, &physical::directory(&self.path)?)?;
        match (&self.index, optional(&self.root, "index")?) {
            (Some(saved), Some(current)) => {
                same(saved, &current)?;
                for (name, held) in &self.directories {
                    match (held, optional(saved, name)?) {
                        (Some(saved), Some(current)) => same(saved, &current)?,
                        (None, None) => (),
                        _ => return Err(Error::EpochChanged),
                    }
                }
            }
            (None, None) => (),
            _ => return Err(Error::EpochChanged),
        }
        Ok(())
    }
    pub fn ensure(&mut self) -> Result<()> {
        self.check()?;
        if self.index.is_none() {
            self.index = Some(create(&self.root, "index")?);
        }
        for (name, held) in &mut self.directories {
            if held.is_none() {
                *held = Some(create(self.index.as_ref().unwrap(), name)?);
            }
        }
        self.check()
    }
    pub fn directory(&self, name: &str) -> Result<&File> {
        self.directories
            .get(name)
            .and_then(Option::as_ref)
            .ok_or(Error::Unavailable)
    }
    pub fn capture(&self, budget: &Budget) -> Result<Snapshot> {
        self.check()?;
        let mut out = Snapshot::default();
        let Some(index) = &self.index else {
            return Ok(out);
        };
        for name in names(index, budget)? {
            if self.directories.contains_key(&name) {
                continue;
            }
            if name == "type" {
                out.kind = physical::read(index, &name, 64, false, budget)?;
                if out.kind.as_ref().is_none_or(|v| v.bytes != b"default") {
                    return Err(Error::Invalid);
                }
            } else if name.starts_with(".lapis-stack-recovery-") {
                physical::read(index, &name, MAX_SEGMENT, false, budget)?
                    .ok_or(Error::Unavailable)?;
            } else {
                return Err(Error::Invalid);
            }
        }
        let mut total = 0;
        for (directory, held) in &self.directories {
            let Some(held) = held else {
                continue;
            };
            let names = names(held, budget)?;
            let limit = if directory == "op_links" { 4096 } else { 128 };
            let mut count = 0;
            for name in names {
                if name.starts_with(".lapis-stack-recovery-") {
                    physical::read(held, &name, MAX_SEGMENT, false, budget)?
                        .ok_or(Error::Unavailable)?;
                    continue;
                }
                if name.len() != 128
                    || !name
                        .bytes()
                        .all(|v| v.is_ascii_digit() || (b'a'..=b'f').contains(&v))
                {
                    return Err(Error::Invalid);
                }
                count += 1;
                if count > limit {
                    return Err(Error::Limit);
                }
                let max = if directory == "op_links" {
                    65536
                } else {
                    MAX_SEGMENT
                };
                let value =
                    physical::read(held, &name, max, false, budget)?.ok_or(Error::Unavailable)?;
                total += value.bytes.len();
                if total > MAX_BYTES {
                    return Err(Error::Limit);
                }
                out.files.insert((directory.clone(), name), value);
            }
        }
        self.check()?;
        Ok(out)
    }
}
impl Snapshot {
    pub fn materialize(&self, path: &std::path::Path) -> Result<()> {
        for name in ["segments", "op_links", "changed_paths"] {
            std::fs::create_dir_all(path.join(name)).map_err(|_| Error::Unavailable)?;
        }
        for ((directory, name), value) in &self.files {
            std::fs::write(path.join(directory).join(name), &value.bytes)
                .map_err(|_| Error::Unavailable)?;
        }
        Ok(())
    }
}
