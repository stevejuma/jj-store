use super::*;
use jj_store_jj::local_browser::Error;
use jj_store_model as m;
#[test]
fn noncanonical_tampered_incomplete_and_conflicting_inventory_cannot_decode() {
    let root = fixture();
    let inventory = scan(root.path()).inventory().unwrap();
    let id = inventory.id().unwrap();
    let bytes = inventory.encode().unwrap();
    let mut spaced = bytes.clone();
    spaced.push(b' ');
    assert_eq!(
        PhysicalInventoryV1::decode(id, &spaced),
        Err(Error::Invalid)
    );
    assert_eq!(
        PhysicalInventoryV1::decode(m::RecordDigest::new([1; 32]), &bytes),
        Err(Error::Invalid)
    );
    for case in [
        "generation",
        "workspace",
        "fingerprint",
        "file_fp",
        "parent",
        "root",
        "unsorted",
        "duplicate",
        "collision",
        "unsafe",
        "mode",
        "unknown",
    ] {
        let mut value = serde_json::to_value(&inventory).unwrap();
        match case {
            "generation" => value["generation"] = 8.into(),
            "workspace" | "fingerprint" => {
                value[case] = serde_json::to_value(m::RecordDigest::new([3; 32])).unwrap()
            }
            "file_fp" => {
                value["files"][0]["fingerprint"] =
                    serde_json::to_value(m::RecordDigest::new([4; 32])).unwrap()
            }
            "parent" => {
                value["directories"].as_array_mut().unwrap().remove(1);
            }
            "root" => {
                value["directories"].as_array_mut().unwrap().remove(0);
            }
            "unsorted" => value["directories"].as_array_mut().unwrap().swap(0, 1),
            "duplicate" => {
                let row = value["files"][0].clone();
                value["files"].as_array_mut().unwrap().push(row);
            }
            "collision" => value["files"][0]["path"] = "src/empty".into(),
            "unsafe" => value["files"][0]["path"] = ".jj/repo".into(),
            "mode" => value["directories"][0]["stamp"][3] = 0o100600u64.into(),
            "unknown" => value["unexpected"] = true.into(),
            _ => unreachable!(),
        }
        assert!(
            PhysicalInventoryV1::decode(id, &serde_json::to_vec(&value).unwrap()).is_err(),
            "{case}"
        );
    }
}
#[test]
fn physical_rows_and_scope_must_match_the_exact_original_digest() {
    let root = fixture();
    let original = scan(root.path());
    let inventory = original.inventory().unwrap();
    let binding = jj_store_jj::local_execution::RecoveryBinding {
        repository: original.repository,
        generation: original.generation,
        physical_workspace: original.workspace.0,
        operation: m::OperationId::new([8; 64]),
        view: m::ViewId::new([9; 64]),
        commit: m::GitSha1Id::new([10; 20]),
        workspace: "default".into(),
    };
    inventory
        .check_selection(&binding, original.fingerprint)
        .unwrap();
    let mut replaced = binding.clone();
    replaced.generation += 1;
    assert_eq!(
        inventory.check_selection(&replaced, original.fingerprint),
        Err(Error::EpochChanged)
    );
    replaced = binding.clone();
    replaced.repository = m::RepositoryId::new([3; 16]);
    assert_eq!(
        inventory.check_selection(&replaced, original.fingerprint),
        Err(Error::EpochChanged)
    );
    replaced = binding.clone();
    replaced.physical_workspace = m::RecordDigest::new([3; 32]);
    assert_eq!(
        inventory.check_selection(&replaced, original.fingerprint),
        Err(Error::EpochChanged)
    );
    assert_eq!(
        inventory.check_selection(&binding, m::RecordDigest::new([4; 32])),
        Err(Error::EpochChanged)
    );
    std::fs::rename(root.path().join("src/empty"), root.path().join("src/old")).unwrap();
    std::fs::create_dir(root.path().join("src/empty")).unwrap();
    assert_ne!(inventory, scan(root.path()).inventory().unwrap());
    let mut forged = scan(root.path());
    forged.fingerprint = original.fingerprint;
    assert_eq!(forged.inventory(), Err(tree::Error::Stale));
}
#[test]
fn record_byte_path_and_namespace_bounds_fail_without_a_partial_inventory() {
    let root = fixture();
    let inventory = scan(root.path()).inventory().unwrap();
    let dir = inventory.directories().next().unwrap().clone();
    let files = (0..4097)
        .map(|n| PhysicalFileV1 {
            path: format!("f-{n:04}"),
            fingerprint: jj_store_model::RecordDigest::new([1; 32]),
        })
        .collect();
    assert_eq!(
        PhysicalInventoryV1::new(
            RepositoryId::new([1; 16]),
            7,
            inventory_scope(&inventory),
            vec![dir.clone()],
            files
        ),
        Err(Error::Limit)
    );
    let bytes = vec![b' '; 8 * 1024 * 1024 + 1];
    assert_eq!(
        PhysicalInventoryV1::decode(inventory.id().unwrap(), &bytes),
        Err(Error::Limit)
    );
    let mut directories = vec![dir];
    let mut prefix = String::new();
    // Synthetic pressure records isolate byte bounds; no filesystem proof.
    for n in 0..32 {
        if n > 0 {
            prefix.push('/');
        }
        prefix.push_str(&"a".repeat(60));
        directories.push(PhysicalDirectoryV1 {
            path: prefix.clone(),
            stamp: [1, 2, 0, 0o040700, 0, 0, 0, 0],
        });
    }
    let files = (0..4000)
        .map(|n| PhysicalFileV1 {
            path: format!("{prefix}/{}-{n:04}", "z".repeat(90)),
            fingerprint: jj_store_model::RecordDigest::new([1; 32]),
        })
        .collect();
    assert_eq!(
        PhysicalInventoryV1::new(
            RepositoryId::new([1; 16]),
            7,
            inventory_scope(&inventory),
            directories,
            files
        ),
        Err(Error::Limit)
    );
}
fn inventory_scope(inventory: &PhysicalInventoryV1) -> jj_store_model::RecordDigest {
    serde_json::from_value(serde_json::to_value(inventory).unwrap()["workspace"].clone()).unwrap()
}
