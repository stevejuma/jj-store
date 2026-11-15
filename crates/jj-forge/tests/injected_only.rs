//! Injected-custody builds must refuse accidental native-store calls.
#![cfg(not(feature = "native-secrets"))]
use jj_forge::{
    Error,
    accounts::store::{NativeSecrets, ScopedNativeSecrets, SecretStore},
};

#[test]
fn native_custody_is_unavailable_without_the_platform_backend() {
    for store in [
        &NativeSecrets as &dyn SecretStore,
        &ScopedNativeSecrets::new("md.lapis.fixture.unavailable") as &dyn SecretStore,
    ] {
        assert!(matches!(store.get("fixture"), Err(Error::Unsupported)));
        assert!(matches!(
            store.set("fixture", "synthetic"),
            Err(Error::Unsupported)
        ));
        assert!(matches!(store.remove("fixture"), Err(Error::Unsupported)));
    }
}
