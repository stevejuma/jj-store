use super::*;
use std::os::unix::fs::PermissionsExt as _;

#[test]
fn preselection_authority_or_source_loss_withholds_catalog_selection() {
    for loss in [
        "authority",
        "append",
        "rewrite",
        "replacement",
        "directory",
        "lock",
    ] {
        let fixture = Fixture::new(4);
        let (probe, binding) = fixture.prepared();
        let before = fixture.catalog();
        let mut checks = 0;
        let result = fixture.registry.acquire_import(&probe, binding, || {
            checks += 1;
            if checks == 2 {
                match loss {
                    "authority" => return Err("authority revoked".into()),
                    "append" => append(&fixture.path),
                    "rewrite" => {
                        let original = fs::read_to_string(&fixture.path).unwrap();
                        fs::write(
                            &fixture.path,
                            original.replace("/private/import", "/private/change"),
                        )
                        .unwrap();
                    }
                    "replacement" => {
                        let bytes = fs::read(&fixture.path).unwrap();
                        fs::rename(&fixture.path, fixture.transcripts.join("retained")).unwrap();
                        fs::write(&fixture.path, bytes).unwrap();
                    }
                    "directory" => {
                        fs::rename(&fixture.state, fixture._root.path().join("retained-state"))
                            .unwrap();
                        fs::create_dir(&fixture.state).unwrap();
                        fs::set_permissions(&fixture.state, fs::Permissions::from_mode(0o700))
                            .unwrap();
                    }
                    "lock" => {
                        fs::rename(
                            fixture.state.join("registry.lock"),
                            fixture._root.path().join("retained-lock"),
                        )
                        .unwrap();
                        fs::write(fixture.state.join("registry.lock"), []).unwrap();
                        fs::set_permissions(
                            fixture.state.join("registry.lock"),
                            fs::Permissions::from_mode(0o600),
                        )
                        .unwrap();
                    }
                    _ => unreachable!(),
                }
            }
            Ok(())
        });
        assert!(result.is_err(), "{loss} must withhold selection");
        let selected_path = if loss == "directory" {
            fixture._root.path().join("retained-state/sources.json")
        } else {
            fixture.state.join("sources.json")
        };
        assert_eq!(
            fs::read(selected_path).unwrap(),
            before,
            "{loss} changed catalog"
        );
    }
}

#[test]
fn postselection_loss_keeps_marker_and_exact_retry_does_not_duplicate_identity() {
    let fixture = Fixture::new(4);
    let (probe, binding) = fixture.prepared();
    let mut checks = 0;
    assert!(
        fixture
            .registry
            .acquire_import(&probe, binding, || {
                checks += 1;
                if checks == 3 {
                    return Err("unknown after durable selection".into());
                }
                Ok(())
            })
            .is_err()
    );
    assert_eq!(checks, 3);
    let selected = fixture
        .registry
        .lookup_import_admitted(&probe, binding, || Ok(()))
        .unwrap()
        .unwrap();
    let before = fixture.catalog();
    assert_eq!(
        fixture
            .registry
            .acquire_import(&probe, binding, || Ok(()))
            .unwrap()
            .source_instance_id(),
        selected
    );
    assert_eq!(fixture.catalog(), before);
}

#[test]
fn retained_registry_gate_excludes_other_acquisition_through_selection() {
    let fixture = Fixture::new(4);
    let other = SourceRegistry::open(&fixture.state, [8; 16], 4).unwrap();
    let (probe, binding) = fixture.prepared();
    let mut checks = 0;
    let selected = fixture
        .registry
        .acquire_import(&probe, binding, || {
            checks += 1;
            assert!(other.acquire(fixture.qualified()).is_err());
            Ok(())
        })
        .unwrap();
    assert_eq!(checks, 3);
    assert!(other.acquire_import(&probe, binding, || Ok(())).is_err());
    drop(selected);
    assert!(other.acquire_import(&probe, binding, || Ok(())).is_ok());
}
