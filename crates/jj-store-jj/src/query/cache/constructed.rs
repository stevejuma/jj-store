//! Verify private owned output before admitting its allocating JJ decoder.
use super::*;

pub(in crate::query) fn verify(
    path: &Path,
    operation: m::OperationId,
    policy: Policy,
    budget: &Budget,
) -> Result<()> {
    verify_files(path, operation, policy, budget, false)
}
pub(in crate::query) fn verify_primary(
    path: &Path,
    operation: m::OperationId,
    policy: Policy,
    budget: &Budget,
) -> Result<()> {
    verify_files(path, operation, policy, budget, true)
}
fn verify_files(
    path: &Path,
    operation: m::OperationId,
    policy: Policy,
    budget: &Budget,
    primary: bool,
) -> Result<()> {
    let (mut files, _) = inventory(path, policy, budget, true)?;
    if primary {
        let expected = b"default";
        if files.remove("type").is_none_or(|file| {
            file.bytes != expected.len() as u64
                || file.digest != format!("{:x}", Blake2b512::digest(expected))
        }) {
            return Err(Error::Unavailable);
        }
    }
    let filter = super::super::prepared::filters::NAME;
    let checksum = format!("{filter}.sha256");
    match (files.remove(filter), files.remove(&checksum)) {
        (None, None) => {}
        (Some(data), Some(sum)) if data.bytes > 0 && sum.bytes == 32 => {}
        _ => return Err(Error::Unavailable),
    }
    if files.len() != 2 {
        return Err(Error::Unavailable);
    }
    let link = format!("op_links/{operation}");
    if files.get(&link).is_none_or(|file| file.bytes != 66) {
        return Err(Error::Unavailable);
    }
    let mut data = [0; 66];
    let mut file = fs::File::open(path.join(link)).map_err(|_| Error::Unavailable)?;
    file.read_exact(&mut data).map_err(|_| Error::Unavailable)?;
    if file.read(&mut [0]).map_err(|_| Error::Unavailable)? != 0 || data[..2] != [0x0a, 64] {
        return Err(Error::Unavailable);
    }
    let name: String = data[2..].iter().map(|n| format!("{n:02x}")).collect();
    if files
        .get(&format!("segments/{name}"))
        .is_none_or(|file| file.digest != name)
    {
        return Err(Error::Unavailable);
    }
    budget.check()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn private_output_requires_exact_operation_link_and_segment_checksum() {
        let root = tempfile::tempdir().unwrap();
        let operation = m::OperationId::new([7; 64]);
        fs::create_dir(root.path().join("op_links")).unwrap();
        fs::create_dir(root.path().join("segments")).unwrap();
        let segment = b"owned format bytes";
        let digest = Blake2b512::digest(segment);
        let name = format!("{digest:x}");
        let segment_path = root.path().join("segments").join(&name);
        fs::write(&segment_path, segment).unwrap();
        let mut link = vec![0x0a, 64];
        link.extend_from_slice(&digest);
        let link_path = root.path().join("op_links").join(operation.to_string());
        fs::write(&link_path, &link).unwrap();
        let budget = Budget::new(Limits::default()).unwrap();
        let policy = Policy::default();
        verify(root.path(), operation, policy, &budget).unwrap();

        assert!(verify(root.path(), m::OperationId::new([8; 64]), policy, &budget).is_err());
        fs::write(&segment_path, b"corrupt checksum").unwrap();
        assert!(verify(root.path(), operation, policy, &budget).is_err());
        fs::write(&segment_path, segment).unwrap();
        link[0] = 0x12;
        fs::write(&link_path, &link).unwrap();
        assert!(verify(root.path(), operation, policy, &budget).is_err());
        link[0] = 0x0a;
        fs::write(&link_path, &link).unwrap();
        fs::write(root.path().join("unexpected"), b"extra output").unwrap();
        assert!(verify(root.path(), operation, policy, &budget).is_err());
        fs::remove_file(root.path().join("unexpected")).unwrap();
        fs::write(root.path().join("type"), b"default").unwrap();
        assert!(verify(root.path(), operation, policy, &budget).is_err());
        verify_primary(root.path(), operation, policy, &budget).unwrap();
        fs::write(root.path().join("type"), b"invalid").unwrap();
        assert!(verify_primary(root.path(), operation, policy, &budget).is_err());
    }
}
