//! Validate the paired helper set before the desktop spawns any resource.
use crate::Result;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fs,
    io::Read,
    path::{Component, Path},
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Manifest {
    version: u32,
    binaries: Vec<Entry>,
    native_runtime: bool,
    #[serde(default)]
    native_files: Vec<Entry>,
    mesh_revision: String,
    local_agent_revision: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Entry {
    name: String,
    sha256: String,
}

fn check(root: &Path, entry: &Entry) -> Result<()> {
    let relative = Path::new(&entry.name);
    if entry.name.is_empty()
        || !relative
            .components()
            .all(|c| matches!(c, Component::Normal(_)))
    {
        return Err("Invalid bundled resource path".into());
    }
    let mut path = root.to_path_buf();
    for component in relative.components() {
        path.push(component);
        if fs::symlink_metadata(&path)
            .map_err(|_| "Missing bundled resource")?
            .file_type()
            .is_symlink()
        {
            return Err("Bundled resource cannot be a symlink".into());
        }
    }
    if !path.is_file() {
        return Err("Missing regular bundled resource".into());
    }
    let mut file = fs::File::open(&path).map_err(|_| "Unreadable bundled resource")?;
    let mut hash = Sha256::new();
    let mut buffer = [0u8; 65536];
    loop {
        let n = file
            .read(&mut buffer)
            .map_err(|_| "Unreadable bundled resource")?;
        if n == 0 {
            break;
        }
        hash.update(&buffer[..n]);
    }
    if format!("{:x}", hash.finalize()) != entry.sha256 {
        return Err("Bundled resource checksum mismatch".into());
    }
    Ok(())
}

pub(crate) fn verify(root: &Path) -> Result<()> {
    let path = root.join("agent-resources.json");
    let meta = fs::symlink_metadata(&path).map_err(|_| "Missing agent resource manifest")?;
    if !meta.is_file() || meta.file_type().is_symlink() || meta.len() > 262144 {
        return Err("Invalid agent resource manifest".into());
    }
    let manifest: Manifest =
        serde_json::from_slice(&fs::read(path).map_err(|_| "Unreadable agent resource manifest")?)
            .map_err(|_| "Invalid agent resource manifest")?;
    let required = [
        "jj-acp",
        "jj-store-local-agent",
        "jj-store-agent-tools",
        "jj-store-compute",
    ];
    let names: BTreeSet<_> = manifest.binaries.iter().map(|e| e.name.as_str()).collect();
    if manifest.version != 1
        || manifest.mesh_revision != "9f192c9d821991ba43d79bb15fd2f2f12e597571"
        || manifest.local_agent_revision != "ef2aa1ae38fadcc0bc22b8bf6ed96b35933146be"
        || manifest.binaries.len() != required.len()
        || names != required.into_iter().collect()
        || manifest.native_files.len() > 512
        || manifest.native_runtime == manifest.native_files.is_empty()
    {
        return Err("Unsupported agent resource set".into());
    }
    if root.join("native-runtime").exists() != manifest.native_runtime {
        return Err("Native runtime manifest mismatch".into());
    }
    for entry in &manifest.binaries {
        check(root, entry)?;
    }
    let mut native_names = BTreeSet::new();
    for entry in &manifest.native_files {
        if !entry.name.starts_with("native-runtime/") || !native_names.insert(&entry.name) {
            return Err("Invalid native runtime resource".into());
        }
        check(root, entry)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn paired_resource_manifest_refuses_tampering_and_missing_helpers() {
        let root = tempfile::tempdir().unwrap();
        let names = [
            "jj-acp",
            "jj-store-local-agent",
            "jj-store-agent-tools",
            "jj-store-compute",
        ];
        let binaries: Vec<_> = names.iter().map(|name| {
            fs::write(root.path().join(name), name).unwrap();
            serde_json::json!({"name":name,"sha256":format!("{:x}",Sha256::digest(name.as_bytes()))})
        }).collect();
        let manifest = serde_json::json!({"version":1,"binaries":binaries,"nativeRuntime":false,
            "meshRevision":"9f192c9d821991ba43d79bb15fd2f2f12e597571","localAgentRevision":"ef2aa1ae38fadcc0bc22b8bf6ed96b35933146be"});
        fs::write(
            root.path().join("agent-resources.json"),
            manifest.to_string(),
        )
        .unwrap();
        verify(root.path()).unwrap();
        fs::write(root.path().join("jj-acp"), "other build").unwrap();
        assert!(verify(root.path()).unwrap_err().contains("checksum"));
        fs::write(root.path().join("jj-acp"), "jj-acp").unwrap();
        fs::remove_file(root.path().join("jj-store-agent-tools")).unwrap();
        assert!(verify(root.path()).is_err());
    }
}
